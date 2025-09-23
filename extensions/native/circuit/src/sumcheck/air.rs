use std::{array::from_fn, borrow::Borrow, sync::Arc};
use openvm_circuit::{
    arch::{ExecutionBridge, ExecutionState},
    system::memory::{offline_checker::MemoryBridge, MemoryAddress},
};
use openvm_circuit_primitives::utils::{assert_array_eq, not};
use openvm_instructions::LocalOpcode;
use openvm_native_compiler::SumcheckOpcode::SUMCHECK_LAYER_EVAL;
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
    pub address_space: F,
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

        // Header
        let header_row_specific: &HeaderSpecificCols<AB::Var> =
            specific[..HeaderSpecificCols::<AB::Var>::width()].borrow();
        let registers = header_row_specific.registers;

        self.execution_bridge
            .execute_and_increment_pc(
                AB::Expr::from_canonical_usize(SUMCHECK_LAYER_EVAL.global_opcode().as_usize()),
                [
                    registers[4].into(),
                    registers[0].into(),
                    registers[1].into(),
                    self.address_space.into(),
                    self.address_space.into(),
                    registers[2].into(),
                    registers[3].into(),
                ],
                ExecutionState::new(header_row_specific.pc, first_timestamp),
                last_timestamp - first_timestamp,
            )
            .eval(builder, header_row);

        // Read registers
        // _debug
        // for i in 0..5usize {
        for i in 0..1usize {
            self.memory_bridge
                .read(
                    MemoryAddress::new(self.address_space, registers[i]),
                    [register_ptrs[i]],
                    first_timestamp + AB::F::from_canonical_usize(i),
                    &header_row_specific.read_records[i],
                )
                .eval(builder, header_row);
        }

        /* 
        // React ctx
        self.memory_bridge
            .read(
                MemoryAddress::new(self.address_space, register_ptrs[0]),
                ctx,
                first_timestamp + AB::F::from_canonical_usize(5),
                &header_row_specific.read_records[5],
            )
            .eval(builder, header_row);

        // Read challenges
        self.memory_bridge
            .read(
                MemoryAddress::new(self.address_space, register_ptrs[1]),
                challenges,
                first_timestamp + AB::F::from_canonical_usize(6),
                &header_row_specific.read_records[6],
            )
            .eval(builder, header_row);
        */

        /* _debug
        // Separate aggregate column clusters
        let alpha1: [_; EXT_DEG] = challenges[0..EXT_DEG].try_into().expect("");
        let c1: [_; EXT_DEG] = challenges[EXT_DEG..{EXT_DEG * 2}].try_into().expect("");
        let c2: [_; EXT_DEG] = challenges[{EXT_DEG * 2}..{EXT_DEG * 3}].try_into().expect("");
        let alpha2: [_; EXT_DEG] = challenges[{EXT_DEG * 3}..{EXT_DEG * 4}].try_into().expect("");

        // Carry along columns
        assert_array_eq(&mut builder.when(next.prod_row + next.logup_row), register_ptrs, next.register_ptrs);
        assert_array_eq(&mut builder.when(next.prod_row + next.logup_row), ctx, next.ctx);
        assert_array_eq::<_, _, _, {EXT_DEG * 2}>(
            &mut builder.when(next.prod_row + next.logup_row), 
            challenges[EXT_DEG..(EXT_DEG * 3)].try_into().expect(""), 
            next.challenges[EXT_DEG..(EXT_DEG * 3)].try_into().expect("")
        );
        assert_array_eq(&mut builder.when(next.prod_row + next.logup_row), register_ptrs, next.register_ptrs);

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





        // Prod spec evaluation
        let prod_row_specific: &ProdSpecificCols<AB::Var> =
            specific[..ProdSpecificCols::<AB::Var>::width()].borrow();

        self.memory_bridge
            .read(
                MemoryAddress::new(self.address_space, register_ptrs[0] + AB::F::from_canonical_usize(EXT_DEG * 2 - 1) + curr_prod_n),
                [max_round],
                start_timestamp,
                &prod_row_specific.read_records[0],
            )
            .eval(builder, prod_row);

        self.memory_bridge
            .read(
                MemoryAddress::new(
                    self.address_space,
                    register_ptrs[2] + (ctx[4] * ctx[3] * (curr_prod_n - AB::F::ONE) + ctx[4] * ctx[0]) * AB::F::from_canonical_usize(EXT_DEG),
                ),
                prod_row_specific.p,
                start_timestamp + AB::F::ONE,
                &prod_row_specific.read_records[1],
            )
            .eval(builder, prod_row);

        let p1: [_; EXT_DEG] = prod_row_specific.p[0..EXT_DEG].try_into().expect("");
        let p2: [_; EXT_DEG] = prod_row_specific.p[EXT_DEG..(EXT_DEG * 2)].try_into().expect("");

        self.memory_bridge
            .write(
                MemoryAddress::new(
                    self.address_space,
                    register_ptrs[4] + curr_prod_n * AB::F::from_canonical_usize(EXT_DEG),
                ),
                prod_row_specific.p_evals,
                start_timestamp + AB::F::TWO,
                &prod_row_specific.write_record,
            )
            .eval(builder, prod_row);

        // Logup spec evaluation
        let logup_row_specific: &LogupSpecificCols<AB::Var> =
            specific[..LogupSpecificCols::<AB::Var>::width()].borrow();

        self.memory_bridge
            .read(
                MemoryAddress::new(self.address_space, register_ptrs[0] + ctx[1] + AB::F::from_canonical_usize(EXT_DEG * 2 - 1) + curr_logup_n),
                [max_round],
                start_timestamp,
                &prod_row_specific.read_records[0],
            )
            .eval(builder, prod_row);

        self.memory_bridge
            .read(
                MemoryAddress::new(
                    self.address_space,
                    register_ptrs[2] + (ctx[4] * ctx[3] * (curr_prod_n - AB::F::ONE) + ctx[4] * ctx[0]) * AB::F::from_canonical_usize(EXT_DEG),
                ),
                prod_row_specific.p,
                start_timestamp + AB::F::ONE,
                &prod_row_specific.read_records[1],
            )
            .eval(builder, prod_row);

        let p1: [_; EXT_DEG] = logup_row_specific.pq[0..EXT_DEG].try_into().expect("");
        let p2: [_; EXT_DEG] = logup_row_specific.pq[EXT_DEG..(EXT_DEG * 2)].try_into().expect("");
        let q1: [_; EXT_DEG] = logup_row_specific.pq[(EXT_DEG * 2)..{EXT_DEG * 3}].try_into().expect("");
        let q2: [_; EXT_DEG] = logup_row_specific.pq[(EXT_DEG * 3)..(EXT_DEG * 4)].try_into().expect("");

        self.memory_bridge
            .write(
                MemoryAddress::new(
                    self.address_space,
                    register_ptrs[4] + curr_prod_n * AB::F::from_canonical_usize(EXT_DEG),
                ),
                prod_row_specific.p_evals,
                start_timestamp + AB::F::TWO,
                &prod_row_specific.write_record,
            )
            .eval(builder, prod_row);

        // Termination condition

        */
    }
}