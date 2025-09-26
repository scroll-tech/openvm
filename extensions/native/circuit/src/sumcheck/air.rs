use std::{array::from_fn, borrow::Borrow, sync::Arc};
use openvm_circuit::{
    arch::{ContinuationVmProof, ExecutionBridge, ExecutionState},
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
use crate::{sumcheck::columns::{HeaderSpecificCols, LogupSpecificCols, NativeSumcheckCols, ProdSpecificCols}, FieldExtension, EXT_DEG};

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
            // Row indicators
            header_row,
            prod_row,
            logup_row,

            // Whether valid prod/logup row operations follow this row
            header_continuation,
            prod_continuation,
            logup_continuation,

            // Round limit
            prod_row_within_max_round,
            logup_row_within_max_round,

            // What type of evaluation is performed
            prod_in_round_evaluation,
            prod_next_round_evaluation,
            logup_in_round_evaluation,
            logup_next_round_evaluation,

            // Indicates whether the round evaluations should be added to the accumulator
            prod_acc,
            logup_acc,

            // Timestamps
            first_timestamp,
            start_timestamp,
            last_timestamp,

            // Results from reading registers
            register_ptrs,
            ctx,
            prod_nested_len,
            logup_nested_len,

            // Challenges
            alpha,
            challenges,

            curr_prod_n,
            curr_logup_n,
            
            max_round,
            within_round_limit,
            should_acc,
            eval_acc,
            specific,
        } = local;

        builder.assert_bool(header_row);
        builder.assert_bool(prod_row);
        builder.assert_bool(logup_row);
        builder.assert_bool(header_continuation);
        builder.assert_bool(prod_continuation);
        builder.assert_bool(logup_continuation);
        builder.assert_bool(prod_row_within_max_round);
        builder.assert_bool(logup_row_within_max_round);
        builder.assert_bool(prod_in_round_evaluation);
        builder.assert_bool(logup_in_round_evaluation);
        let enabled = header_row + prod_row + logup_row;
        builder.assert_bool(enabled.clone());
        let in_round = ctx[7];
        let continuation = header_continuation + prod_continuation + logup_continuation;
        builder.assert_bool(continuation.clone());

        // Randomness transition
        let alpha1: [_; EXT_DEG] = challenges[0..EXT_DEG].try_into().expect("");
        let c1: [_; EXT_DEG] = challenges[EXT_DEG..{EXT_DEG * 2}].try_into().expect("");
        let c2: [_; EXT_DEG] = challenges[{EXT_DEG * 2}..{EXT_DEG * 3}].try_into().expect("");
        let alpha2: [_; EXT_DEG] = challenges[{EXT_DEG * 3}..{EXT_DEG * 4}].try_into().expect("");
        let next_alpha1: [_; EXT_DEG] = next.challenges[0..EXT_DEG].try_into().expect("");

        // Carry along columns
        assert_array_eq(&mut builder.when(next.prod_row + next.logup_row), register_ptrs, next.register_ptrs);
        assert_array_eq(&mut builder.when(next.prod_row + next.logup_row), ctx, next.ctx);
        assert_array_eq::<_, _, _, {EXT_DEG * 2}>(
            &mut builder.when(next.prod_row + next.logup_row), 
            challenges[EXT_DEG..(EXT_DEG * 3)].try_into().expect(""), 
            next.challenges[EXT_DEG..(EXT_DEG * 3)].try_into().expect("")
        );
        builder.when(next.prod_row + next.logup_row).assert_eq(prod_nested_len, next.prod_nested_len);
        builder.when(next.prod_row + next.logup_row).assert_eq(logup_nested_len, next.logup_nested_len);
        
        // Row transition
        builder
            .when(next.prod_row)
            .assert_eq(curr_prod_n + AB::F::ONE, next.curr_prod_n);
        builder
            .when(next.logup_row)
            .assert_eq(curr_logup_n + AB::F::ONE, next.curr_logup_n);
        builder
            .when(header_row)
            .when(next.logup_row)
            .assert_zero(ctx[1]);
        builder
            .when(prod_row)
            .when(next.logup_row)
            .assert_eq(ctx[1], curr_prod_n);
        builder
            .when(prod_row)
            .when(not(prod_continuation))
            .assert_eq(ctx[1], curr_prod_n);
        builder
            .when(logup_row)
            .when(not(logup_continuation))
            .assert_eq(ctx[2], curr_logup_n);

        // Timestamp transition
        builder
            .when(header_row)
            .when(next.prod_row + next.logup_row)
            .assert_eq(next.start_timestamp, start_timestamp + AB::F::from_canonical_usize(7));
        builder
            .when(prod_row)
            .when(next.prod_row + next.logup_row)
            .assert_eq(next.start_timestamp, start_timestamp + AB::F::ONE + within_round_limit * AB::F::TWO);
        builder
            .when(logup_row)
            .when(next.prod_row + next.logup_row)
            .assert_eq(next.start_timestamp, start_timestamp + AB::F::ONE + within_round_limit * AB::F::from_canonical_usize(3));

        // Termination condition
        assert_array_eq(&mut builder.when::<AB::Expr>(not(continuation)), eval_acc, [AB::F::ZERO; 4]);

        // Randomness transition
        let alpha_denominator = FieldExtension::multiply(alpha1, alpha);
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(logup_row), alpha_denominator, alpha2);
        
        
        /* _debug
        let logup_next_alpha = FieldExtension::multiply(alpha2, alpha);
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(logup_continuation), logup_next_alpha, next_alpha1);
        let prod_next_alpha = FieldExtension::multiply(alpha1, alpha);
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(prod_continuation), prod_next_alpha, next_alpha1);
        */

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
        for i in 0..5usize {
            self.memory_bridge
                .read(
                    MemoryAddress::new(self.address_space, registers[i]),
                    [register_ptrs[i]],
                    first_timestamp + AB::F::from_canonical_usize(i),
                    &header_row_specific.read_records[i],
                )
                .eval(builder, header_row);
        }

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

        // Write final result
        self.memory_bridge
            .write(
                MemoryAddress::new(
                    self.address_space,
                    register_ptrs[4],
                ),
                eval_acc,
                last_timestamp - AB::F::ONE,
                &header_row_specific.write_records,
            )
            .eval(builder, header_row);

        // Prod spec evaluation
        let prod_row_specific: &ProdSpecificCols<AB::Var> =
            specific[..ProdSpecificCols::<AB::Var>::width()].borrow();
        let next_prod_row_specific: &ProdSpecificCols<AB::Var> =
            next.specific[..ProdSpecificCols::<AB::Var>::width()].borrow();

        self.memory_bridge
            .read(
                MemoryAddress::new(self.address_space, register_ptrs[0] + AB::F::from_canonical_usize(EXT_DEG * 2 - 1) + curr_prod_n),
                [max_round],
                start_timestamp,
                &prod_row_specific.read_records[0],
            )
            .eval(builder, prod_row);

        builder
            .when(prod_row_within_max_round)
            .assert_eq(prod_row_specific.data_ptr, (prod_nested_len * (curr_prod_n - AB::F::ONE) + ctx[4] * ctx[0]) * AB::F::from_canonical_usize(EXT_DEG));
        builder
            .assert_eq(prod_row * prod_row_within_max_round * in_round, prod_in_round_evaluation);
        builder
            .assert_eq(prod_row * prod_row_within_max_round * not(in_round), prod_next_round_evaluation);
        builder
            .assert_eq(prod_row * should_acc, prod_acc);

        self.memory_bridge
            .read(
                MemoryAddress::new(
                    self.address_space,
                    register_ptrs[2] + prod_row_specific.data_ptr,
                ),
                prod_row_specific.p,
                start_timestamp + AB::F::ONE,
                &prod_row_specific.read_records[1],
            )
            .eval(builder, prod_row_within_max_round);

        let p1: [AB::Var; EXT_DEG] = prod_row_specific.p[0..EXT_DEG].try_into().expect("");
        let p2: [AB::Var; EXT_DEG] = prod_row_specific.p[EXT_DEG..(EXT_DEG * 2)].try_into().expect("");

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
            .eval(builder, prod_row_within_max_round);

        // Calculate evaluations
        let next_round_p_evals = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(p1, c1),
            FieldExtension::multiply::<AB::Var, AB::Expr>(p2, c2),
        );
        let in_round_p_evals = FieldExtension::multiply::<AB::Var, AB::Expr>(p1, p2);
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(prod_in_round_evaluation), in_round_p_evals, prod_row_specific.p_evals);
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(prod_next_round_evaluation), next_round_p_evals, prod_row_specific.p_evals);

        // Accumulate evaluation
        let acc_eval = FieldExtension::multiply::<AB::Var, AB::Expr>(prod_row_specific.p_evals, alpha1);
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(prod_acc), prod_row_specific.acc_eval, acc_eval);

        let next_acc = FieldExtension::subtract(
            eval_acc, 
            next_prod_row_specific.acc_eval,
        );
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(next.prod_acc), next.eval_acc, next_acc);

        // Logup spec evaluation
        let logup_row_specific: &LogupSpecificCols<AB::Var> =
            specific[..LogupSpecificCols::<AB::Var>::width()].borrow();
        let next_logup_row_specfic: &LogupSpecificCols<AB::Var> =
            next.specific[..LogupSpecificCols::<AB::Var>::width()].borrow();

        self.memory_bridge
            .read(
                MemoryAddress::new(self.address_space, register_ptrs[0] + AB::F::from_canonical_usize(EXT_DEG * 2 - 1) + ctx[1] + curr_logup_n),
                [max_round],
                start_timestamp,
                &logup_row_specific.read_records[0],
            )
            .eval(builder, logup_row);

        builder
            .when(logup_row_within_max_round)
            .assert_eq(logup_row_specific.data_ptr, (logup_nested_len * (curr_logup_n - AB::F::ONE) + ctx[6] * ctx[0]) * AB::F::from_canonical_usize(EXT_DEG));
        builder
            .assert_eq(logup_row * logup_row_within_max_round * in_round, logup_in_round_evaluation);
        builder
            .assert_eq(logup_row * logup_row_within_max_round * not(in_round), logup_next_round_evaluation);
        builder
            .assert_eq(logup_row * should_acc, logup_acc);

        self.memory_bridge
            .read(
                MemoryAddress::new(
                    self.address_space,
                    register_ptrs[3] + logup_row_specific.data_ptr,
                ),
                logup_row_specific.pq,
                start_timestamp + AB::F::ONE,
                &logup_row_specific.read_records[1],
            )
            .eval(builder, logup_row_within_max_round);

        let p1: [_; EXT_DEG] = logup_row_specific.pq[0..EXT_DEG].try_into().expect("");
        let p2: [_; EXT_DEG] = logup_row_specific.pq[EXT_DEG..(EXT_DEG * 2)].try_into().expect("");
        let q1: [_; EXT_DEG] = logup_row_specific.pq[(EXT_DEG * 2)..{EXT_DEG * 3}].try_into().expect("");
        let q2: [_; EXT_DEG] = logup_row_specific.pq[(EXT_DEG * 3)..(EXT_DEG * 4)].try_into().expect("");
        
        self.memory_bridge
            .write(
                MemoryAddress::new(
                    self.address_space,
                    register_ptrs[4] + (ctx[1] + curr_logup_n) * AB::F::from_canonical_usize(EXT_DEG),
                ),
                logup_row_specific.p_evals,
                start_timestamp + AB::F::TWO,
                &logup_row_specific.write_records[0],
            )
            .eval(builder, logup_row_within_max_round);

        self.memory_bridge
            .write(
                MemoryAddress::new(
                    self.address_space,
                    register_ptrs[4] + (ctx[1] + ctx[2] + curr_logup_n) * AB::F::from_canonical_usize(EXT_DEG),
                ),
                logup_row_specific.q_evals,
                start_timestamp + AB::F::from_canonical_usize(3),
                &logup_row_specific.write_records[1],
            )
            .eval(builder, logup_row_within_max_round);

        // Calculate evaluations
        let next_round_p_evals = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(p1, c1),
            FieldExtension::multiply::<AB::Var, AB::Expr>(p2, c2),
        );
        let in_round_p_evals = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(p1, q2),
            FieldExtension::multiply::<AB::Var, AB::Expr>(p2, q1),
        );
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(logup_in_round_evaluation), in_round_p_evals, logup_row_specific.p_evals);
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(logup_next_round_evaluation), next_round_p_evals, logup_row_specific.p_evals);

        let next_round_q_evals = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(q1, c1),
            FieldExtension::multiply::<AB::Var, AB::Expr>(q2, c2),
        );
        let in_round_q_evals = FieldExtension::multiply::<AB::Var, AB::Expr>(q1, q2);
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(logup_in_round_evaluation), in_round_q_evals, logup_row_specific.q_evals);
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(logup_next_round_evaluation), next_round_q_evals, logup_row_specific.q_evals);
        
        // Accumulate evaluation
        let acc_eval = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(logup_row_specific.p_evals, alpha1),
            FieldExtension::multiply::<AB::Var, AB::Expr>(logup_row_specific.q_evals, alpha2),
        );
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(logup_acc), logup_row_specific.acc_eval, acc_eval);

        let next_acc = FieldExtension::subtract(
            eval_acc, 
            next_logup_row_specfic.acc_eval,
        );
        assert_array_eq::<_, _, _, EXT_DEG>(&mut builder.when(next.logup_acc), next.eval_acc, next_acc);
    }
}