use std::{borrow::{Borrow, BorrowMut}, sync::{Arc, Mutex}};
use openvm_circuit::system::memory::offline_checker::{MemoryReadAuxCols, MemoryWriteAuxCols};
use openvm_circuit_primitives_derive::AlignedBorrow;
use openvm_poseidon2_air::Poseidon2SubChip;
use openvm_circuit::{
    arch::{ExecutionBridge, ExecutionState, ExecutionError, InstructionExecutor, SystemPort},
    system::memory::{offline_checker::MemoryBridge, MemoryAddress, MemoryController, OfflineMemory, RecordId},
};
use openvm_circuit_primitives::utils::not;
use openvm_instructions::{instruction::Instruction, program::DEFAULT_PC_STEP, LocalOpcode};
use openvm_native_compiler::{
    conversion::AS,
    Poseidon2Opcode::MULTI_OBSERVE
};
use openvm_poseidon2_air::Poseidon2Config;
use openvm_stark_backend::{
    interaction::InteractionBuilder, 
    p3_air::{Air, AirBuilder, BaseAir}, 
    p3_field::{Field, FieldAlgebra, PrimeField32}, 
    p3_matrix::Matrix, 
    rap::{BaseAirWithPublicValues, PartitionedBaseAir}
};
use serde::{Deserialize, Serialize};
use openvm_circuit::system::memory::{MemoryAuxColsFactory};
use openvm_circuit_primitives::utils::next_power_of_two_or_zero;
use openvm_stark_backend::{
    config::{StarkGenericConfig, Val},
    p3_matrix::dense::RowMajorMatrix,
    prover::types::AirProofInput,
    AirRef, Chip, ChipUsageGetter,
};
const CHUNK: usize = 8;

#[repr(C)]
#[derive(AlignedBorrow)]
pub struct NativeMultiObserveCols<T> {
    pub enable: T,
    pub is_first: T,
    pub is_final: T,
    pub is_observe: T,

    pub pc: T,
    pub state_idx: T,
    pub remaining_len: T,
    pub counter: T,
    pub should_permute: T,

    /// The initial timestamp of the instruction, which must be identical for first row
    /// and all following intermediate observation rows.
    pub first_timestamp: T,
    pub curr_timestamp: T,
    pub final_timestamp_increment: T,

    pub read_state_ptr: MemoryReadAuxCols<T>,
    pub read_input_ptr: MemoryReadAuxCols<T>,
    pub state_ptr: T,
    pub input_ptr: T,

    pub read_init_pos: MemoryReadAuxCols<T>,
    pub read_len: MemoryReadAuxCols<T>,
    pub init_pos: T,
    pub len: T,

    pub read_data: MemoryReadAuxCols<T>,
    pub write_data: MemoryWriteAuxCols<T, 1>,
    pub data: T,

    pub read_sponge_state: MemoryReadAuxCols<T>,
    pub write_sponge_state: MemoryWriteAuxCols<T, { CHUNK * 2 }>,
    pub permutation_input: [T; 2 * CHUNK],
    pub permutation_output: [T; 2 * CHUNK],

    pub write_final_idx: MemoryWriteAuxCols<T, 1>,
    pub final_idx: T,

    pub input_register_1: T,
    pub input_register_2: T,
    pub input_register_3: T,
    pub output_register: T,
}

#[derive(Clone, Debug)]
pub struct NativeMultiObserveAir<F: Field> {
    pub execution_bridge: ExecutionBridge,
    pub memory_bridge: MemoryBridge,
    pub(crate) address_space: F,
}
impl<F: Field> BaseAir<F> for NativeMultiObserveAir<F> {
    fn width(&self) -> usize {
        NativeMultiObserveCols::<F>::width()
    }
}
impl<F: Field> BaseAirWithPublicValues<F> for NativeMultiObserveAir<F> {}
impl<F: Field> PartitionedBaseAir<F> for NativeMultiObserveAir<F> {}

impl<AB: InteractionBuilder> Air<AB> for NativeMultiObserveAir<AB::F> {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local: &NativeMultiObserveCols<AB::Var> = (*local).borrow();
        let next = main.row_slice(1);
        let next: &NativeMultiObserveCols<AB::Var> = (*next).borrow();

        let &NativeMultiObserveCols {
            enable,
            is_first,
            is_final,
            is_observe,
            pc,
            state_idx,
            remaining_len,
            counter,
            should_permute,
            first_timestamp,
            curr_timestamp,
            final_timestamp_increment,
            read_state_ptr,
            read_input_ptr,
            state_ptr,
            input_ptr,
            read_init_pos,
            read_len,
            init_pos,
            len,
            read_data,
            write_data,
            data,
            read_sponge_state,
            write_sponge_state,
            permutation_input,
            permutation_output,
            write_final_idx,
            final_idx,
            input_register_1,
            input_register_2,
            input_register_3,
            output_register,
        } = local;

        // self.execution_bridge
        //     .execute_and_increment_pc(
        //         AB::F::from_canonical_usize(MULTI_OBSERVE.global_opcode().as_usize()),
        //         [
        //             output_register.into(),
        //             input_register_1.into(),
        //             input_register_2.into(),
        //             self.address_space.into(),
        //             self.address_space.into(),
        //             input_register_3.into(),
        //         ],
        //         ExecutionState::new(pc, first_timestamp),
        //         final_timestamp_increment,
        //     )
        //     .eval(builder, is_first);

        // // Memory operations
        // self.memory_bridge
        //     .read(
        //         MemoryAddress::new(self.address_space, output_register),
        //         [state_ptr],
        //         first_timestamp,
        //         &read_state_ptr,
        //     )
        //     .eval(builder, is_first);

        // self.memory_bridge
        //     .read(
        //         MemoryAddress::new(self.address_space, input_register_2),
        //         [input_ptr],
        //         first_timestamp + AB::F::ONE,
        //         &read_input_ptr,
        //     )
        //     .eval(builder, is_first);

        // self.memory_bridge
        //     .read(
        //         MemoryAddress::new(self.address_space, input_register_1),
        //         [init_pos],
        //         first_timestamp + AB::F::TWO,
        //         &read_init_pos,
        //     )
        //     .eval(builder, is_first);
        
        // self.memory_bridge
        //     .read(
        //         MemoryAddress::new(self.address_space, input_register_3),
        //         [len],
        //         first_timestamp + AB::F::from_canonical_usize(3),
        //         &read_len,
        //     )
        //     .eval(builder, is_first);

        self.memory_bridge
            .read(
                MemoryAddress::new(self.address_space, input_ptr + counter - AB::F::ONE),
                [data],
                first_timestamp + curr_timestamp,
                &read_data
            )
            .eval(builder, is_observe);
            
        self.memory_bridge
            .write(
                MemoryAddress::new(
                    self.address_space,
                    state_ptr + state_idx,
                ),
                [data],
                first_timestamp + curr_timestamp + AB::F::ONE,
                &write_data,
            )
            .eval(builder, is_observe);

        self.memory_bridge
            .read(
                MemoryAddress::new(
                    self.address_space,
                    state_ptr,
                ),
                permutation_input,
                first_timestamp + curr_timestamp + AB::F::TWO,
                &read_sponge_state,
            )
            .eval(builder, should_permute);
        
        self.memory_bridge
            .write(
                MemoryAddress::new(
                    self.address_space,
                    state_ptr
                ),
                permutation_output,
                first_timestamp + curr_timestamp + AB::F::from_canonical_usize(3),
                &write_sponge_state,
            )
            .eval(builder, should_permute);

        self.memory_bridge
            .write(
                MemoryAddress::new(
                    self.address_space,
                    input_register_1,
                ),
                [final_idx],
                first_timestamp + curr_timestamp + is_first * AB::F::from_canonical_usize(4) + is_observe * AB::F::TWO + should_permute * AB::F::TWO,
                &write_final_idx
            )
            .eval(builder, is_final);

        // Binary indicators columns
        builder.assert_bool(enable);
        builder.assert_bool(is_first);
        builder.assert_bool(is_observe);
        builder.assert_bool(is_final);
        builder.assert_bool(should_permute);

        // Except header rows, any other rows must observe an element
        // All rows following the header row must observe an element
        builder
            .when(enable)
            .assert_eq(is_first + is_observe, AB::F::ONE);
        builder
            .when(is_observe)
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(next.is_observe, AB::F::ONE);

        // Each non-header row must process a field element
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(next.counter, counter + AB::F::ONE);
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(next.remaining_len, remaining_len - AB::F::ONE);

        // Boundary conditions
        // At the header row, counter must be 0, remaining_len must be len
        builder
            .when(is_first)
            .assert_eq(counter, AB::F::ZERO);
        builder
            .when(is_first)
            .assert_eq(remaining_len, len);

        // At the final row, counter must be len, remaining_len must be 0
        builder
            .when(is_final)
            .assert_eq(counter, len);
        builder
            .when(is_final)
            .assert_eq(remaining_len, AB::F::ZERO);

        // After each permutation, the sponge state index must revert to 0
        builder
            .when(should_permute)
            .when(not(next.is_first))
            .assert_eq(next.state_idx, AB::F::ZERO);

        // Timestamp constraints
        builder
            .when(is_first)
            .assert_eq(curr_timestamp, AB::F::ZERO);
        builder
            .when(is_first)
            .when(not(is_final))
            .assert_eq(next.curr_timestamp - curr_timestamp, AB::F::from_canonical_usize(4));
        builder
            .when(is_observe)
            .when(not(is_final))
            .when(not(should_permute))
            .assert_eq(next.curr_timestamp - curr_timestamp, AB::F::TWO);
        builder
            .when(is_observe)
            .when(not(is_final))
            .when(should_permute)
            .assert_eq(next.curr_timestamp - curr_timestamp, AB::F::from_canonical_usize(4));

        // Fields that remain constant for a single MULTI_OBSERVE call
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(state_ptr, next.state_ptr);
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(input_ptr, next.input_ptr);
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(init_pos, next.init_pos);
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(len, next.len);
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(input_register_1, next.input_register_1);
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(input_register_2, next.input_register_2);
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(input_register_3, next.input_register_3);
        builder
            .when(next.enable)
            .when(not(next.is_first))
            .assert_eq(output_register, next.output_register);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(bound = "F: Field")]
pub struct TranscriptObservationRecord<F: Field> {
    pub from_state: ExecutionState<u32>,
    pub instruction: Instruction<F>,
    pub curr_timestamp: usize,
    pub final_timestamp_increment: usize,
    
    pub state_idx: usize,
    
    pub is_first: bool,
    pub is_observe: bool,
    pub is_final: bool,
    pub remaining_len: usize,
    pub counter: usize,
    pub should_permute: bool,

    pub read_state_ptr: RecordId,
    pub read_input_ptr: RecordId,
    pub state_ptr: F,
    pub input_ptr: F,

    pub read_init_pos: RecordId,
    pub init_pos: F,
    pub read_len: RecordId,
    pub len: usize,

    pub read_input_data: RecordId,
    pub write_input_data: RecordId,
    pub input_data: F,

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
pub struct NativeMultiObserveRecordSet<F: Field> {
    pub transcript_observation_records: Vec<TranscriptObservationRecord<F>>,
}

pub struct NativeMultiObserveChip<F: Field> {
    pub(super) air: NativeMultiObserveAir<F>,
    pub record_set: NativeMultiObserveRecordSet<F>,
    pub height: usize,
    pub(super) offline_memory: Arc<Mutex<OfflineMemory<F>>>,
    pub(super) subchip: Poseidon2SubChip<F, 1>,
}

impl<F: PrimeField32> NativeMultiObserveChip<F> {
    pub fn new(
        port: SystemPort,
        offline_memory: Arc<Mutex<OfflineMemory<F>>>,
        poseidon2_config: Poseidon2Config<F>,
    ) -> Self {
        let air = NativeMultiObserveAir {
            execution_bridge: ExecutionBridge::new(port.execution_bus, port.program_bus),
            memory_bridge: port.memory_bridge,
            address_space: F::from_canonical_u32(AS::Native as u32),
        };
        Self {
            record_set: Default::default(),
            air,
            height: 0,
            offline_memory,
            subchip: Poseidon2SubChip::new(poseidon2_config.constants),
        }
    }

    fn record_to_row(
        &self,
        record: &TranscriptObservationRecord<F>,
        aux_cols_factory: &MemoryAuxColsFactory<F>,
        slice: &mut [F],
        memory: &OfflineMemory<F>,
    ) {
        let cols: &mut NativeMultiObserveCols<F> = slice.borrow_mut();

        cols.enable = F::ONE;
        cols.is_first = if record.is_first { F::ONE } else { F::ZERO };
        cols.is_final = if record.is_final { F::ONE } else { F::ZERO };
        cols.is_observe = if !record.is_first { F::ONE } else { F::ZERO };
        cols.should_permute = if record.should_permute { F::ONE } else { F::ZERO };

        let read_state_ptr_record = memory.record_by_id(record.read_state_ptr);
        let read_input_ptr_record = memory.record_by_id(record.read_input_ptr);
        let read_init_pos_record = memory.record_by_id(record.read_init_pos);
        let read_len_record = memory.record_by_id(record.read_len);

        cols.state_ptr = record.state_ptr;
        cols.input_ptr = record.input_ptr;
        cols.init_pos = record.init_pos;
        cols.len = F::from_canonical_usize(record.len);
        cols.data = record.input_data;
        cols.curr_timestamp = F::from_canonical_usize(record.curr_timestamp);
        cols.final_timestamp_increment = F::from_canonical_usize(record.final_timestamp_increment);

        cols.permutation_input = record.permutation_input;
        cols.permutation_output = record.permutation_output;

        cols.state_idx = F::from_canonical_usize(record.state_idx);
        cols.remaining_len = F::from_canonical_usize(record.remaining_len);
        cols.counter = F::from_canonical_usize(record.counter);
        cols.final_idx = F::from_canonical_usize(record.final_idx);

        cols.first_timestamp = F::from_canonical_u32(record.from_state.timestamp);
        cols.pc = F::from_canonical_u32(record.from_state.pc);

        cols.input_register_1 = record.input_register_1;
        cols.input_register_2 = record.input_register_2;
        cols.input_register_3 = record.input_register_3;
        cols.output_register = record.output_register;

        if record.is_first {
            aux_cols_factory.generate_read_aux(read_state_ptr_record, &mut cols.read_state_ptr);
            aux_cols_factory.generate_read_aux(read_input_ptr_record, &mut cols.read_input_ptr);
            aux_cols_factory.generate_read_aux(read_init_pos_record, &mut cols.read_init_pos);
            aux_cols_factory.generate_read_aux(read_len_record, &mut cols.read_len);
        }

        if record.is_observe {
            let read_data_record = memory.record_by_id(record.read_input_data);
            let write_data_record = memory.record_by_id(record.write_input_data);
            aux_cols_factory.generate_read_aux(read_data_record, &mut cols.read_data);
            aux_cols_factory.generate_write_aux(write_data_record, &mut cols.write_data);
        }

        if record.should_permute {
            let read_sponge_record = memory.record_by_id(record.read_sponge_state);
            let write_sponge_record = memory.record_by_id(record.write_sponge_state);
            aux_cols_factory.generate_read_aux(read_sponge_record, &mut cols.read_sponge_state);
            aux_cols_factory.generate_write_aux(write_sponge_record, &mut cols.write_sponge_state);
        }

        if record.is_final {
            let write_final_idx_record = memory.record_by_id(record.write_final_idx);
            aux_cols_factory.generate_write_aux(write_final_idx_record, &mut cols.write_final_idx);
        }
    }

    fn generate_trace(self) -> RowMajorMatrix<F> {
        let width = self.trace_width();
        let height = next_power_of_two_or_zero(self.height);
        let mut flat_trace = F::zero_vec(width * height);

        let memory = self.offline_memory.lock().unwrap();
        let aux_cols_factory = memory.aux_cols_factory();
        let mut used_cells = 0;
        for record in self.record_set.transcript_observation_records.iter() {
            self.record_to_row(
                record,
                &aux_cols_factory,
                &mut flat_trace[used_cells..used_cells + width],
                &memory,
            );
            used_cells += width;
        }

        RowMajorMatrix::new(flat_trace, width)
    }
}

impl<F: PrimeField32> InstructionExecutor<F> for NativeMultiObserveChip<F> {
    fn execute(
        &mut self,
        memory: &mut MemoryController<F>,
        instruction: &Instruction<F>,
        from_state: ExecutionState<u32>,
    ) -> Result<ExecutionState<u32>, ExecutionError> {
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
        let (read_arr_ptr, arr_ptr) = memory.read_cell(register_address_space, input_register_2);
        let (read_init_pos, pos) = memory.read_cell(register_address_space, input_register_1);
        let init_pos = pos.clone();
        let mut pos = pos.as_canonical_u32() as usize;
        let (read_len, len) = memory.read_cell(register_address_space, input_register_3);
        let len = len.as_canonical_u32() as usize;

        let head_record: TranscriptObservationRecord<F> = TranscriptObservationRecord {
            from_state,
            instruction: instruction.clone(),
            is_first: true,
            state_idx: pos % CHUNK,
            remaining_len: len, 
            counter: 0,
            read_state_ptr: read_sponge_ptr, 
            read_input_ptr: read_arr_ptr,
            state_ptr: sponge_ptr, 
            input_ptr: arr_ptr, 
            read_init_pos: read_init_pos, 
            init_pos, 
            read_len, 
            len,
            input_register_1,
            input_register_2,
            input_register_3,
            output_register,
            ..Default::default()
        };
        
        self.height += 1;
        observation_records.push(head_record);

        let mut curr_timestamp = 4usize;

        
        for i in 0..len {
            let mut record: TranscriptObservationRecord<F> = TranscriptObservationRecord {
                from_state,
                curr_timestamp,
                instruction: instruction.clone(),
                is_observe: true,
                state_idx: pos % CHUNK,
                remaining_len: len - i - 1, 
                counter: i + 1, 
                read_state_ptr: read_sponge_ptr, 
                read_input_ptr: read_arr_ptr,
                state_ptr: sponge_ptr, 
                input_ptr: arr_ptr, 
                read_init_pos: read_init_pos, 
                init_pos, 
                read_len, 
                len,
                input_register_1,
                input_register_2,
                input_register_3,
                output_register,
                ..Default::default()
            };

            let (n_read, n_f) = memory.read_cell(data_address_space, arr_ptr + F::from_canonical_usize(i));
            record.read_input_data = n_read;
            record.input_data = n_f;
            curr_timestamp += 1;

            let (n_write, _) = memory.write_cell(data_address_space, sponge_ptr + F::from_canonical_usize(record.state_idx), record.input_data);
            record.write_input_data = n_write;
            curr_timestamp += 1;

            pos += 1;

            if pos % CHUNK == 0 {
                record.should_permute = true;
                let (read_sponge_record, permutation_input) = memory.read::<{CHUNK * 2}>(data_address_space, sponge_ptr);
                let output = self.subchip.permute(permutation_input);
                let (write_sponge_record, _) = memory.write::<{CHUNK * 2}>(data_address_space, sponge_ptr, std::array::from_fn(|i| output[i]));

                curr_timestamp += 2;

                record.read_sponge_state = read_sponge_record;
                record.write_sponge_state = write_sponge_record;
                record.permutation_input = permutation_input;
                record.permutation_output = output;
            }

            observation_records.push(record);
            self.height += 1;
        }
        
        let mod_pos = pos % CHUNK;
        let (write_final, _) = memory.write_cell(register_address_space, input_register_1, F::from_canonical_usize(mod_pos));
        curr_timestamp += 1;

        observation_records[0].is_first = true;
        observation_records.last_mut().unwrap().is_final = true;
        observation_records.last_mut().unwrap().write_final_idx = write_final;
        observation_records.last_mut().unwrap().final_idx = mod_pos;

        for record in &mut observation_records {
            record.final_timestamp_increment = curr_timestamp;
        }

        for record in observation_records {
            self.record_set.transcript_observation_records.push(record);
        }
        
        Ok(ExecutionState {
            pc: from_state.pc + DEFAULT_PC_STEP,
            timestamp: memory.timestamp(),
        })
    }

    fn get_opcode_name(&self, opcode: usize) -> String {
        if opcode == MULTI_OBSERVE.global_opcode().as_usize() {
            String::from("MULTI_OBSERVE")
        } else {
            unreachable!("unsupported opcode: {}", opcode)
        }
    }
}

impl<F: Field> ChipUsageGetter for NativeMultiObserveChip<F> {
    fn air_name(&self) -> String {
        "MultiObserve".to_string()
    }

    fn current_trace_height(&self) -> usize {
        self.height
    }

    fn trace_width(&self) -> usize {
        NativeMultiObserveCols::<F>::width()
    }
}

impl<SC: StarkGenericConfig> Chip<SC>
    for NativeMultiObserveChip<Val<SC>>
where
    Val<SC>: PrimeField32,
{
    fn air(&self) -> AirRef<SC> {
        Arc::new(self.air.clone())
    }
    fn generate_air_proof_input(self) -> AirProofInput<SC> {
        AirProofInput::simple_no_pis(self.generate_trace())
    }
}