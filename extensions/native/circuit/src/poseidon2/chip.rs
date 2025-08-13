use std::sync::{Arc, Mutex};

use openvm_circuit::{
    arch::{
        ExecutionBridge, ExecutionError, ExecutionState, InstructionExecutor, Streams, SystemPort,
    },
    system::memory::{MemoryController, OfflineMemory, RecordId},
};
use openvm_instructions::{instruction::Instruction, program::DEFAULT_PC_STEP, LocalOpcode};
use openvm_native_compiler::{
    conversion::AS,
    Poseidon2Opcode::{COMP_POS2, PERM_POS2, MULTI_OBSERVE},
    VerifyBatchOpcode::VERIFY_BATCH,
};
use openvm_poseidon2_air::{Poseidon2Config, Poseidon2SubAir, Poseidon2SubChip};
use openvm_stark_backend::{
    p3_field::{Field, PrimeField32},
    p3_maybe_rayon::prelude::{ParallelIterator, ParallelSlice},
};
use serde::{Deserialize, Serialize};

use crate::poseidon2::{
    air::{NativePoseidon2Air, VerifyBatchBus},
    CHUNK,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "F: Field")]
pub struct VerifyBatchRecord<F: Field> {
    pub from_state: ExecutionState<u32>,
    pub instruction: Instruction<F>,

    pub dim_base_pointer: F,
    pub opened_base_pointer: F,
    pub opened_length: usize,
    pub index_base_pointer: F,
    pub commit_pointer: F,

    pub dim_base_pointer_read: RecordId,
    pub opened_base_pointer_read: RecordId,
    pub opened_length_read: RecordId,
    pub index_base_pointer_read: RecordId,
    pub commit_pointer_read: RecordId,

    pub commit_read: RecordId,
    pub initial_log_height: usize,
    pub top_level: Vec<TopLevelRecord<F>>,
}

impl<F: PrimeField32> VerifyBatchRecord<F> {
    pub fn opened_element_size_inv(&self) -> F {
        self.instruction.g
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "F: Field")]
pub struct TopLevelRecord<F: Field> {
    // must be present in first record
    pub incorporate_row: Option<IncorporateRowRecord<F>>,
    // must be present in all bust last record
    pub incorporate_sibling: Option<IncorporateSiblingRecord<F>>,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "F: Field")]
pub struct IncorporateSiblingRecord<F: Field> {
    pub read_sibling_is_on_right: RecordId,
    pub sibling_is_on_right: bool,
    pub p2_input: [F; 2 * CHUNK],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "F: Field")]
pub struct IncorporateRowRecord<F: Field> {
    pub chunks: Vec<InsideRowRecord<F>>,
    pub initial_opened_index: usize,
    pub final_opened_index: usize,
    pub initial_height_read: RecordId,
    pub final_height_read: RecordId,
    pub p2_input: [F; 2 * CHUNK],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "F: Field")]
pub struct InsideRowRecord<F: Field> {
    pub cells: Vec<CellRecord>,
    pub p2_input: [F; 2 * CHUNK],
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellRecord {
    pub read: RecordId,
    pub opened_index: usize,
    pub read_row_pointer_and_length: Option<RecordId>,
    pub row_pointer: usize,
    pub row_end: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "F: Field")]
pub struct SimplePoseidonRecord<F: Field> {
    pub from_state: ExecutionState<u32>,
    pub instruction: Instruction<F>,

    pub read_input_pointer_1: RecordId,
    pub read_input_pointer_2: Option<RecordId>,
    pub read_output_pointer: RecordId,
    pub read_data_1: RecordId,
    pub read_data_2: RecordId,
    pub write_data_1: RecordId,
    pub write_data_2: Option<RecordId>,

    pub input_pointer_1: F,
    pub input_pointer_2: F,
    pub output_pointer: F,
    pub p2_input: [F; 2 * CHUNK],
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(bound = "F: Field")]
pub struct TranscriptObservationRecord<F: Field> {
    pub from_state: ExecutionState<u32>,
    pub instruction: Instruction<F>,
    pub start_idx: usize,
    pub end_idx: usize,
    pub is_first: bool,
    pub is_last: bool,
    pub curr_timestamp_increment: usize,
    pub final_timestamp_increment: usize,

    pub state_ptr: F,
    pub input_ptr: F,
    pub init_pos: F,
    pub len: usize,
    pub curr_len: usize,
    pub should_permute: bool,

    pub read_input_data: [RecordId; CHUNK],
    pub write_input_data: [RecordId; CHUNK],
    pub input_data: [F; CHUNK],

    pub read_sponge_state: RecordId,
    pub write_sponge_state: RecordId,
    pub permutation_input: [F; 2 * CHUNK],
    pub permutation_output: [F; 2 * CHUNK],

    pub write_final_idx: RecordId,
    pub final_idx: usize,

    pub input_register_1: F,
    pub input_register_2: F,
    pub input_register_3: F,
    pub output_register: F,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(bound = "F: Field")]
pub struct NativePoseidon2RecordSet<F: Field> {
    pub verify_batch_records: Vec<VerifyBatchRecord<F>>,
    pub simple_permute_records: Vec<SimplePoseidonRecord<F>>,
    pub transcript_observation_records: Vec<TranscriptObservationRecord<F>>,
}

pub struct NativePoseidon2Chip<F: Field, const SBOX_REGISTERS: usize> {
    pub(super) air: NativePoseidon2Air<F, SBOX_REGISTERS>,
    pub record_set: NativePoseidon2RecordSet<F>,
    pub height: usize,
    pub(super) offline_memory: Arc<Mutex<OfflineMemory<F>>>,
    pub(super) subchip: Poseidon2SubChip<F, SBOX_REGISTERS>,
    pub(super) streams: Arc<Mutex<Streams<F>>>,
}

impl<F: PrimeField32, const SBOX_REGISTERS: usize> NativePoseidon2Chip<F, SBOX_REGISTERS> {
    pub fn new(
        port: SystemPort,
        offline_memory: Arc<Mutex<OfflineMemory<F>>>,
        poseidon2_config: Poseidon2Config<F>,
        verify_batch_bus: VerifyBatchBus,
        streams: Arc<Mutex<Streams<F>>>,
    ) -> Self {
        let air = NativePoseidon2Air {
            execution_bridge: ExecutionBridge::new(port.execution_bus, port.program_bus),
            memory_bridge: port.memory_bridge,
            internal_bus: verify_batch_bus,
            subair: Arc::new(Poseidon2SubAir::new(poseidon2_config.constants.into())),
            address_space: F::from_canonical_u32(AS::Native as u32),
        };
        Self {
            record_set: Default::default(),
            air,
            height: 0,
            offline_memory,
            subchip: Poseidon2SubChip::new(poseidon2_config.constants),
            streams,
        }
    }

    fn compress(&self, left: [F; CHUNK], right: [F; CHUNK]) -> ([F; 2 * CHUNK], [F; CHUNK]) {
        let concatenated =
            std::array::from_fn(|i| if i < CHUNK { left[i] } else { right[i - CHUNK] });
        let permuted = self.subchip.permute(concatenated);
        (concatenated, std::array::from_fn(|i| permuted[i]))
    }
}

pub(super) const NUM_INITIAL_READS: usize = 6;
pub(super) const NUM_SIMPLE_ACCESSES: u32 = 7;

impl<F: PrimeField32, const SBOX_REGISTERS: usize> InstructionExecutor<F>
    for NativePoseidon2Chip<F, SBOX_REGISTERS>
{
    fn execute(
        &mut self,
        memory: &mut MemoryController<F>,
        instruction: &Instruction<F>,
        from_state: ExecutionState<u32>,
    ) -> Result<ExecutionState<u32>, ExecutionError> {
        if instruction.opcode == PERM_POS2.global_opcode()
            || instruction.opcode == COMP_POS2.global_opcode()
        {
            let &Instruction {
                a: output_register,
                b: input_register_1,
                c: input_register_2,
                d: register_address_space,
                e: data_address_space,
                ..
            } = instruction;

            let (read_output_pointer, output_pointer) =
                memory.read_cell(register_address_space, output_register);
            let (read_input_pointer_1, input_pointer_1) =
                memory.read_cell(register_address_space, input_register_1);
            let (read_input_pointer_2, input_pointer_2) =
                if instruction.opcode == PERM_POS2.global_opcode() {
                    memory.increment_timestamp();
                    (None, input_pointer_1 + F::from_canonical_usize(CHUNK))
                } else {
                    let (read_input_pointer_2, input_pointer_2) =
                        memory.read_cell(register_address_space, input_register_2);
                    (Some(read_input_pointer_2), input_pointer_2)
                };
            let (read_data_1, data_1) = memory.read::<CHUNK>(data_address_space, input_pointer_1);
            let (read_data_2, data_2) = memory.read::<CHUNK>(data_address_space, input_pointer_2);
            let p2_input = std::array::from_fn(|i| {
                if i < CHUNK {
                    data_1[i]
                } else {
                    data_2[i - CHUNK]
                }
            });
            let output = self.subchip.permute(p2_input);
            let (write_data_1, _) = memory.write::<CHUNK>(
                data_address_space,
                output_pointer,
                std::array::from_fn(|i| output[i]),
            );
            let write_data_2 = if instruction.opcode == PERM_POS2.global_opcode() {
                Some(
                    memory
                        .write::<CHUNK>(
                            data_address_space,
                            output_pointer + F::from_canonical_usize(CHUNK),
                            std::array::from_fn(|i| output[CHUNK + i]),
                        )
                        .0,
                )
            } else {
                memory.increment_timestamp();
                None
            };

            assert_eq!(
                memory.timestamp(),
                from_state.timestamp + NUM_SIMPLE_ACCESSES
            );

            self.record_set
                .simple_permute_records
                .push(SimplePoseidonRecord {
                    from_state,
                    instruction: instruction.clone(),
                    read_input_pointer_1,
                    read_input_pointer_2,
                    read_output_pointer,
                    read_data_1,
                    read_data_2,
                    write_data_1,
                    write_data_2,
                    input_pointer_1,
                    input_pointer_2,
                    output_pointer,
                    p2_input,
                });
            self.height += 1;
        } else if instruction.opcode == MULTI_OBSERVE.global_opcode() {
            let mut observation_records: Vec<TranscriptObservationRecord<F>> = vec![];
            
            let &Instruction {
                a: output_register,
                b: input_register_1,
                c: input_register_2,
                d: data_address_space,
                e: register_address_space,
                f: input_register_3,
                ..
            } = instruction;

            let (read_sponge_ptr, sponge_ptr) = memory.read_cell(register_address_space, output_register);
            let (read_init_pos, pos) = memory.read_cell(register_address_space, input_register_1);
            let (read_arr_ptr, arr_ptr) = memory.read_cell(register_address_space, input_register_2);
            let init_pos = pos.clone();

            let mut pos = pos.as_canonical_u32() as usize;
            let (read_len, len) = memory.read_cell(register_address_space, input_register_3);
            let init_len = len.as_canonical_u32() as usize;
            let mut len = len.as_canonical_u32() as usize;

            let mut header_record: TranscriptObservationRecord<F> = TranscriptObservationRecord {
                from_state,
                instruction: instruction.clone(),
                curr_timestamp_increment: 0,
                is_first: true,
                state_ptr: sponge_ptr, 
                input_ptr: arr_ptr, 
                init_pos,
                len: init_len,
                input_register_1,
                input_register_2,
                input_register_3,
                output_register,
                ..Default::default()
            };
            header_record.read_input_data[0] = read_sponge_ptr;
            header_record.read_input_data[1] = read_arr_ptr;
            header_record.read_input_data[2] = read_init_pos;
            header_record.read_input_data[3] = read_len;

            observation_records.push(header_record);
            self.height += 1;

            // Observe bytes
            let mut observation_chunks: Vec<(usize, usize, bool)> = vec![];
            while len > 0 {
                if len >= (CHUNK - pos) {
                    observation_chunks.push((pos.clone(), CHUNK.clone(), true));
                    len -= CHUNK - pos;
                    pos = 0;
                } else {
                    observation_chunks.push((pos.clone(), pos + len, false));
                    len = 0;
                    pos = pos + len;
                }
            }
            
            let mut curr_timestamp = 4usize;
            let mut input_idx: usize = 0;
            for chunk in observation_chunks {
                let mut record: TranscriptObservationRecord<F> = TranscriptObservationRecord {
                    from_state,
                    instruction: instruction.clone(),

                    start_idx: chunk.0,
                    end_idx: chunk.1,

                    curr_timestamp_increment: curr_timestamp,
                    state_ptr: sponge_ptr, 
                    input_ptr: arr_ptr, 
                    init_pos,
                    len: init_len,
                    curr_len: input_idx,
                    input_register_1,
                    input_register_2,
                    input_register_3,
                    output_register,
                    ..Default::default()
                };

                for j in chunk.0..chunk.1 {
                    let (n_read, n_f) = memory.read_cell(data_address_space, arr_ptr + F::from_canonical_usize(input_idx));
                    record.read_input_data[j] = n_read;
                    record.input_data[j] = n_f;
                    input_idx += 1;
                    curr_timestamp += 1;

                    let (n_write, _) = memory.write_cell(data_address_space, sponge_ptr + F::from_canonical_usize(j), n_f);
                    record.write_input_data[j] = n_write;
                    curr_timestamp += 1;
                }

                if record.end_idx >= CHUNK {
                    let (read_sponge_record, permutation_input) = memory.read::<{CHUNK * 2}>(data_address_space, sponge_ptr);
                    let output = self.subchip.permute(permutation_input);
                    let (write_sponge_record, _) = memory.write::<{CHUNK * 2}>(data_address_space, sponge_ptr, std::array::from_fn(|i| output[i]));

                    curr_timestamp += 2;

                    record.should_permute = true;
                    record.read_sponge_state = read_sponge_record;
                    record.write_sponge_state = write_sponge_record;
                    record.permutation_input = permutation_input;
                    record.permutation_output = output;
                }

                observation_records.push(record);
                self.height += 1;
            }

            let last_record = observation_records.last_mut().unwrap();
            let final_idx = last_record.end_idx % CHUNK;
            let (write_final, _) = memory.write_cell(register_address_space, input_register_1, F::from_canonical_usize(final_idx));
            last_record.is_last = true;
            last_record.write_final_idx = write_final;
            last_record.final_idx = final_idx;
            curr_timestamp += 1;

            for record in &mut observation_records {
                record.final_timestamp_increment = curr_timestamp;
            }
            self.record_set.transcript_observation_records.extend(observation_records);
        } else if instruction.opcode == VERIFY_BATCH.global_opcode() {
            let &Instruction {
                a: dim_register,
                b: opened_register,
                c: opened_length_register,
                d: proof_id_ptr,
                e: index_register,
                f: commit_register,
                g: opened_element_size_inv,
                ..
            } = instruction;
            let address_space = self.air.address_space;
            // calc inverse fast assuming opened_element_size in {1, 4}
            let mut opened_element_size = F::ONE;
            while opened_element_size * opened_element_size_inv != F::ONE {
                opened_element_size += F::ONE;
            }

            let proof_id = memory.unsafe_read_cell(address_space, proof_id_ptr);
            let (dim_base_pointer_read, dim_base_pointer) =
                memory.read_cell(address_space, dim_register);
            let (opened_base_pointer_read, opened_base_pointer) =
                memory.read_cell(address_space, opened_register);
            let (opened_length_read, opened_length) =
                memory.read_cell(address_space, opened_length_register);
            let (index_base_pointer_read, index_base_pointer) =
                memory.read_cell(address_space, index_register);
            let (commit_pointer_read, commit_pointer) =
                memory.read_cell(address_space, commit_register);
            let (commit_read, commit) = memory.read(address_space, commit_pointer);

            let opened_length = opened_length.as_canonical_u32() as usize;

            let initial_log_height = memory
                .unsafe_read_cell(address_space, dim_base_pointer)
                .as_canonical_u32();
            let mut log_height = initial_log_height as i32;
            let mut sibling_index = 0;
            let mut opened_index = 0;
            let mut top_level = vec![];

            let mut root = [F::ZERO; CHUNK];
            let sibling_proof: Vec<[F; CHUNK]> = {
                let streams = self.streams.lock().unwrap();
                let proof_idx = proof_id.as_canonical_u32() as usize;
                streams.hint_space[proof_idx]
                    .par_chunks(CHUNK)
                    .map(|c| c.try_into().unwrap())
                    .collect()
            };

            while log_height >= 0 {
                let incorporate_row = if opened_index < opened_length
                    && memory.unsafe_read_cell(
                        address_space,
                        dim_base_pointer + F::from_canonical_usize(opened_index),
                    ) == F::from_canonical_u32(log_height as u32)
                {
                    let initial_opened_index = opened_index;
                    for _ in 0..NUM_INITIAL_READS {
                        memory.increment_timestamp();
                    }
                    let mut chunks = vec![];

                    let mut row_pointer = 0;
                    let mut row_end = 0;

                    let mut prev_rolling_hash: Option<[F; 2 * CHUNK]> = None;
                    let mut rolling_hash = [F::ZERO; 2 * CHUNK];

                    let mut is_first_in_segment = true;

                    loop {
                        let mut cells = vec![];
                        for chunk_elem in rolling_hash.iter_mut().take(CHUNK) {
                            let read_row_pointer_and_length = if is_first_in_segment
                                || row_pointer == row_end
                            {
                                if is_first_in_segment {
                                    is_first_in_segment = false;
                                } else {
                                    opened_index += 1;
                                    if opened_index == opened_length
                                        || memory.unsafe_read_cell(
                                            address_space,
                                            dim_base_pointer
                                                + F::from_canonical_usize(opened_index),
                                        ) != F::from_canonical_u32(log_height as u32)
                                    {
                                        break;
                                    }
                                }
                                let (result, [new_row_pointer, row_len]) = memory.read(
                                    address_space,
                                    opened_base_pointer + F::from_canonical_usize(2 * opened_index),
                                );
                                row_pointer = new_row_pointer.as_canonical_u32() as usize;
                                row_end = row_pointer
                                    + (opened_element_size * row_len).as_canonical_u32() as usize;
                                Some(result)
                            } else {
                                memory.increment_timestamp();
                                None
                            };
                            let (read, value) = memory
                                .read_cell(address_space, F::from_canonical_usize(row_pointer));
                            cells.push(CellRecord {
                                read,
                                opened_index,
                                read_row_pointer_and_length,
                                row_pointer,
                                row_end,
                            });
                            *chunk_elem = value;
                            row_pointer += 1;
                        }
                        if cells.is_empty() {
                            break;
                        }
                        let cells_len = cells.len();
                        chunks.push(InsideRowRecord {
                            cells,
                            p2_input: rolling_hash,
                        });
                        self.height += 1;
                        prev_rolling_hash = Some(rolling_hash);
                        self.subchip.permute_mut(&mut rolling_hash);
                        if cells_len < CHUNK {
                            for _ in 0..CHUNK - cells_len {
                                memory.increment_timestamp();
                                memory.increment_timestamp();
                            }
                            break;
                        }
                    }
                    let final_opened_index = opened_index - 1;
                    let (initial_height_read, height_check) = memory.read_cell(
                        address_space,
                        dim_base_pointer + F::from_canonical_usize(initial_opened_index),
                    );
                    assert_eq!(height_check, F::from_canonical_u32(log_height as u32));
                    let (final_height_read, height_check) = memory.read_cell(
                        address_space,
                        dim_base_pointer + F::from_canonical_usize(final_opened_index),
                    );
                    assert_eq!(height_check, F::from_canonical_u32(log_height as u32));

                    let hash: [F; CHUNK] = std::array::from_fn(|i| rolling_hash[i]);

                    let (p2_input, new_root) = if log_height as u32 == initial_log_height {
                        (prev_rolling_hash.unwrap(), hash)
                    } else {
                        self.compress(root, hash)
                    };
                    root = new_root;

                    self.height += 1;
                    Some(IncorporateRowRecord {
                        chunks,
                        initial_opened_index,
                        final_opened_index,
                        initial_height_read,
                        final_height_read,
                        p2_input,
                    })
                } else {
                    None
                };

                let incorporate_sibling = if log_height == 0 {
                    None
                } else {
                    for _ in 0..NUM_INITIAL_READS {
                        memory.increment_timestamp();
                    }

                    let (read_sibling_is_on_right, sibling_is_on_right) = memory.read_cell(
                        address_space,
                        index_base_pointer + F::from_canonical_usize(sibling_index),
                    );
                    let sibling_is_on_right = sibling_is_on_right == F::ONE;
                    let sibling = sibling_proof[sibling_index];
                    let (p2_input, new_root) = if sibling_is_on_right {
                        self.compress(sibling, root)
                    } else {
                        self.compress(root, sibling)
                    };
                    root = new_root;

                    self.height += 1;
                    Some(IncorporateSiblingRecord {
                        read_sibling_is_on_right,
                        sibling_is_on_right,
                        p2_input,
                    })
                };

                top_level.push(TopLevelRecord {
                    incorporate_row,
                    incorporate_sibling,
                });

                log_height -= 1;
                sibling_index += 1;
            }

            assert_eq!(commit, root);
            self.record_set
                .verify_batch_records
                .push(VerifyBatchRecord {
                    from_state,
                    instruction: instruction.clone(),
                    dim_base_pointer,
                    opened_base_pointer,
                    opened_length,
                    index_base_pointer,
                    commit_pointer,
                    dim_base_pointer_read,
                    opened_base_pointer_read,
                    opened_length_read,
                    index_base_pointer_read,
                    commit_pointer_read,
                    commit_read,
                    initial_log_height: initial_log_height as usize,
                    top_level,
                });
        } else {
            unreachable!()
        }
        Ok(ExecutionState {
            pc: from_state.pc + DEFAULT_PC_STEP,
            timestamp: memory.timestamp(),
        })
    }

    fn get_opcode_name(&self, opcode: usize) -> String {
        if opcode == VERIFY_BATCH.global_opcode().as_usize() {
            String::from("VERIFY_BATCH")
        } else if opcode == PERM_POS2.global_opcode().as_usize() {
            String::from("PERM_POS2")
        } else if opcode == COMP_POS2.global_opcode().as_usize() {
            String::from("COMP_POS2")
        } else if opcode == MULTI_OBSERVE.global_opcode().as_usize() {
            String::from("MULTI_OBSERVE")
        }else {
            unreachable!("unsupported opcode: {}", opcode)
        }
    }
}
