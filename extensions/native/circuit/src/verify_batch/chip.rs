use std::sync::{Arc, Mutex};

use openvm_stark_backend::p3_field::{Field, FieldAlgebra, PrimeField32};

use openvm_circuit::{
    arch::{ExecutionBridge, ExecutionBus, ExecutionError, ExecutionState, InstructionExecutor},
    system::{
        memory::{MemoryController, offline_checker::MemoryBridge, OfflineMemory, RecordId},
        program::ProgramBus,
    },
};
use openvm_instructions::{instruction::Instruction, program::DEFAULT_PC_STEP};
use openvm_native_compiler::VerifyBatchOpcode::VERIFY_BATCH;
use openvm_poseidon2_air::{Poseidon2Config, Poseidon2SubAir, Poseidon2SubChip};

use crate::verify_batch::{
    air::{VerifyBatchAir, VerifyBatchBus},
    CHUNK,
};

pub struct VerifyBatchRecord<F: Field> {
    from_state: ExecutionState<u32>,
    instruction: Instruction<F>,
    dim_base_pointer: F,
    opened_base_pointer: F,
    opened_length: usize,
    sibling_base_pointer: F,
    index_base_pointer: F,
    commit_pointer: F,
    dim_base_pointer_read: RecordId,
    opened_base_pointer_and_length_read: RecordId,
    sibling_base_pointer_read: RecordId,
    index_base_pointer_read: RecordId,
    commit_pointer_read: RecordId,
    commit_read: RecordId,
    initial_height: usize,
    top_level: Vec<TopLevelRecord<F>>,
}

struct TopLevelRecord<F: Field> {
    // must be present in first record
    incorporate_row: Option<IncorporateRowRecord<F>>,
    // must be present in all bust last record
    incorporate_sibling: Option<IncorporateSiblingRecord<F>>,
}

struct IncorporateSiblingRecord<F: Field> {
    read_sibling_array_start: RecordId,
    read_root_is_on_right: RecordId,
    sibling: [F; CHUNK],
    reads: [RecordId; CHUNK],
}

struct IncorporateRowRecord<F: Field> {
    chunks: Vec<InsideRowRecord<F>>,
    initial_opened_index: usize,
    final_opened_index: usize,
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
    subchip: Poseidon2SubChip<F, SBOX_REGISTERS>
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
            subair: Arc::new(Poseidon2SubAir::new(poseidon2_config.constants.into())),
            offset,
        };
        Self {
            records: vec![],
            air,
            height: 0,
            offline_memory,
            subchip: Poseidon2SubChip::new(poseidon2_config.constants),
        }
    }
    
    fn compress(&self, left: [F; CHUNK], right: [F; CHUNK]) -> [F; CHUNK] {
        let concatenated = std::array::from_fn(|i| if i < CHUNK { left[i] } else { right[i - CHUNK] });
        let permuted = self.subchip.permute(concatenated);
        std::array::from_fn(|i| permuted[i])
    }
}

impl<F: PrimeField32, const SBOX_REGISTERS: usize> InstructionExecutor<F>
    for VerifyBatchChip<F, SBOX_REGISTERS>
{
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

        let (dim_base_pointer_read, dim_base_pointer) = memory.read_cell(address_space, dim_register);
        let (opened_base_pointer_and_length_read, [opened_base_pointer, opened_length]) =
            memory.read(address_space, opened_register);
        let (sibling_base_pointer_read, sibling_base_pointer) =
            memory.read_cell(address_space, sibling_register);
        let (index_base_pointer_read, index_base_pointer) = memory.read_cell(address_space, index_register);
        let (commit_pointer_read, commit_pointer) = memory.read_cell(address_space, commit_register);
        let (commit_read, commit) = memory.read(address_space, commit_pointer);

        let opened_length = opened_length.as_canonical_usize();

        let initial_height = memory.unsafe_read_cell(address_space, dim_base_pointer).as_canonical_u32();
        let mut height = initial_height;
        let mut proof_index = 0;
        let mut opened_index = 0;
        let mut top_level = vec![];
        
        let mut root = [F::ZERO; CHUNK];

        while height >= 1 {
            let incorporate_row = if opened_index < opened_length
                && memory.unsafe_read_cell(
                    address_space,
                    dim_base_pointer + F::from_canonical_usize(opened_index),
                ) == F::from_canonical_u32(height)
            {
                let initial_opened_index = opened_index;
                for _ in 0..6 {
                    memory.increment_timestamp();
                }
                let mut chunks = vec![];
                
                opened_index -= 1;
                let mut row_pointer = 0;
                let mut row_end = 0;
                
                let mut rolling_hash = [F::ZERO; 2 * CHUNK];
                
                while opened_index < opened_length
                    && memory.unsafe_read_cell(
                    address_space,
                    dim_base_pointer + F::from_canonical_usize(opened_index),
                ) == F::from_canonical_u32(height) {
                    let mut cells = vec![];
                    let mut chunk = [F::ZERO; CHUNK];
                    for i in 0..CHUNK {
                        let read_row_pointer_and_length = if row_pointer == row_end {
                            opened_index += 1;
                            if opened_index == opened_length || memory.unsafe_read_cell(
                                address_space,
                                dim_base_pointer + F::from_canonical_u32(opened_index),
                            ) != F::from_canonical_u32(height) {
                                break;
                            }
                            let (result, [new_row_pointer, row_len]) = memory.read(address_space, opened_base_pointer + F::from_canonical_u32(2 * opened_index));
                            row_pointer = new_row_pointer.as_canonical_u32() as usize;
                            row_end = row_pointer + row_len.as_canonical_u32() as usize;
                            Some(result)
                        } else {
                            memory.increment_timestamp();
                            None
                        };
                        let (read, value) = memory.read_cell(address_space, F::from_canonical_usize(row_pointer));
                        
                        cells.push(CellRecord {
                            read,
                            opened_index,
                            read_row_pointer_and_length,
                            row_pointer,
                            row_end,
                        });
                        chunk[i] = value;
                        row_pointer += 1;
                    }
                    chunks.push(InsideRowRecord {
                        cells,
                        chunk,
                    });
                    for i in 0..CHUNK {
                        rolling_hash[i] = chunk[i];
                    }
                    self.subchip.permute_mut(rolling_hash);
                }
                let final_opened_index = opened_index - 1;
                let (initial_height_read, height_check) = memory.read_cell(address_space, dim_base_pointer + F::from_canonical_usize(initial_opened_index));
                assert_eq!(height_check, F::from_canonical_u32(height));
                let (final_height_read, height_check) = memory.read_cell(address_space, dim_base_pointer + F::from_canonical_usize(final_opened_index));
                assert_eq!(height_check, F::from_canonical_u32(height));
                
                let hash: [F; CHUNK] = std::array::from_fn(|i| rolling_hash[i]);
                
                root = if height == initial_height {
                    hash
                } else {
                    self.compress(root, hash)
                };
                
                Some(IncorporateRowRecord {
                    chunks,
                    initial_opened_index,
                    final_opened_index,
                    initial_height_read,
                    final_height_read,
                })
            } else {
                None
            };

            let incorporate_sibling = if height == 0 {
                None
            } else {
                for _ in 0..6 {
                    memory.increment_timestamp();
                }

                let (read_root_is_on_right, root_is_on_right) = memory.read_cell(address_space, index_base_pointer + F::from_canonical_usize(proof_index));
                let root_is_on_right = root_is_on_right == F::ONE;

                let (read_sibling_array_start, sibling_array_start) = memory.unsafe_read_cell(address_space, sibling_base_pointer + F::from_canonical_usize(proof_index));
                let sibling_array_start = sibling_array_start.as_canonical_u32() as usize;

                let mut sibling = [F::ZERO; CHUNK];
                let mut reads = vec![];
                for i in 0..CHUNK {
                    let (read, value) = memory.read_cell(address_space, sibling_base_pointer + F::from_canonical_usize(sibling_array_start + i));
                    sibling[i] = value;
                    reads.push(read);
                }

                root = if root_is_on_right {
                    self.compress(sibling, root)
                } else {
                    self.compress(root, sibling)
                };

                Some(IncorporateSiblingRecord {
                    read_sibling_array_start,
                    read_root_is_on_right,
                    sibling,
                    reads,
                })
            };
            
            top_level.push(TopLevelRecord {
                incorporate_row,
                incorporate_sibling,
            });

            height /= 2;
            proof_index += 1;
        }
        
        assert_eq!(commit, root);
        self.records.push(VerifyBatchRecord {
            from_state,
            instruction,
            dim_base_pointer,
            opened_base_pointer,
            opened_length,
            sibling_base_pointer,
            index_base_pointer,
            commit_pointer,
            dim_base_pointer_read,
            opened_base_pointer_and_length_read,
            sibling_base_pointer_read,
            index_base_pointer_read,
            commit_pointer_read,
            commit_read,
            initial_height,
            top_level,
        });

        Ok(ExecutionState {
            pc: from_state.pc + DEFAULT_PC_STEP,
            timestamp: memory.timestamp(),
        })
    }

    fn get_opcode_name(&self, opcode: usize) -> String {
        assert_eq!(opcode, (VERIFY_BATCH as usize) + self.air.offset);
        String::from("VERIFY_BATCH")
    }
}
