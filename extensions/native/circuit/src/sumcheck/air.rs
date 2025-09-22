use std::{array::from_fn, borrow::Borrow, sync::Arc};
use openvm_circuit::{
    arch::{ExecutionBridge, ExecutionState},
    system::memory::{offline_checker::MemoryBridge, MemoryAddress, CHUNK},
};
use openvm_circuit_primitives::utils::{assert_array_eq, not};
use openvm_stark_backend::{
    air_builders::sub::SubAirBuilder,
    interaction::{BusIndex, InteractionBuilder, PermutationCheckBus},
    p3_air::{Air, AirBuilder, BaseAir},
    p3_field::{Field, FieldAlgebra},
    p3_matrix::Matrix,
    rap::{BaseAirWithPublicValues, PartitionedBaseAir},
};

use crate::{sumcheck::columns::{HeaderSpecificCols, LogupSpecificCols, NativeSumcheckCols, ProdSpecificCols}, EXT_DEG};

#[derive(Clone, Debug)]
pub struct NativeSumcheckAir<F: Field> {
    pub execution_bridge: ExecutionBridge,
    pub memory_bridge: MemoryBridge,
    pub(crate) address_space: F,
}

impl<F: Field> BaseAir<F> for NativeSumcheckAir<F> {
    fn width(&self) -> usize {
        NativeSumcheckCols::<F>::width()
    }
}

impl<F: Field> BaseAirWithPublicValues<F>
    for NativeSumcheckAir<F>
{
}

impl<F: Field> PartitionedBaseAir<F>
    for NativeSumcheckAir<F>
{
}

impl<AB: InteractionBuilder> Air<AB>
    for NativeSumcheckAir<AB::F>
{
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local: &NativeSumcheckCols<AB::Var> = (*local).borrow();
        let next = main.row_slice(1);
        let next: &NativeSumcheckCols<AB::Var> = (*next).borrow();

        let &NativeSumcheckCols {
            header_row,
            prod_row,
            logup_row,
            first_timestamp,
            start_timestamp,
            last_timestamp,
            register_ptrs,
            ctx,
            curr_prod_n,
            curr_logup_n,
            alpha,
            challenges,
            max_round,
            should_acc,
            eval_acc,
            specific,
        } = local;

        builder.assert_bool(header_row);
        builder.assert_bool(prod_row);
        builder.assert_bool(logup_row);
        let enabled = header_row + prod_row + logup_row;
        builder.assert_bool(enabled.clone());

        // Carry along columns
        assert_array_eq(&mut builder.when(next.prod_row + next.logup_row), register_ptrs, next.register_ptrs);
        assert_array_eq(&mut builder.when(next.prod_row + next.logup_row), ctx, next.ctx);
        assert_array_eq::<_, _, _, {EXT_DEG * 2}>(
            &mut builder.when(next.prod_row + next.logup_row), 
            challenges[EXT_DEG..(EXT_DEG * 3)].try_into().expect(""), 
            next.challenges[EXT_DEG..(EXT_DEG * 3)].try_into().expect("")
        );

        // Row transitions
        builder
            .when(header_row)
            .when(next.logup_row)
            .assert_zero(ctx[1]);
        builder
            .when(next.prod_row)
            .assert_eq(curr_prod_n + AB::F::ONE, next.curr_prod_n);
        builder
            .when(next.logup_row)
            .assert_eq(curr_logup_n + AB::F::ONE, next.curr_logup_n);
        builder
            .when(prod_row)
            .when(next.logup_row)
            .assert_eq(ctx[1], curr_prod_n);
        builder
            .when(logup_row)
            .when(not(next.logup_row))
            .assert_eq(ctx[2], curr_logup_n);

        let header_row_specific: &HeaderSpecificCols<AB::Var> =
            specific[..HeaderSpecificCols::<AB::Var>::width()].borrow();
        let prod_row_specific: &ProdSpecificCols<AB::Var> =
            specific[..ProdSpecificCols::<AB::Var>::width()].borrow();
        let logup_row_specific: &LogupSpecificCols<AB::Var> =
            specific[..LogupSpecificCols::<AB::Var>::width()].borrow();


    }
}