use openvm_stark_backend::p3_field::{Field, FieldAlgebra, PrimeField32};
use openvm_circuit::arch::{ExecutionBridge, ExecutionBus, ExecutionError, ExecutionState, InstructionExecutor};
use openvm_circuit::system::memory::offline_checker::MemoryBridge;
use openvm_circuit::system::memory::{MemoryController, OfflineMemory, RecordId};
use openvm_circuit::system::program::ProgramBus;
use openvm_instructions::instruction::Instruction;
use std::sync::{Arc, Mutex};
use openvm_instructions::program::DEFAULT_PC_STEP;
use openvm_native_compiler::FriOpcode::FRI_REDUCED_OPENING;
use openvm_poseidon2_air::{Poseidon2Config, Poseidon2SubAir};
use crate::{EXT_DEG, FieldExtension};
use crate::verify_batch::air::{VerifyBatchAir, VerifyBatchBus};
use crate::verify_batch::CHUNK;

pub struct VerifyBatchRecord<F: Field> {
    initial_state: ExecutionState<u32>,
    instruction: Instruction<F>,
    dim_base_pointer: usize,
    opened_base_pointer: usize,
    opened_length: usize,
    sibling_base_pointer: usize,
    index_base_pointer: usize,
    commit_pointer: usize,
    dim_read: RecordId,
    opened_read: RecordId,
    sibling_read: RecordId,
    index_read: RecordId,
    commit_read: RecordId,
    top_level: Vec<TopLevelRecord<F>>,
}

struct TopLevelRecord<F: Field> {
    incorporate_row: Option<IncorporateRowRecord<F>>,
    incorporate_sibling_record: IncorporateSiblingRecord<F>,
}

struct IncorporateSiblingRecord<F: Field> {
    read_sibling_array_start: RecordId,
    read_root_is_on_right: RecordId,
    sibling: [F; CHUNK],
    reads: [RecordId; CHUNK],
}

struct IncorporateRowRecord<F: Field> {
    chunks: Vec<InsideRowRecord<F>>,
    initial_height_read: RecordId,
    final_height_read: RecordId,
}

struct InsideRowRecord<F: Field> {
    cells: Vec<CellRecord>,
    chunk: [F; CHUNK],
}

struct CellRecord {
    read: RecordId,
    opened_index: usize,
    read_row_pointer_and_length: Option<RecordId>,
    row_pointer: usize,
    row_end: usize,
}

pub struct VerifyBatchChip<F: Field, const SBOX_REGISTERS: usize> {
    air: VerifyBatchAir<F, SBOX_REGISTERS>,
    records: Vec<VerifyBatchRecord<F>>,
    height: usize,
    offline_memory: Arc<Mutex<OfflineMemory<F>>>,
}

impl<F: PrimeField32, const SBOX_REGISTERS: usize> VerifyBatchChip<F, SBOX_REGISTERS> {
    pub fn new(
        execution_bus: ExecutionBus,
        program_bus: ProgramBus,
        memory_bridge: MemoryBridge,
        offset: usize,
        offline_memory: Arc<Mutex<OfflineMemory<F>>>,
        poseidon2_config: Poseidon2Config<F>,
    ) -> Self {
        let air = VerifyBatchAir {
            execution_bridge: ExecutionBridge::new(execution_bus, program_bus),
            memory_bridge,
            internal_bus: VerifyBatchBus(7),
            subair: Arc::new(Poseidon2SubAir::new(
                poseidon2_config.constants.into(),
            )),
            offset,
        };
        Self {
            records: vec![],
            air,
            height: 0,
            offline_memory,
        }
    }
}

impl<F: PrimeField32, const SBOX_REGISTERS: usize> InstructionExecutor<F> for VerifyBatchChip<F, SBOX_REGISTERS> {
    fn execute(
        &mut self,
        memory: &mut MemoryController<F>,
        instruction: &Instruction<F>,
        from_state: ExecutionState<u32>,
    ) -> Result<ExecutionState<u32>, ExecutionError> {
        let &Instruction {
            a: dim_register,
            b: opened_register,
            c: sibling_register,
            d: index_register,
            e: commit_register,
            f: address_space,
            ..
        } = instruction;

        let alpha_read = memory.read(addr_space, alpha_ptr);
        let length_read = memory.read_cell(addr_space, length_ptr);
        let a_ptr_read = memory.read_cell(addr_space, a_ptr_ptr);
        let b_ptr_read = memory.read_cell(addr_space, b_ptr_ptr);

        let alpha = alpha_read.1;
        let alpha_pow_original = from_fn(|i| {
            memory.unsafe_read_cell(addr_space, alpha_pow_ptr + F::from_canonical_usize(i))
        });
        let mut alpha_pow = alpha_pow_original;
        let length = length_read.1.as_canonical_u32() as usize;
        let a_ptr = a_ptr_read.1;
        let b_ptr = b_ptr_read.1;

        let mut a_reads = Vec::with_capacity(length);
        let mut b_reads = Vec::with_capacity(length);
        let mut result = [F::ZERO; EXT_DEG];

        for i in 0..length {
            let a_read = memory.read_cell(addr_space, a_ptr + F::from_canonical_usize(i));
            let b_read = memory.read(addr_space, b_ptr + F::from_canonical_usize(4 * i));
            a_reads.push(a_read);
            b_reads.push(b_read);
            let a = a_read.1;
            let b = b_read.1;
            result = FieldExtension::add(
                result,
                FieldExtension::multiply(FieldExtension::subtract(b, elem_to_ext(a)), alpha_pow),
            );
            alpha_pow = FieldExtension::multiply(alpha, alpha_pow);
        }

        let (alpha_pow_write, prev_data) = memory.write(addr_space, alpha_pow_ptr, alpha_pow);
        debug_assert_eq!(prev_data, alpha_pow_original);
        let (result_write, _) = memory.write(addr_space, result_ptr, result);

        self.records.push(VerifyBatchRecord {
            pc: F::from_canonical_u32(from_state.pc),
            start_timestamp: F::from_canonical_u32(from_state.timestamp),
            instruction: instruction.clone(),
            alpha_read: alpha_read.0,
            length_read: length_read.0,
            a_ptr_read: a_ptr_read.0,
            b_ptr_read: b_ptr_read.0,
            a_reads: a_reads.into_iter().map(|r| r.0).collect(),
            b_reads: b_reads.into_iter().map(|r| r.0).collect(),
            alpha_pow_original,
            alpha_pow_write,
            result_write,
        });

        self.height += length;

        Ok(ExecutionState {
            pc: from_state.pc + DEFAULT_PC_STEP,
            timestamp: memory.timestamp(),
        })
    }

    fn get_opcode_name(&self, opcode: usize) -> String {
        assert_eq!(opcode, (FRI_REDUCED_OPENING as usize) + self.air.offset);
        String::from("FRI_REDUCED_OPENING")
    }
}