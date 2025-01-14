use std::{borrow::Borrow, sync::Arc};

use openvm_circuit::{
    arch::{ExecutionBridge, ExecutionState},
    system::memory::{offline_checker::MemoryBridge, MemoryAddress},
};
use openvm_circuit_primitives::utils::not;
use openvm_native_compiler::VerifyBatchOpcode::VERIFY_BATCH;
use openvm_poseidon2_air::{Poseidon2SubAir, BABY_BEAR_POSEIDON2_HALF_FULL_ROUNDS};
use openvm_stark_backend::{
    interaction::{InteractionBuilder, InteractionType},
    p3_air::{Air, AirBuilder, BaseAir},
    p3_field::{Field, FieldAlgebra},
    p3_matrix::Matrix,
    rap::{BaseAirWithPublicValues, PartitionedBaseAir},
};
use openvm_stark_backend::air_builders::sub::SubAirBuilder;
use crate::verify_batch::{columns::VerifyBatchCols, CHUNK};

#[derive(Clone, Debug)]
pub struct VerifyBatchAir<F: Field, const SBOX_REGISTERS: usize> {
    pub execution_bridge: ExecutionBridge,
    pub memory_bridge: MemoryBridge,
    pub internal_bus: VerifyBatchBus,
    pub(crate) subair: Arc<Poseidon2SubAir<F, SBOX_REGISTERS>>,
    pub(super) offset: usize,
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
            start_top_level,
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
            ..
        } = local;

        let left_input = std::array::from_fn::<_, CHUNK, _>(|i| local.inner.inputs[i]);
        let right_input = std::array::from_fn::<_, CHUNK, _>(|i| local.inner.inputs[i + CHUNK]);
        let left_output = std::array::from_fn::<_, CHUNK, _>(|i| {
            local.inner.ending_full_rounds[BABY_BEAR_POSEIDON2_HALF_FULL_ROUNDS - 1].post[i]
        });
        let right_output = std::array::from_fn::<_, CHUNK, _>(|i| {
            local.inner.ending_full_rounds[BABY_BEAR_POSEIDON2_HALF_FULL_ROUNDS - 1].post[i + CHUNK]
        });
        let next_left_input = std::array::from_fn::<_, CHUNK, _>(|i| next.inner.inputs[i]);
        let next_right_input = std::array::from_fn::<_, CHUNK, _>(|i| next.inner.inputs[i + CHUNK]);

        builder.assert_bool(incorporate_row);
        builder.assert_bool(incorporate_sibling);
        builder.assert_bool(inside_row);
        let enabled = incorporate_row + incorporate_sibling + inside_row;
        builder.assert_bool(enabled.clone());
        builder.assert_bool(end_inside_row);
        builder.when(end_inside_row).assert_one(inside_row);
        builder.assert_bool(end_top_level);
        builder.when(end_top_level).assert_one(incorporate_sibling);

        let end = end_inside_row + end_top_level + (AB::Expr::ONE - enabled.clone());
        builder
            .when(end.clone())
            .when(next.incorporate_row)
            .assert_one(next.start_top_level);

        let mut sub_builder =
            SubAirBuilder::<AB, Poseidon2SubAir<AB::F, SBOX_REGISTERS>, AB::F>::new(
                builder,
                0..self.subair.width(),
            );
        self.subair.eval(&mut sub_builder);

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
            address_space,
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
                .assert_eq(cell.row_pointer + AB::F::ONE, cell.row_end);
        }

        //// top level constraints

        builder
            .when(end.clone())
            .when(next.incorporate_row + next.incorporate_sibling)
            .assert_eq(next.proof_index, AB::F::ZERO);

        let timestamp_after_end_operations = start_timestamp + AB::F::from_canonical_usize(5 + 1);

        builder
            .when(end.clone())
            .when(next.incorporate_row)
            .assert_eq(next.initial_opened_index, AB::F::ZERO);
        self.execution_bridge
            .execute_and_increment_pc(
                AB::Expr::from_canonical_usize(VERIFY_BATCH as usize + self.offset),
                [
                    dim_register,
                    opened_register,
                    sibling_register,
                    index_register,
                    commit_register,
                    address_space,
                ],
                ExecutionState::new(pc, very_first_timestamp),
                end_timestamp - very_first_timestamp,
            )
            .eval(builder, end_top_level);

        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, dim_register),
                [dim_base_pointer],
                very_first_timestamp,
                &dim_base_pointer_read,
            )
            .eval(builder, end_top_level);
        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, opened_register),
                [opened_base_pointer, opened_length],
                very_first_timestamp + AB::F::ONE,
                &opened_base_pointer_and_length_read,
            )
            .eval(builder, end_top_level);
        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, sibling_register),
                [sibling_base_pointer],
                very_first_timestamp + AB::F::TWO,
                &sibling_base_pointer_read,
            )
            .eval(builder, end_top_level);
        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, index_register),
                [index_base_pointer],
                very_first_timestamp + AB::F::from_canonical_usize(3),
                &index_base_pointer_read,
            )
            .eval(builder, end_top_level);
        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, commit_register),
                [commit_pointer],
                very_first_timestamp + AB::F::from_canonical_usize(4),
                &commit_pointer_read,
            )
            .eval(builder, end_top_level);

        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, commit_pointer),
                left_output,
                very_first_timestamp + AB::F::from_canonical_usize(5),
                &commit_read,
            )
            .eval(builder, end_top_level);

        let mut when_top_level_not_end =
            builder.when(incorporate_row + incorporate_sibling - end_top_level);

        when_top_level_not_end.assert_eq(next.dim_base_pointer, dim_base_pointer);
        when_top_level_not_end.assert_eq(next.opened_base_pointer, opened_base_pointer);
        when_top_level_not_end.assert_eq(next.sibling_base_pointer, sibling_base_pointer);
        when_top_level_not_end.assert_eq(next.index_base_pointer, index_base_pointer);
        when_top_level_not_end.assert_eq(next.start_timestamp, end_timestamp);
        when_top_level_not_end.assert_eq(next.opened_length, opened_length);
        when_top_level_not_end.assert_eq(next.address_space, address_space);
        when_top_level_not_end
            .assert_eq(next.initial_opened_index, final_opened_index + AB::F::ONE);

        builder
            .when(incorporate_sibling - end_top_level)
            .assert_eq(next.height * AB::F::TWO, height);
        builder
            .when(incorporate_row - end_top_level)
            .assert_eq(next.height, height);
        builder
            .when(incorporate_sibling - end_top_level)
            .assert_eq(next.proof_index, proof_index + AB::F::ONE);
        builder
            .when(incorporate_row - end_top_level)
            .assert_eq(next.proof_index, proof_index);

        builder
            .when(end_top_level)
            .when(incorporate_row)
            .assert_eq(height, AB::F::ONE);
        builder
            .when(end_top_level)
            .when(incorporate_sibling)
            .assert_eq(height, AB::F::TWO);

        // incorporate row

        builder
            .when(incorporate_row - end_top_level)
            .assert_one(next.incorporate_sibling);

        let row_hash = std::array::from_fn(|i| {
            (start_top_level * left_output[i])
                + ((AB::Expr::ONE - start_top_level) * right_input[i])
        });

        self.internal_bus.interact(
            builder,
            true,
            incorporate_row,
            timestamp_after_end_operations.clone(),
            end_timestamp - AB::F::TWO,
            opened_base_pointer,
            initial_opened_index,
            final_opened_index,
            address_space,
            row_hash,
        );

        for i in 0..CHUNK {
            builder
                .when(AB::Expr::ONE - end.clone())
                .when(next.incorporate_row)
                .assert_eq(next_left_input[i], left_output[i]);
        }

        builder
            .when(end_top_level)
            .when(incorporate_row)
            .assert_eq(final_opened_index, opened_length - AB::F::ONE);

        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, dim_base_pointer + initial_opened_index),
                [height],
                end_timestamp - AB::F::TWO,
                &read_initial_height_or_root_is_on_right,
            )
            .eval(builder, incorporate_row);
        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, dim_base_pointer + final_opened_index),
                [height],
                end_timestamp - AB::F::ONE,
                &read_initial_height_or_root_is_on_right,
            )
            .eval(builder, incorporate_row);

        // incorporate sibling

        builder
            .when(incorporate_sibling - end_top_level)
            .assert_one(next.incorporate_row + next.incorporate_sibling);
        builder
            .when(end_top_level)
            .when(incorporate_sibling)
            .assert_eq(initial_opened_index, opened_length);

        builder
            .when(incorporate_sibling)
            .assert_eq(final_opened_index + AB::F::ONE, initial_opened_index);

        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, index_base_pointer + proof_index),
                [root_is_on_right],
                timestamp_after_end_operations.clone(),
                &read_initial_height_or_root_is_on_right,
            )
            .eval(builder, incorporate_row);
        self.memory_bridge
            .read(
                MemoryAddress::new(address_space, sibling_base_pointer + proof_index),
                [sibling_array_start],
                timestamp_after_end_operations.clone() + AB::F::ONE,
                &read_final_height_or_sibling_array_start,
            )
            .eval(builder, incorporate_row);

        for i in 0..CHUNK {
            builder
                .when(next.incorporate_sibling)
                .when(next.root_is_on_right)
                .assert_eq(next_right_input[i], left_output[i]);
            builder
                .when(next.incorporate_sibling)
                .when(AB::Expr::ONE - next.root_is_on_right)
                .assert_eq(next_left_input[i], left_output[i]);

            self.memory_bridge
                .read(
                    MemoryAddress::new(address_space, sibling_array_start + proof_index),
                    [(root_is_on_right * left_input[i])
                        + ((AB::Expr::ONE - root_is_on_right) * right_input[i])],
                    timestamp_after_end_operations.clone() + AB::F::from_canonical_usize(2 + i),
                    &read_initial_height_or_root_is_on_right,
                )
                .eval(builder, incorporate_row);
        }

        builder.assert_eq(
            end_timestamp,
            timestamp_after_end_operations + AB::F::from_canonical_usize(2 + CHUNK),
        );
    }
}

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
        address_space: impl Into<AB::Expr>,
        hash: [impl Into<AB::Expr>; CHUNK],
    ) {
        let mut fields = vec![
            start_timestamp.into(),
            end_timestamp.into(),
            opened_base_pointer.into(),
            initial_opened_index.into(),
            final_opened_index.into(),
            address_space.into(),
        ];
        fields.extend(hash.into_iter().map(Into::into));
        builder.push_interaction(
            self.0,
            fields,
            multiplicity.into(),
            if send {
                InteractionType::Send
            } else {
                InteractionType::Receive
            },
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VerifyBatchBus(pub usize);
