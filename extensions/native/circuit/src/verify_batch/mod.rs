use std::{
    borrow::{Borrow, BorrowMut},
    sync::{Arc, Mutex},
};

use openvm_circuit::{
    arch::{ExecutionBridge, ExecutionBus, ExecutionError, ExecutionState, InstructionExecutor},
    system::{
        memory::{
            offline_checker::{
                MemoryBaseAuxCols, MemoryBridge, MemoryReadAuxCols, MemoryWriteAuxCols,
            },
            MemoryAddress, MemoryAuxColsFactory, MemoryController, OfflineMemory, RecordId,
        },
        program::ProgramBus,
    },
};
use openvm_circuit_primitives::{
    is_zero::{IsZeroIo, IsZeroSubAir},
    utils::{assert_array_eq, next_power_of_two_or_zero, not},
    SubAir, TraceSubRowGenerator,
};
use openvm_circuit_primitives_derive::AlignedBorrow;
use openvm_instructions::{instruction::Instruction, program::DEFAULT_PC_STEP};
use openvm_native_compiler::FriOpcode::FRI_REDUCED_OPENING;
use openvm_poseidon2_air::{BABY_BEAR_POSEIDON2_HALF_FULL_ROUNDS, Poseidon2SubAir, Poseidon2SubCols};
use openvm_stark_backend::{
    config::{StarkGenericConfig, Val},
    interaction::InteractionBuilder,
    p3_air::{Air, AirBuilder, BaseAir},
    p3_field::{Field, FieldAlgebra, PrimeField32},
    p3_matrix::{dense::RowMajorMatrix, Matrix},
    p3_maybe_rayon::prelude::*,
    prover::types::AirProofInput,
    rap::{AnyRap, BaseAirWithPublicValues, PartitionedBaseAir},
    Chip, ChipUsageGetter,
};

use super::field_extension::{FieldExtension, EXT_DEG};
use crate::NATIVE_POSEIDON2_CHUNK_SIZE;

#[cfg(test)]
mod tests;

const CHUNK: usize = NATIVE_POSEIDON2_CHUNK_SIZE;

#[repr(C)]
#[derive(AlignedBorrow)]
pub struct VerifyBatchCols<T, const SBOX_REGISTERS: usize> {
    // flags - at most 1 is true, if none is true then row is disabled
    pub incorporate_row: T,
    pub incorporate_sibling: T,
    pub inside_row: T,

    pub end_inside_row: T,
    pub end_top_level: T,
    pub start: T,

    // execution state
    pub pc: T,
    pub very_first_timestamp: T,
    pub start_timestamp: T,
    pub end_timestamp: T,

    // instruction (a, b, c, d, e)
    pub dim_register: T,
    pub opened_register: T,
    pub sibling_register: T,
    pub index_register: T,
    pub commit_register: T,
    pub address_space: T,

    // poseidon2
    pub inner: Poseidon2SubCols<T, SBOX_REGISTERS>,

    pub cells: [VerifyBatchCellCols<T, SBOX_REGISTERS>; CHUNK],
    // initial/final opened index for a subsegment with same height
    // initial is used in both, final is used only in top level
    pub initial_opened_index: T,
    pub final_opened_index: T,

    pub height: T,
    pub opened_length: T,

    pub dim_base_pointer: T,
    pub opened_base_pointer: T,
    pub sibling_base_pointer: T,
    pub index_base_pointer: T,

    pub dim_base_pointer_read: MemoryReadAuxCols<T, 1>,
    pub opened_base_pointer_and_length_read: MemoryReadAuxCols<T, 2>,
    pub sibling_base_pointer_read: MemoryReadAuxCols<T, 1>,
    pub index_base_pointer_read: MemoryReadAuxCols<T, 1>,
    pub commit_pointer_read: MemoryReadAuxCols<T, 1>,
    
    pub proof_index: T,
    
    pub read_initial_height_or_root_is_on_right: MemoryReadAuxCols<T, 1>,
    pub read_final_height_or_sibling_array_start: MemoryReadAuxCols<T, 1>,
    
    pub root_is_on_right: T,
    pub sibling_array_start: T,

    pub commit_pointer: T,
    pub commit_read: MemoryReadAuxCols<T, CHUNK>,
}

#[repr(C)]
#[derive(AlignedBorrow, Copy, Clone)]
pub struct VerifyBatchCellCols<T, const SBOX_REGISTERS: usize> {
    pub read: MemoryReadAuxCols<T, 1>,
    pub opened_index: T,
    pub read_row_pointer_and_length: MemoryReadAuxCols<T, 2>,
    pub row_pointer: T,
    pub row_end: T,
    pub is_first_in_row: T,
    pub is_exhausted: T,
}

#[derive(Clone, Copy, Debug)]
pub struct VerifyBatchBus(usize);

impl VerifyBatchBus {
    pub fn interact<AB: InteractionBuilder>(
        &self,
        builder: &mut AB,
        send: bool,
        multiplicity: impl Into<AB::Expr>,
        start_timestamp: impl Into<AB::Expr>,
        end_timestamp: impl Into<AB::Expr>,
        opened_base_pointer: impl Into<AB::Expr>,
        initial_opened_index: impl Into<AB::Expr>,
        final_opened_index: impl Into<AB::Expr>,
        hash: [impl Into<AB::Expr>; CHUNK],
    ) {
    }
}
#[derive(Clone, Debug)]
pub struct VerifyBatchAir<F: Field, const SBOX_REGISTERS: usize> {
    pub execution_bridge: ExecutionBridge,
    pub memory_bridge: MemoryBridge,
    pub internal_bus: VerifyBatchBus,
    pub(super) subair: Arc<Poseidon2SubAir<F, SBOX_REGISTERS>>,
    offset: usize,
    half: F,
}

impl<F: Field, const SBOX_REGISTERS: usize> BaseAir<F> for VerifyBatchAir<F, SBOX_REGISTERS> {
    fn width(&self) -> usize {
        VerifyBatchCols::<F, SBOX_REGISTERS>::width()
    }
}

impl<F: Field, const SBOX_REGISTERS: usize> BaseAirWithPublicValues<F>
    for VerifyBatchAir<F, SBOX_REGISTERS>
{
}
impl<F: Field, const SBOX_REGISTERS: usize> PartitionedBaseAir<F>
    for VerifyBatchAir<F, SBOX_REGISTERS>
{
}

impl<AB: InteractionBuilder, const SBOX_REGISTERS: usize> Air<AB>
    for VerifyBatchAir<AB::F, SBOX_REGISTERS>
{
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local: &VerifyBatchCols<AB::Var, SBOX_REGISTERS> = (*local).borrow();
        let next = main.row_slice(1);
        let next: &VerifyBatchCols<AB::Var, SBOX_REGISTERS> = (*next).borrow();

        let &VerifyBatchCols {
            incorporate_row,
            incorporate_sibling,
            inside_row,
            end_inside_row,
            end_top_level,
            start,
            pc,
            very_first_timestamp,
            start_timestamp,
            end_timestamp,
            dim_register,
            opened_register,
            sibling_register,
            index_register,
            commit_register,
            address_space,
            inner,
            cells,
            initial_opened_index,
            final_opened_index,
            height,
            opened_length,
            dim_base_pointer,
            opened_base_pointer,
            sibling_base_pointer,
            index_base_pointer,
            read_initial_height_or_root_is_on_right,
            read_final_height_or_sibling_array_start,
            dim_base_pointer_read,
            opened_base_pointer_and_length_read,
            sibling_base_pointer_read,
            index_base_pointer_read,
            commit_pointer_read,
            root_is_on_right,
            commit_pointer,
            commit_read,
            proof_index,
            sibling_array_start,
        } = local;

        let left_input = std::array::from_fn::<_, CHUNK, _>(|i| inner.inputs[i]);
        let right_input = std::array::from_fn::<_, CHUNK, _>(|i| inner.inputs[i + CHUNK]);
        let left_output =
            std::array::from_fn::<_, CHUNK, _>(|i| inner.ending_full_rounds[BABY_BEAR_POSEIDON2_HALF_FULL_ROUNDS - 1].post[i]);
        let right_output =
            std::array::from_fn::<_, CHUNK, _>(|i| inner.ending_full_rounds[BABY_BEAR_POSEIDON2_HALF_FULL_ROUNDS - 1].post[i + CHUNK]);
        let next_left_input = std::array::from_fn::<_, CHUNK, _>(|i| next.inner.inputs[i]);
        let next_right_input = std::array::from_fn::<_, CHUNK, _>(|i| next.inner.inputs[i + CHUNK]);
        let next_left_output =
            std::array::from_fn::<_, CHUNK, _>(|i| next.inner.ending_full_rounds[BABY_BEAR_POSEIDON2_HALF_FULL_ROUNDS - 1].post[i]);
        let next_right_output =
            std::array::from_fn::<_, CHUNK, _>(|i| next.inner.ending_full_rounds[BABY_BEAR_POSEIDON2_HALF_FULL_ROUNDS - 1].post[i + CHUNK]);

        builder.assert_bool(incorporate_row);
        builder.assert_bool(incorporate_sibling);
        builder.assert_bool(inside_row);
        let enabled = incorporate_row + incorporate_sibling + inside_row;
        builder.assert_bool(enabled.clone());
        builder.assert_bool(end_inside_row);
        builder.when(end_inside_row).assert_one(inside_row);
        builder.assert_bool(end_top_level);
        builder.when(end_top_level).assert_one(incorporate_row);

        let end = end_inside_row + end_top_level + (AB::Expr::ONE - enabled.clone());
        builder.assert_eq(next.start, end.clone());
        builder.when(end.clone()).assert_zero(next.incorporate_sibling);

        self.subair.eval(builder);

        //// inside row constraints

        // start
        builder
            .when(end.clone())
            .when(next.inside_row)
            .assert_eq(next.initial_opened_index, next.cells[0].opened_index);
        builder
            .when(end.clone())
            .when(next.inside_row)
            .assert_eq(next.very_first_timestamp, next.start_timestamp);

        // end
        self.internal_bus.interact(
            builder,
            false,
            end_inside_row,
            very_first_timestamp,
            start_timestamp + AB::F::from_canonical_usize(2 * CHUNK),
            opened_base_pointer,
            initial_opened_index,
            cells[CHUNK - 1].opened_index,
            left_output,
        );

        // things that stay the same (roughly)

        builder.when(inside_row - end_inside_row).assert_eq(
            next.start_timestamp,
            start_timestamp + AB::F::from_canonical_usize(2 * CHUNK),
        );
        builder
            .when(inside_row - end_inside_row)
            .assert_eq(next.opened_base_pointer, opened_base_pointer);
        builder
            .when(inside_row - end_inside_row)
            .assert_eq(next.dim_base_pointer, dim_base_pointer);
        builder
            .when(inside_row - end_inside_row)
            .assert_eq(next.initial_opened_index, initial_opened_index);
        builder
            .when(inside_row - end_inside_row)
            .assert_eq(next.very_first_timestamp, very_first_timestamp);

        // right input

        for i in 0..CHUNK {
            builder
                .when(end.clone())
                .when(next.inside_row)
                .assert_zero(next_right_input[i]);
        }

        for i in 0..CHUNK {
            builder
                .when(inside_row - end_inside_row)
                .assert_eq(right_output[i], next_right_input[i]);
        }

        // left input

        for i in 0..CHUNK {
            let cell = cells[i];
            let next_cell = if i + 1 == CHUNK {
                next.cells[0]
            } else {
                cells[i + 1]
            };

            builder.assert_bool(cell.is_first_in_row);
            builder.assert_bool(cell.is_exhausted);
            builder.assert_bool(cell.is_first_in_row + cell.is_exhausted);

            let next_is_normal = AB::Expr::ONE - next_cell.is_first_in_row - next_cell.is_exhausted;
            self.memory_bridge
                .read(
                    MemoryAddress::new(address_space, cell.row_pointer),
                    [left_input[i]],
                    start_timestamp + AB::F::from_canonical_usize((2 * i) + 1),
                    &cell.read,
                )
                .eval(builder, inside_row * not(cell.is_exhausted));
            builder
                .when(cell.is_exhausted)
                .assert_eq(left_input[i], AB::F::ZERO);

            let mut when_inside_row_not_last = builder.when(inside_row - end_inside_row);

            // update state for normal cell
            when_inside_row_not_last
                .when(next_is_normal.clone())
                .assert_eq(next_cell.row_pointer, cell.row_pointer + AB::F::ONE);
            when_inside_row_not_last
                .when(next_is_normal.clone())
                .assert_eq(next_cell.row_end, cell.row_end);
            when_inside_row_not_last
                .when(AB::Expr::ONE - next_cell.is_first_in_row)
                .assert_eq(next_cell.opened_index, cell.opened_index);

            // update state for first in row cell
            self.memory_bridge
                .read(
                    MemoryAddress::new(
                        address_space,
                        opened_base_pointer + (cell.opened_index * AB::F::TWO),
                    ),
                    [cell.row_pointer.into(), cell.row_end - cell.row_pointer],
                    start_timestamp + AB::F::from_canonical_usize(2 * i),
                    &cell.read_row_pointer_and_length,
                )
                .eval(builder, inside_row * cell.is_first_in_row);
            let mut when_inside_row_not_last = builder.when(inside_row - end_inside_row);
            when_inside_row_not_last
                .when(next_cell.is_first_in_row)
                .assert_eq(next_cell.opened_index, cell.opened_index + AB::F::ONE);

            when_inside_row_not_last
                .when(cell.is_exhausted)
                .assert_eq(next_cell.is_exhausted, AB::F::ONE);

            let is_last_in_row = if i == CHUNK - 1 {
                end_inside_row.into()
            } else {
                next_cell.is_first_in_row + next_cell.is_exhausted
            };
            builder
                .when(inside_row)
                .when(is_last_in_row)
                .assert_eq(cell.row_pointer, cell.row_end);
        }
        
        //// top level constraints
        
        builder.when(end.clone()).when(next.incorporate_row + next.incorporate_sibling).assert_eq(next.proof_index, AB::F::ZERO);
        
        let timestamp_after_end_operations = start_timestamp + AB::F::from_canonical_usize(5 + 1);
        
        builder
            .when(end.clone())
            .when(next.incorporate_row)
            .assert_eq(next.initial_opened_index, AB::F::ZERO);
        self.execution_bridge.execute_and_increment_pc(
            AB::Expr::from_canonical_usize(FRI_REDUCED_OPENING as usize + self.offset),
            [dim_register, opened_register, sibling_register, index_register, commit_register, address_space],
            ExecutionState::new(pc, very_first_timestamp),
            end_timestamp - very_first_timestamp,
        ).eval(builder, end_top_level);
        
        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    dim_register,
                ),
                [dim_base_pointer],
                start_timestamp,
                &dim_base_pointer_read,
            )
            .eval(builder, end_top_level);
        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    opened_register,
                ),
                [opened_base_pointer, opened_length],
                start_timestamp + AB::F::ONE,
                &opened_base_pointer_and_length_read,
            )
            .eval(builder, end_top_level);
        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    sibling_register,
                ),
                [sibling_base_pointer],
                start_timestamp + AB::F::TWO,
                &sibling_base_pointer_read,
            )
            .eval(builder, end_top_level);
        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    index_register,
                ),
                [index_base_pointer],
                start_timestamp + AB::F::from_canonical_usize(3),
                &index_base_pointer_read,
            )
            .eval(builder, end_top_level);
        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    commit_register,
                ),
                [commit_pointer],
                start_timestamp + AB::F::from_canonical_usize(4),
                &commit_pointer_read
            )
            .eval(builder, end_top_level);

        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    commit_pointer,
                ),
                left_output,
                start_timestamp + AB::F::from_canonical_usize(5),
                &commit_read,
            )
            .eval(builder, end_top_level);
        
        let mut when_top_level_not_end = builder
            .when(incorporate_row + incorporate_sibling - end_top_level);

        when_top_level_not_end
            .assert_eq(next.dim_base_pointer, dim_base_pointer);
        when_top_level_not_end
            .assert_eq(next.opened_base_pointer, opened_base_pointer);
        when_top_level_not_end
            .assert_eq(next.sibling_base_pointer, sibling_base_pointer);
        when_top_level_not_end
            .assert_eq(next.index_base_pointer, index_base_pointer);
        when_top_level_not_end
            .assert_eq(next.start_timestamp, end_timestamp);
        when_top_level_not_end
            .assert_eq(next.opened_length, opened_length);
        when_top_level_not_end
            .assert_eq(next.initial_opened_index, final_opened_index + AB::F::ONE);
        
        builder.when(incorporate_sibling - end_top_level).assert_eq(next.height * AB::F::TWO, height);
        builder.when(incorporate_row - end_top_level).assert_eq(next.height, height);
        builder.when(incorporate_sibling - end_top_level).assert_eq(next.proof_index, proof_index + AB::F::ONE);
        builder.when(incorporate_row - end_top_level).assert_eq(next.proof_index, proof_index);
        
        builder.when(end_top_level).assert_eq(height, AB::F::ONE);
        
        // incorporate row
        
        builder.when(incorporate_row - end_top_level).assert_one(next.incorporate_sibling);
        
        let row_hash = std::array::from_fn(|i| (start * left_output[i]) + ((AB::Expr::ONE - start) * right_input[i]));

        self.internal_bus.interact(
            builder,
            true,
            incorporate_row,
            timestamp_after_end_operations.clone(),
            end_timestamp - AB::F::TWO,
            opened_base_pointer,
            initial_opened_index,
            final_opened_index,
            row_hash,
        );
        
        for i in 0..CHUNK {
            builder.when(AB::Expr::ONE - end.clone()).when(next.incorporate_row).assert_eq(next_left_input[i], left_output[i]);
        }
        
        builder.when(end_top_level).when(incorporate_row).assert_eq(final_opened_index, opened_length - AB::F::ONE);
        
        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    dim_base_pointer + initial_opened_index,
                ),
                [height],
                end_timestamp - AB::F::TWO,
                &read_initial_height_or_root_is_on_right,
            )
            .eval(builder, incorporate_row);
        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    dim_base_pointer + final_opened_index,
                ),
                [height],
                end_timestamp - AB::F::ONE,
                &read_initial_height_or_root_is_on_right,
            )
            .eval(builder, incorporate_row);
        
        // incorporate sibling
        
        builder.when(incorporate_sibling - end_top_level).assert_one(next.incorporate_row + next.incorporate_sibling);
        builder.when(end_top_level).when(incorporate_sibling).assert_eq(initial_opened_index, opened_length);
        
        builder.when(incorporate_sibling).assert_eq(final_opened_index + AB::F::ONE, initial_opened_index);

        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    index_base_pointer + proof_index,
                ),
                [root_is_on_right],
                timestamp_after_end_operations.clone(),
                &read_initial_height_or_root_is_on_right,
            )
            .eval(builder, incorporate_row);
        self.memory_bridge
            .read(
                MemoryAddress::new(
                    address_space,
                    sibling_base_pointer + proof_index,
                ),
                [sibling_array_start],
                timestamp_after_end_operations.clone() + AB::F::ONE,
                &read_final_height_or_sibling_array_start
            )
            .eval(builder, incorporate_row);
        
        for i in 0..CHUNK {
            builder.when(next.incorporate_sibling).when(next.root_is_on_right).assert_eq(next_right_input[i], left_output[i]);
            builder.when(next.incorporate_sibling).when(AB::Expr::ONE - next.root_is_on_right).assert_eq(next_left_input[i], left_output[i]);

            self.memory_bridge
                .read(
                    MemoryAddress::new(
                        address_space,
                        sibling_array_start + proof_index,
                    ),
                    [(root_is_on_right * left_input[i]) + ((AB::Expr::ONE - root_is_on_right) * right_input[i])],
                    timestamp_after_end_operations.clone() + AB::F::from_canonical_usize(2 + i),
                    &read_initial_height_or_root_is_on_right,
                )
                .eval(builder, incorporate_row);
        }
        
        builder.assert_eq(end_timestamp, timestamp_after_end_operations + AB::F::from_canonical_usize(2 + CHUNK));
    }
}

pub struct FriReducedOpeningRecord<F: Field> {
    pub pc: F,
    pub start_timestamp: F,
    pub instruction: Instruction<F>,
    pub alpha_read: RecordId,
    pub length_read: RecordId,
    pub a_ptr_read: RecordId,
    pub b_ptr_read: RecordId,
    pub a_reads: Vec<RecordId>,
    pub b_reads: Vec<RecordId>,
    pub alpha_pow_original: [F; EXT_DEG],
    pub alpha_pow_write: RecordId,
    pub result_write: RecordId,
}

pub struct FriReducedOpeningChip<F: Field> {
    air: VerifyBatchAir<F>,
    records: Vec<FriReducedOpeningRecord<F>>,
    height: usize,
    offline_memory: Arc<Mutex<OfflineMemory<F>>>,
}

impl<F: PrimeField32> FriReducedOpeningChip<F> {
    pub fn new(
        execution_bus: ExecutionBus,
        program_bus: ProgramBus,
        memory_bridge: MemoryBridge,
        offset: usize,
        offline_memory: Arc<Mutex<OfflineMemory<F>>>,
    ) -> Self {
        let air = VerifyBatchAir {
            execution_bridge: ExecutionBridge::new(execution_bus, program_bus),
            memory_bridge,
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

fn elem_to_ext<F: Field>(elem: F) -> [F; EXT_DEG] {
    let mut ret = [F::ZERO; EXT_DEG];
    ret[0] = elem;
    ret
}

impl<F: PrimeField32> InstructionExecutor<F> for FriReducedOpeningChip<F> {
    fn execute(
        &mut self,
        memory: &mut MemoryController<F>,
        instruction: &Instruction<F>,
        from_state: ExecutionState<u32>,
    ) -> Result<ExecutionState<u32>, ExecutionError> {
        let &Instruction {
            a: a_ptr_ptr,
            b: b_ptr_ptr,
            c: result_ptr,
            d: addr_space,
            e: length_ptr,
            f: alpha_ptr,
            g: alpha_pow_ptr,
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

        self.records.push(FriReducedOpeningRecord {
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

impl<F: Field> ChipUsageGetter for FriReducedOpeningChip<F> {
    fn air_name(&self) -> String {
        "FriReducedOpeningAir".to_string()
    }

    fn current_trace_height(&self) -> usize {
        self.height
    }

    fn trace_width(&self) -> usize {
        VerifyBatchCols::<F>::width()
    }
}

impl<F: PrimeField32> FriReducedOpeningChip<F> {
    fn record_to_rows(
        record: FriReducedOpeningRecord<F>,
        aux_cols_factory: &MemoryAuxColsFactory<F>,
        slice: &mut [F],
        memory: &OfflineMemory<F>,
    ) {
        let width = VerifyBatchCols::<F>::width();

        let Instruction {
            a: a_ptr_ptr,
            b: b_ptr_ptr,
            c: result_ptr,
            d: addr_space,
            e: length_ptr,
            f: alpha_ptr,
            g: alpha_pow_ptr,
            ..
        } = record.instruction;

        let length_read = memory.record_by_id(record.length_read);
        let alpha_read = memory.record_by_id(record.alpha_read);
        let a_ptr_read = memory.record_by_id(record.a_ptr_read);
        let b_ptr_read = memory.record_by_id(record.b_ptr_read);

        let length = length_read.data[0].as_canonical_u32() as usize;
        let alpha: [F; EXT_DEG] = array::from_fn(|i| alpha_read.data[i]);
        let a_ptr = a_ptr_read.data[0];
        let b_ptr = b_ptr_read.data[0];

        let mut alpha_pow_current = record.alpha_pow_original;
        let mut current = [F::ZERO; EXT_DEG];

        let alpha_aux = aux_cols_factory.make_read_aux_cols(alpha_read);
        let length_aux = aux_cols_factory.make_read_aux_cols(length_read);
        let a_ptr_aux = aux_cols_factory.make_read_aux_cols(a_ptr_read);
        let b_ptr_aux = aux_cols_factory.make_read_aux_cols(b_ptr_read);

        let alpha_pow_aux = aux_cols_factory
            .make_write_aux_cols::<EXT_DEG>(memory.record_by_id(record.alpha_pow_write))
            .get_base();
        let result_aux =
            aux_cols_factory.make_write_aux_cols(memory.record_by_id(record.result_write));

        for i in 0..length {
            let a_read = memory.record_by_id(record.a_reads[i]);
            let b_read = memory.record_by_id(record.b_reads[i]);
            let a = a_read.data[0];
            let b: [F; EXT_DEG] = array::from_fn(|i| b_read.data[i]);
            current = FieldExtension::add(
                current,
                FieldExtension::multiply(
                    FieldExtension::subtract(b, elem_to_ext(a)),
                    alpha_pow_current,
                ),
            );

            let mut idx_is_zero = F::ZERO;
            let mut is_zero_aux = F::ZERO;

            let idx = F::from_canonical_usize(i);
            IsZeroSubAir.generate_subrow(idx, (&mut is_zero_aux, &mut idx_is_zero));

            let cols: &mut VerifyBatchCols<F> = slice[i * width..(i + 1) * width].borrow_mut();
            *cols = VerifyBatchCols {
                enabled: F::ONE,
                pc: record.pc,
                a_ptr_ptr,
                b_ptr_ptr,
                result_ptr,
                addr_space,
                length_ptr,
                alpha_ptr,
                alpha_pow_ptr,
                start_timestamp: record.start_timestamp,
                a_ptr_aux,
                b_ptr_aux,
                a_aux: aux_cols_factory.make_read_aux_cols(a_read),
                b_aux: aux_cols_factory.make_read_aux_cols(b_read),
                alpha_aux,
                length_aux,
                alpha_pow_aux,
                result_aux,
                a_ptr,
                b_ptr,
                a,
                b,
                alpha,
                alpha_pow_original: record.alpha_pow_original,
                alpha_pow_current,
                idx,
                idx_is_zero,
                is_zero_aux,
                current,
            };

            alpha_pow_current = FieldExtension::multiply(alpha, alpha_pow_current);
        }
    }

    fn generate_trace(self) -> RowMajorMatrix<F> {
        let width = self.trace_width();
        let height = next_power_of_two_or_zero(self.height);
        let mut flat_trace = F::zero_vec(width * height);

        let memory = self.offline_memory.lock().unwrap();

        let aux_cols_factory = memory.aux_cols_factory();

        let mut idx = 0;
        for record in self.records {
            let length = record.a_reads.len();
            Self::record_to_rows(
                record,
                &aux_cols_factory,
                &mut flat_trace[idx..idx + (length * width)],
                &memory,
            );
            idx += length * width;
        }
        // In padding rows, need idx_is_zero = 1 so IsZero constraints pass, and also because next.idx_is_zero is used
        // to determine the last row per instruction, so the last non-padding row needs next.idx_is_zero = 1
        flat_trace[self.height * width..]
            .par_chunks_mut(width)
            .for_each(|row| {
                let row: &mut VerifyBatchCols<F> = row.borrow_mut();
                row.idx_is_zero = F::ONE;
            });

        RowMajorMatrix::new(flat_trace, width)
    }
}

impl<SC: StarkGenericConfig> Chip<SC> for FriReducedOpeningChip<Val<SC>>
where
    Val<SC>: PrimeField32,
{
    fn air(&self) -> Arc<dyn AnyRap<SC>> {
        Arc::new(self.air)
    }
    fn generate_air_proof_input(self) -> AirProofInput<SC> {
        AirProofInput::simple_no_pis(self.air(), self.generate_trace())
    }
}
