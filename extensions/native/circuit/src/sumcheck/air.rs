use std::borrow::Borrow;

use openvm_circuit::{
    arch::{ExecutionBridge, ExecutionState},
    system::memory::{
        offline_checker::{HintBridge, MemoryBridge},
        MemoryAddress,
    },
};
use openvm_circuit_primitives::utils::{and, assert_array_eq, not};
use openvm_instructions::{LocalOpcode, NATIVE_AS};
use openvm_native_compiler::SumcheckOpcode::SUMCHECK_LAYER_EVAL;
use openvm_stark_backend::{
    interaction::InteractionBuilder,
    p3_air::{Air, AirBuilder, BaseAir},
    p3_field::{Field, FieldAlgebra},
    p3_matrix::Matrix,
    rap::{BaseAirWithPublicValues, PartitionedBaseAir},
};

use crate::{
    field_extension::{FieldExtension, EXT_DEG},
    sumcheck::{
        chip::CONTEXT_ARR_BASE_LEN,
        columns::{HeaderSpecificCols, LogupSpecificCols, NativeSumcheckCols, ProdSpecificCols},
    },
};

#[derive(Clone, Debug)]
pub struct NativeSumcheckAir {
    pub execution_bridge: ExecutionBridge,
    pub memory_bridge: MemoryBridge,
    pub hint_bridge: HintBridge,
}

impl NativeSumcheckAir {
    pub fn new(
        execution_bridge: ExecutionBridge,
        memory_bridge: MemoryBridge,
        hint_bridge: HintBridge,
    ) -> Self {
        Self {
            execution_bridge,
            memory_bridge,
            hint_bridge,
        }
    }
}

impl<F: Field> BaseAir<F> for NativeSumcheckAir {
    fn width(&self) -> usize {
        NativeSumcheckCols::<F>::width()
    }
}

impl<F: Field> BaseAirWithPublicValues<F> for NativeSumcheckAir {}

impl<F: Field> PartitionedBaseAir<F> for NativeSumcheckAir {}

impl<AB: InteractionBuilder> Air<AB> for NativeSumcheckAir {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        let local: &NativeSumcheckCols<AB::Var> = (*local).borrow();
        let next = main.row_slice(1);
        let next: &NativeSumcheckCols<AB::Var> = (*next).borrow();
        let native_as = AB::F::from_canonical_u32(NATIVE_AS);

        let &NativeSumcheckCols {
            // Row indicators
            header_row,
            prod_row,
            logup_row,
            is_end,

            prod_continued,
            logup_continued,
            // What type of evaluation is performed
            // mainly for reducing constraint degree
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
            is_writeback,
            prod_hint_id,
            logup_hint_id,
            specific,
        } = local;

        let [round, num_prod_spec, num_logup_spec, _prod_spec_inner_len, prod_spec_inner_inner_len, _logup_spec_inner_len, logup_spec_inner_inner_len, in_round] =
            ctx;
        builder.assert_bool(header_row);
        builder.assert_bool(prod_row);
        builder.assert_bool(logup_row);
        builder.assert_bool(within_round_limit);
        builder.assert_bool(is_writeback);
        builder.assert_bool(prod_in_round_evaluation);
        builder.assert_bool(logup_in_round_evaluation);

        let enabled = header_row + prod_row + logup_row;
        let next_enabled = next.header_row + next.prod_row + next.logup_row;
        builder.assert_bool(enabled.clone());

        builder
            .when_transition()
            .assert_eq(prod_row * next.prod_row, prod_continued);
        builder
            .when_transition()
            .assert_eq(logup_row * next.logup_row, logup_continued);
        // TODO: handle last row properly

        builder.when_transition().assert_eq::<AB::Expr, AB::Expr>(
            prod_row * next.header_row
                + logup_row * next.header_row
                + not::<AB::Expr>(next_enabled),
            is_end.into(),
        );

        // TODO: within_round_limit = true => round < max_round

        // Randomness transition
        let alpha1: [_; EXT_DEG] = challenges[0..EXT_DEG].try_into().unwrap();
        let c1: [_; EXT_DEG] = challenges[EXT_DEG..{ EXT_DEG * 2 }].try_into().unwrap();
        let c2: [_; EXT_DEG] = challenges[{ EXT_DEG * 2 }..{ EXT_DEG * 3 }]
            .try_into()
            .unwrap();
        let alpha2: [_; EXT_DEG] = challenges[{ EXT_DEG * 3 }..{ EXT_DEG * 4 }]
            .try_into()
            .unwrap();
        let next_alpha1: [_; EXT_DEG] = next.challenges[0..EXT_DEG].try_into().unwrap();

        // Carry along columns
        assert_array_eq(
            &mut builder.when(next.prod_row + next.logup_row),
            register_ptrs,
            next.register_ptrs,
        );
        assert_array_eq(
            &mut builder.when(next.prod_row + next.logup_row),
            ctx,
            next.ctx,
        );
        // c1, c2 remain the same
        assert_array_eq::<_, _, _, { EXT_DEG * 2 }>(
            &mut builder.when(next.prod_row + next.logup_row),
            challenges[EXT_DEG..(EXT_DEG * 3)].try_into().expect(""),
            next.challenges[EXT_DEG..(EXT_DEG * 3)]
                .try_into()
                .expect(""),
        );
        assert_array_eq(
            &mut builder.when(next.prod_row + next.logup_row),
            alpha,
            next.alpha,
        );
        builder
            .when(next.prod_row + next.logup_row)
            .assert_eq(max_round, next.max_round);
        builder
            .when(next.prod_row + next.logup_row)
            .assert_eq(prod_nested_len, next.prod_nested_len);
        builder
            .when(next.prod_row + next.logup_row)
            .assert_eq(logup_nested_len, next.logup_nested_len);
        builder
            .when(next.prod_row + next.logup_row)
            .assert_eq(is_writeback, next.is_writeback);
        builder
            .when(next.prod_row + next.logup_row)
            .assert_eq(prod_hint_id, next.prod_hint_id);
        builder
            .when(next.prod_row + next.logup_row)
            .assert_eq(logup_hint_id, next.logup_hint_id);

        ////////////////////////////////////////////////////////////////
        // Row transitions from current to next row
        // The basic pattern is
        //    header_row -> prod_row -> ... -> prod_row
        //         -> logup_row -> ... -> logup_row
        ////////////////////////////////////////////////////////////////

        // (curr_prod_n, curr_logup_n) start at 0
        builder.when(header_row).assert_zero(curr_prod_n);
        builder
            .when(header_row + prod_row)
            .assert_zero(curr_logup_n);
        builder
            .when(next.prod_row)
            .assert_eq(curr_prod_n + AB::F::ONE, next.curr_prod_n);
        builder
            .when(next.logup_row)
            .assert_eq(curr_logup_n + AB::F::ONE, next.curr_logup_n);
        // if header row is followed by another header row
        // then num_prod_spec and num_logup_spec should be zero
        builder
            .when(header_row)
            .when(next.header_row)
            .assert_zero(num_prod_spec);
        builder
            .when(header_row)
            .when(next.header_row)
            .assert_zero(num_logup_spec);
        // if header row is followed by a logup row,
        // then num_prod_spec should be zero
        builder
            .when(header_row)
            .when(next.logup_row)
            .assert_zero(num_prod_spec);
        builder
            .when(prod_row)
            .when(next.logup_row)
            .assert_eq(curr_prod_n, num_prod_spec);
        builder
            .when(logup_row)
            .when(next.header_row)
            .assert_eq(curr_logup_n, num_logup_spec);

        // Timestamp transition
        builder
            .when(header_row)
            .when(next.prod_row + next.logup_row)
            .assert_eq(
                next.start_timestamp,
                start_timestamp + AB::F::from_canonical_usize(8),
            );

        // _debug
        // // Prod row timestamp transition
        // // Equivalent to: when(prod_row): next_ts = ts + within_round_limit * (1 + is_writeback)
        // // Reformulated using prod_in_round_evaluation + prod_next_round_evaluation = prod_row * within_round_limit
        // // to stay within max constraint degree 3 while guarding against is_end boundary rows.
        // builder
        //     .when_transition()
        //     .when_ne(is_end, AB::Expr::ONE)
        //     .assert_eq(
        //         prod_row * (next.start_timestamp - start_timestamp),
        //         (prod_in_round_evaluation + prod_next_round_evaluation)
        //             * (AB::Expr::ONE + is_writeback),
        //     );

        // // Logup row timestamp transition
        // // Same reformulation using logup_in_round_evaluation + logup_next_round_evaluation = logup_row * within_round_limit
        // builder
        //     .when_transition()
        //     .when_ne(is_end, AB::Expr::ONE)
        //     .assert_eq(
        //         logup_row * (next.start_timestamp - start_timestamp),
        //         (logup_in_round_evaluation + logup_next_round_evaluation)
        //             * (AB::Expr::TWO + is_writeback),
        //     );

        // Termination condition
        assert_array_eq(
            &mut builder.when::<AB::Expr>(is_end.into()),
            eval_acc,
            [AB::F::ZERO; 4],
        );

        // Randomness transition
        assert_array_eq(
            &mut builder.when(and(header_row, next.prod_row + next.logup_row)),
            next.challenges[0..EXT_DEG].try_into().unwrap(),
            [AB::F::ONE, AB::F::ZERO, AB::F::ZERO, AB::F::ZERO],
        );
        assert_array_eq::<_, _, _, { EXT_DEG }>(&mut builder.when(header_row), alpha, alpha1);
        let prod_next_alpha = FieldExtension::multiply(alpha1, alpha);
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(prod_continued),
            prod_next_alpha,
            next_alpha1,
        );
        // alpha1 = alpha_numerator, alpha2 = alpha_denominator for logup row
        let alpha_denominator = FieldExtension::multiply(alpha1, alpha);
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(logup_row),
            alpha_denominator,
            alpha2,
        );
        let logup_next_alpha = FieldExtension::multiply(alpha2, alpha);
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(logup_continued),
            logup_next_alpha,
            next_alpha1,
        );

        ///////////////////////////////////////
        // Header
        ///////////////////////////////////////
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
                    header_row_specific.prod_evals_id.into(),
                    header_row_specific.logup_evals_id.into(),
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
                    MemoryAddress::new(native_as, registers[i]),
                    [register_ptrs[i]],
                    first_timestamp + AB::F::from_canonical_usize(i),
                    &header_row_specific.read_records[i],
                )
                .eval(builder, header_row);
        }

        // Read ctx
        self.memory_bridge
            .read(
                MemoryAddress::new(native_as, register_ptrs[0]),
                ctx,
                first_timestamp + AB::F::from_canonical_usize(5),
                &header_row_specific.read_records[5],
            )
            .eval(builder, header_row);

        // Read challenges
        self.memory_bridge
            .read(
                MemoryAddress::new(native_as, register_ptrs[1]),
                challenges,
                first_timestamp + AB::F::from_canonical_usize(6),
                &header_row_specific.read_records[6],
            )
            .eval(builder, header_row);

        // Read max_round
        self.memory_bridge
            .read(
                MemoryAddress::new(
                    native_as,
                    register_ptrs[0] + AB::F::from_canonical_usize(CONTEXT_ARR_BASE_LEN),
                ),
                [max_round, is_writeback],
                first_timestamp + AB::F::from_canonical_usize(7),
                &header_row_specific.read_records[7],
            )
            .eval(builder, header_row);

        // Write final result
        self.memory_bridge
            .write(
                MemoryAddress::new(native_as, register_ptrs[4]),
                eval_acc,
                last_timestamp - AB::F::ONE,
                &header_row_specific.write_records,
            )
            .eval(builder, header_row);

        ///////////////////////////////////////
        // Prod spec evaluation
        ///////////////////////////////////////
        let prod_row_specific: &ProdSpecificCols<AB::Var> =
            specific[..ProdSpecificCols::<AB::Var>::width()].borrow();
        let next_prod_row_specific: &ProdSpecificCols<AB::Var> =
            next.specific[..ProdSpecificCols::<AB::Var>::width()].borrow();

        // prod_row * within_round_limit =
        //    prod_in_round_evaluation + prod_next_round_evaluation
        builder
            .when(prod_in_round_evaluation + prod_next_round_evaluation)
            .assert_eq(
                prod_row_specific.data_ptr,
                (prod_nested_len * (curr_prod_n - AB::F::ONE) + prod_spec_inner_inner_len * round)
                    * AB::F::from_canonical_usize(EXT_DEG),
            );
        builder.assert_eq(
            prod_row * within_round_limit * in_round,
            prod_in_round_evaluation,
        );
        builder.assert_eq(
            prod_row * within_round_limit * not(in_round),
            prod_next_round_evaluation,
        );
        builder.assert_eq(prod_row * should_acc, prod_acc);

        // Obtain p1, p2 from hint space and write back to witness arrays
        self.memory_bridge
            .write(
                MemoryAddress::new(native_as, register_ptrs[2] + prod_row_specific.data_ptr),
                prod_row_specific.p,
                start_timestamp,
                &prod_row_specific.ps_record,
            )
            .eval(
                builder,
                (prod_in_round_evaluation + prod_next_round_evaluation) * is_writeback,
            );

        // Lookup each element of p in the hint bus to constrain hint_space reads
        let prod_enabled: AB::Expr = prod_in_round_evaluation + prod_next_round_evaluation;
        for (j, &val) in prod_row_specific.p.iter().enumerate() {
            self.hint_bridge.lookup(
                builder,
                prod_hint_id,
                prod_row_specific.data_ptr + AB::F::from_canonical_usize(j),
                val,
                prod_enabled.clone(),
            );
        }

        let p1: [AB::Var; EXT_DEG] = prod_row_specific.p[0..EXT_DEG].try_into().unwrap();
        let p2: [AB::Var; EXT_DEG] = prod_row_specific.p[EXT_DEG..(EXT_DEG * 2)]
            .try_into()
            .unwrap();

        self.memory_bridge
            .write(
                MemoryAddress::new(
                    native_as,
                    register_ptrs[4] + curr_prod_n * AB::F::from_canonical_usize(EXT_DEG),
                ),
                prod_row_specific.p_evals,
                start_timestamp + is_writeback * AB::F::ONE,
                &prod_row_specific.write_record,
            )
            .eval(builder, prod_row * within_round_limit);

        // Calculate evaluations
        let next_round_p_evals = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(p1, c1),
            FieldExtension::multiply::<AB::Var, AB::Expr>(p2, c2),
        );
        let in_round_p_evals = FieldExtension::multiply::<AB::Var, AB::Expr>(p1, p2);
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(prod_in_round_evaluation),
            in_round_p_evals,
            prod_row_specific.p_evals,
        );
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(prod_next_round_evaluation),
            next_round_p_evals,
            prod_row_specific.p_evals,
        );

        // TODO: add constraint on should_acc

        // Accumulate `eval_rlc` into global accumulator `eval_acc`
        // when round < max_round - 2
        let eval_rlc =
            FieldExtension::multiply::<AB::Var, AB::Expr>(prod_row_specific.p_evals, alpha1);
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(prod_acc),
            prod_row_specific.eval_rlc,
            eval_rlc,
        );
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(next.prod_acc),
            FieldExtension::add(next.eval_acc, next_prod_row_specific.eval_rlc),
            eval_acc,
        );

        ///////////////////////////////////////
        // Logup spec evaluation
        ///////////////////////////////////////
        let logup_row_specific: &LogupSpecificCols<AB::Var> =
            specific[..LogupSpecificCols::<AB::Var>::width()].borrow();
        let next_logup_row_specfic: &LogupSpecificCols<AB::Var> =
            next.specific[..LogupSpecificCols::<AB::Var>::width()].borrow();

        // logup_row * within_round_limit =
        //    logup_in_round_evaluation + logup_next_round_evaluation
        builder
            .when(logup_in_round_evaluation + logup_next_round_evaluation)
            .assert_eq(
                logup_row_specific.data_ptr,
                (logup_nested_len * (curr_logup_n - AB::F::ONE)
                    + logup_spec_inner_inner_len * round)
                    * AB::F::from_canonical_usize(EXT_DEG),
            );
        builder.assert_eq(
            logup_row * within_round_limit * in_round,
            logup_in_round_evaluation,
        );
        builder.assert_eq(
            logup_row * within_round_limit * not(in_round),
            logup_next_round_evaluation,
        );
        builder.assert_eq(logup_row * should_acc, logup_acc);

        // Obtain p1, p2, q1, q2 from hint space
        self.memory_bridge
            .write(
                MemoryAddress::new(native_as, register_ptrs[3] + logup_row_specific.data_ptr),
                logup_row_specific.pq,
                start_timestamp,
                &logup_row_specific.pqs_record,
            )
            .eval(
                builder,
                (logup_in_round_evaluation + logup_next_round_evaluation) * is_writeback,
            );

        // Lookup each element of pq in the hint bus to constrain hint_space reads
        let logup_enabled: AB::Expr = logup_in_round_evaluation + logup_next_round_evaluation;
        for (j, &val) in logup_row_specific.pq.iter().enumerate() {
            self.hint_bridge.lookup(
                builder,
                logup_hint_id,
                logup_row_specific.data_ptr + AB::F::from_canonical_usize(j),
                val,
                logup_enabled.clone(),
            );
        }
        let p1: [_; EXT_DEG] = logup_row_specific.pq[0..EXT_DEG].try_into().unwrap();
        let p2: [_; EXT_DEG] = logup_row_specific.pq[EXT_DEG..(EXT_DEG * 2)]
            .try_into()
            .unwrap();
        let q1: [_; EXT_DEG] = logup_row_specific.pq[(EXT_DEG * 2)..{ EXT_DEG * 3 }]
            .try_into()
            .unwrap();
        let q2: [_; EXT_DEG] = logup_row_specific.pq[(EXT_DEG * 3)..(EXT_DEG * 4)]
            .try_into()
            .unwrap();

        // write p_evals
        self.memory_bridge
            .write(
                MemoryAddress::new(
                    native_as,
                    register_ptrs[4]
                        + (num_prod_spec + curr_logup_n) * AB::F::from_canonical_usize(EXT_DEG),
                ),
                logup_row_specific.p_evals,
                start_timestamp + is_writeback * AB::F::ONE,
                &logup_row_specific.write_records[0],
            )
            .eval(builder, logup_row * within_round_limit);

        // write q_evals
        self.memory_bridge
            .write(
                MemoryAddress::new(
                    native_as,
                    register_ptrs[4]
                        + (num_prod_spec + num_logup_spec + curr_logup_n)
                            * AB::F::from_canonical_usize(EXT_DEG),
                ),
                logup_row_specific.q_evals,
                start_timestamp + is_writeback * AB::F::ONE + AB::F::ONE,
                &logup_row_specific.write_records[1],
            )
            .eval(builder, logup_row * within_round_limit);

        // Calculate evaluations
        let next_round_p_evals = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(p1, c1),
            FieldExtension::multiply::<AB::Var, AB::Expr>(p2, c2),
        );
        let in_round_p_evals = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(p1, q2),
            FieldExtension::multiply::<AB::Var, AB::Expr>(p2, q1),
        );
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(logup_in_round_evaluation),
            in_round_p_evals,
            logup_row_specific.p_evals,
        );
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(logup_next_round_evaluation),
            next_round_p_evals,
            logup_row_specific.p_evals,
        );

        let next_round_q_evals = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(q1, c1),
            FieldExtension::multiply::<AB::Var, AB::Expr>(q2, c2),
        );
        let in_round_q_evals = FieldExtension::multiply::<AB::Var, AB::Expr>(q1, q2);
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(logup_in_round_evaluation),
            in_round_q_evals,
            logup_row_specific.q_evals,
        );
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(logup_next_round_evaluation),
            next_round_q_evals,
            logup_row_specific.q_evals,
        );

        // Accumulate evaluation
        let eval_rlc = FieldExtension::add(
            FieldExtension::multiply::<AB::Var, AB::Expr>(logup_row_specific.p_evals, alpha1),
            FieldExtension::multiply::<AB::Var, AB::Expr>(logup_row_specific.q_evals, alpha2),
        );
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(logup_acc),
            logup_row_specific.eval_rlc,
            eval_rlc,
        );

        // Accumulate into global accumulator `eval_acc`
        // when round < max_round - 2
        assert_array_eq::<_, _, _, { EXT_DEG }>(
            &mut builder.when(next.logup_acc),
            FieldExtension::add(next.eval_acc, next_logup_row_specfic.eval_rlc),
            eval_acc,
        );
    }
}
