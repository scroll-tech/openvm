//! Stateful keccak256 hasher. Handles full keccak sponge (padding, absorb, keccak-f) on
//! variable length inputs read from VM memory.
use std::{
    array,
    cmp::{max, min},
    sync::Arc,
};

use ax_circuit_primitives::{bitwise_op_lookup::BitwiseOperationLookupChip, encoder::Encoder};
use ax_hashes::sha256::{Sha256Air, SHA256_BLOCK_BITS};
use p3_field::PrimeField32;

mod air;
mod columns;
mod trace;

pub use air::*;
pub use columns::*;

#[cfg(test)]
mod tests;

use axvm_instructions::{
    instruction::Instruction,
    program::DEFAULT_PC_STEP,
    riscv::{RV32_CELL_BITS, RV32_REGISTER_NUM_LIMBS}, Rv32Sha256Opcode, UsizeOpcode
};
use sha2::{Digest, Sha256};

use crate::{
    arch::{ExecutionBridge, ExecutionBus, ExecutionState, InstructionExecutor},
    rv32im::adapters::read_rv32_register,
    system::{
        memory::{MemoryControllerRef, MemoryReadRecord, MemoryWriteRecord},
        program::{ExecutionError, ProgramBus},
    },
};

// ==== Constants for register/memory adapter ====
/// Register reads to get dst, src, len
const SHA256_REGISTER_READS: usize = 3;
/// Number of cells to read in a single memory access
const SHA256_READ_SIZE: usize = 16;
/// Number of cells to write in a single memory access
const SHA256_WRITE_SIZE: usize = 32;
/// Number of rv32 cells read in a SHA256 block
pub const SHA256_BLOCK_CELLS: usize = SHA256_BLOCK_BITS / RV32_CELL_BITS;
/// Bus index fo the chip to send interactions to itself
pub const SHA256_CHIP_BUS_IDX: usize = 28;

#[derive(Debug)]
pub struct Sha256VmChip<F: PrimeField32> {
    pub air: Sha256VmAir,
    /// IO and memory data necessary for each opcode call
    pub records: Vec<Sha256Record<F>>,
    pub memory_controller: MemoryControllerRef<F>,
    pub bitwise_lookup_chip: Arc<BitwiseOperationLookupChip<8>>,

    offset: usize,
}

#[derive(Clone, Debug)]
pub struct Sha256Record<F> {
    pub from_state: ExecutionState<F>,
    pub dst_read: MemoryReadRecord<F, RV32_REGISTER_NUM_LIMBS>,
    pub src_read: MemoryReadRecord<F, RV32_REGISTER_NUM_LIMBS>,
    pub len_read: MemoryReadRecord<F, RV32_REGISTER_NUM_LIMBS>,
    pub input_message:
        Vec<[MemoryReadRecord<F, SHA256_READ_SIZE>; SHA256_BLOCK_CELLS / SHA256_READ_SIZE]>,
    pub digest_write: MemoryWriteRecord<F, SHA256_WRITE_SIZE>,
}

impl<F: PrimeField32> Sha256VmChip<F> {
    pub fn new(
        execution_bus: ExecutionBus,
        program_bus: ProgramBus,
        memory_controller: MemoryControllerRef<F>,
        bitwise_lookup_chip: Arc<BitwiseOperationLookupChip<8>>,
        offset: usize,
    ) -> Self {
        let ptr_max_bits = memory_controller.borrow().mem_config.pointer_max_bits;
        let memory_bridge = memory_controller.borrow().memory_bridge();
        Self {
            air: Sha256VmAir::new(
                ExecutionBridge::new(execution_bus, program_bus),
                memory_bridge,
                bitwise_lookup_chip.bus(),
                ptr_max_bits,
                offset,
                Sha256Air::new(bitwise_lookup_chip.bus(), SHA256_CHIP_BUS_IDX),
                Encoder::new(17, 2),
            ),
            memory_controller,
            bitwise_lookup_chip,
            records: Vec::new(),
            offset,
        }
    }
}

impl<F: PrimeField32> InstructionExecutor<F> for Sha256VmChip<F> {
    fn execute(
        &mut self,
        instruction: Instruction<F>,
        from_state: ExecutionState<u32>,
    ) -> Result<ExecutionState<u32>, ExecutionError> {
        let Instruction {
            opcode,
            a,
            b,
            c,
            d,
            e,
            ..
        } = instruction;
        let local_opcode = Rv32Sha256Opcode::from_usize(opcode - self.offset);
        debug_assert_eq!(local_opcode, Rv32Sha256Opcode::SHA256);
        debug_assert_eq!(d, F::from_canonical_u32(1));
        debug_assert_eq!(e, F::from_canonical_u32(2));

        let mut memory = self.memory_controller.borrow_mut();
        debug_assert_eq!(from_state.timestamp, memory.timestamp());

        let (dst_read, dst) = read_rv32_register(&mut memory, d, a);
        let (src_read, src) = read_rv32_register(&mut memory, d, b);
        let (len_read, len) = read_rv32_register(&mut memory, d, c);

        #[cfg(debug_assertions)]
        {
            assert!(dst < (1 << self.air.ptr_max_bits));
            assert!(src < (1 << self.air.ptr_max_bits));
            assert!(len < (1 << self.air.ptr_max_bits));
        }

        // need to pad with one 1 bit, 64 bits for the message length and then pad until the length is divisible by [SHA256_BLOCK_BITS]
        let num_blocks = ((len << 3) as usize + 1 + 64 + SHA256_BLOCK_BITS - 1) / SHA256_BLOCK_BITS;

        // we will read [num_blocks] * [SHA256_BLOCK_CELLS] cells but only [len] cells will be used
        debug_assert!(
            src as usize + num_blocks * SHA256_BLOCK_CELLS <= (1 << self.air.ptr_max_bits)
        );
        let mut hasher = Sha256::new();
        let mut input_message = Vec::new();
        let mut read_ptr = src;
        for _ in 0..num_blocks {
            let block_reads = array::from_fn(|_| {
                let read_record = memory.read(e, F::from_canonical_u32(read_ptr));
                // we add to the hasher only the bytes that are part of the message
                let num_reads = min(
                    SHA256_READ_SIZE,
                    (max(read_ptr, src + len) - read_ptr) as usize,
                );
                println!(
                    "num_reads: {}, read_ptr: {}, src: {}, len: {}, num_blocks: {}",
                    num_reads, read_ptr, src, len, num_blocks
                );
                hasher.update(&read_record.data.map(|x| x.as_canonical_u32() as u8)[0..num_reads]);
                read_ptr += SHA256_READ_SIZE as u32;
                read_record
            });
            input_message.push(block_reads);
        }

        for p in input_message.iter() {
            for p1 in p.iter() {
                println!("p1: {:?}", p1.data);
            }
        }

        let mut digest = [0u8; SHA256_WRITE_SIZE];
        digest.copy_from_slice(hasher.finalize().as_ref());
        let digest_write = memory.write(
            e,
            F::from_canonical_u32(dst),
            digest.map(|b| F::from_canonical_u8(b)),
        );

        self.records.push(Sha256Record {
            from_state: from_state.map(F::from_canonical_u32),
            dst_read,
            src_read,
            len_read,
            input_message,
            digest_write,
        });

        Ok(ExecutionState {
            pc: from_state.pc + DEFAULT_PC_STEP,
            timestamp: memory.timestamp(),
        })
    }

    fn get_opcode_name(&self, _: usize) -> String {
        "SHA256".to_string()
    }
}

impl<F: Copy> Sha256Record<F> {
    pub fn digest_addr_space(&self) -> F {
        self.digest_write.address_space
    }

    pub fn start_timestamp(&self) -> u32 {
        self.dst_read.timestamp
    }
}

pub fn sha256_solve(input_message: &[u8]) -> [u8; SHA256_WRITE_SIZE] {
    let mut hasher = Sha256::new();
    hasher.update(input_message);
    let mut output = [0u8; SHA256_WRITE_SIZE];
    output.copy_from_slice(hasher.finalize().as_ref());
    output
}
