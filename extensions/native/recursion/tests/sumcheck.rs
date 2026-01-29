use std::iter::{once, repeat_n};

use openvm_circuit::{arch::instructions::program::Program, utils::air_test_impl};
#[cfg(feature = "cuda")]
use openvm_cuda_backend::engine::GpuBabyBearPoseidon2Engine;
use openvm_native_circuit::{NativeBuilder, NativeConfig, EXT_DEG};
use openvm_native_compiler::{
    asm::{AsmBuilder, AsmCompiler},
    conversion::{convert_program, CompilerOptions},
    ir::{Ext, Usize},
    prelude::*,
};
use openvm_stark_backend::p3_field::{
    extension::BinomialExtensionField, FieldAlgebra, FieldExtensionAlgebra,
};
#[cfg(not(feature = "cuda"))]
use openvm_stark_sdk::config::baby_bear_poseidon2::BabyBearPoseidon2Engine;
use openvm_stark_sdk::{
    config::{fri_params::standard_fri_params_with_100_bits_conjectured_security, FriParameters},
    p3_baby_bear::BabyBear,
};
use rand::{thread_rng, RngCore};

pub type F = BabyBear;
pub type E = BinomialExtensionField<F, EXT_DEG>;

#[test]
fn test_sumcheck_layer_eval() {
    let mut rng = thread_rng();
    let mut builder = AsmBuilder::<BabyBear, BinomialExtensionField<F, 4>>::default();

    let num_layers = 8;
    let num_prod_specs = 6;
    let num_logup_specs = 8;

    let prod_evals: Vec<E> = (0..(num_prod_specs * num_layers * 2))
        .into_iter()
        .map(|_| new_rand_ext(&mut rng))
        .collect();

    let logup_evals: Vec<E> = (0..(num_logup_specs * num_layers * 4))
        .into_iter()
        .map(|_| new_rand_ext(&mut rng))
        .collect();

    build_test_program(
        &mut builder,
        prod_evals.clone(),
        logup_evals.clone(),
        num_prod_specs,
        num_logup_specs,
        num_layers,
        3,
    );

    let compilation_options = CompilerOptions::default().with_cycle_tracker();
    let mut compiler = AsmCompiler::new(compilation_options.word_size);
    compiler.build(builder.operations);
    let asm_code = compiler.code();

    let program: Program<_> = convert_program(asm_code, compilation_options);
    let sumcheck_max_constraint_degree = 3;
    let fri_params = if matches!(std::env::var("OPENVM_FAST_TEST"), Ok(x) if &x == "1") {
        FriParameters {
            // max constraint degree = 2^log_blowup + 1
            log_blowup: 1,
            log_final_poly_len: 0,
            num_queries: 2,
            proof_of_work_bits: 0,
        }
    } else {
        standard_fri_params_with_100_bits_conjectured_security(1)
    };

    let mut input_stream: Vec<Vec<F>> = vec![];
    input_stream.push(
        prod_evals
            .into_iter()
            .flat_map(|e| <E as FieldExtensionAlgebra<F>>::as_base_slice(&e).to_vec())
            .collect(),
    );
    input_stream.push(
        logup_evals
            .into_iter()
            .flat_map(|e| <E as FieldExtensionAlgebra<F>>::as_base_slice(&e).to_vec())
            .collect(),
    );

    let mut config = NativeConfig::aggregation(0, sumcheck_max_constraint_degree);
    config.system.memory_config.max_access_adapter_n = 16;

    let vb = NativeBuilder::default();
    #[cfg(not(feature = "cuda"))]
    air_test_impl::<BabyBearPoseidon2Engine, _>(
        fri_params,
        vb,
        config,
        program,
        input_stream,
        1,
        true,
    )
    .unwrap();
    #[cfg(feature = "cuda")]
    {
        air_test_impl::<GpuBabyBearPoseidon2Engine, _>(
            fri_params,
            vb,
            config,
            program,
            input_stream,
            1,
            true,
        )
        .unwrap();
    }
}

fn new_rand_ext<R: RngCore>(rng: &mut R) -> E {
    E::from_base_slice(&[
        F::from_canonical_u32(rng.next_u32()),
        F::from_canonical_u32(rng.next_u32()),
        F::from_canonical_u32(rng.next_u32()),
        F::from_canonical_u32(rng.next_u32()),
    ])
}

fn build_test_program<C: Config>(
    builder: &mut Builder<C>,
    prod_evals: Vec<C::EF>,
    logup_evals: Vec<C::EF>,
    num_prod_specs: usize,
    num_logup_specs: usize,
    num_layers: usize,
    round: usize,
) {
    let mode = 1; // current_layer

    let mut ctx_u32s = vec![
        round,
        num_prod_specs,
        num_logup_specs,
        num_layers,
        2,
        num_layers,
        4,
        mode,
    ];
    ctx_u32s.extend(repeat_n(num_layers, num_prod_specs + num_logup_specs));

    let ctx: Array<C, Usize<C::N>> = builder.dyn_array(ctx_u32s.len());
    for (idx, n) in ctx_u32s.into_iter().enumerate() {
        builder.set(&ctx, idx, Usize::from(n));
    }

    #[rustfmt::skip]
    let challenges_u32s = [
        548478283u32, 456436544, 1716290291, 791326976,
        1829717553, 1422025771, 1917123958, 727015942,
        183548369, 591240150, 96141963, 1286249979,
        0, 0, 0, 0,
    ];
    let challenges: Array<C, Ext<C::F, C::EF>> = builder.dyn_array(challenges_u32s.len() / EXT_DEG);
    for (idx, n) in challenges_u32s.chunks(EXT_DEG).enumerate() {
        let e: Ext<C::F, C::EF> = builder.constant(C::EF::from_base_slice(&[
            C::F::from_canonical_u32(n[0]),
            C::F::from_canonical_u32(n[1]),
            C::F::from_canonical_u32(n[2]),
            C::F::from_canonical_u32(n[3]),
        ]));

        builder.set(&challenges, idx, e);
    }

    let num_prod_evals = num_prod_specs * num_layers * 2;
    let prod_spec_evals: Array<C, Ext<C::F, C::EF>> = builder.dyn_array(num_prod_evals);
    for idx in 0..num_prod_evals {
        let e: Ext<C::F, C::EF> = builder.constant(prod_evals[idx]);

        builder.set(&prod_spec_evals, idx, e);
    }

    let num_logup_evals = num_logup_specs * num_layers * 4;
    let logup_spec_evals: Array<C, Ext<C::F, C::EF>> = builder.dyn_array(num_logup_evals);
    for idx in 0..num_logup_evals {
        let e: Ext<C::F, C::EF> = builder.constant(logup_evals[idx]);

        builder.set(&logup_spec_evals, idx, e);
    }

    let alpha = builder.get(&challenges, 0);
    let c1 = builder.get(&challenges, 1);
    let c2 = builder.get(&challenges, 2);

    let alpha_acc: Ext<C::F, C::EF> = builder.constant(C::EF::ONE);
    let eval_acc: Ext<C::F, C::EF> = builder.constant(C::EF::ZERO);

    let mut p_evals = vec![];
    for i in 0..num_prod_specs {
        let start = num_layers * 2 * i + 2 * round;
        let p1 = builder.get(&prod_spec_evals, start);
        let p2 = builder.get(&prod_spec_evals, start + 1);
        let p_eval: Ext<C::F, C::EF> = if mode == 1 {
            // current layer
            builder.eval(p1 * p2)
        } else {
            // next layer
            builder.eval(p1 * c1 + p2 * c2)
        };
        p_evals.push(p_eval);
        let eval_rlc: Ext<C::F, C::EF> = builder.eval(alpha_acc * p_eval);
        builder.assign(&eval_acc, eval_acc + eval_rlc);
        builder.assign(&alpha_acc, alpha_acc * alpha);
    }

    let mut logup_p_evals = vec![];
    let mut logup_q_evals = vec![];
    for i in 0..num_logup_specs {
        let start = num_layers * 4 * i + 4 * round;
        let p1 = builder.get(&logup_spec_evals, start);
        let p2 = builder.get(&logup_spec_evals, start + 1);
        let q1 = builder.get(&logup_spec_evals, start + 2);
        let q2 = builder.get(&logup_spec_evals, start + 3);
        let p_eval: Ext<C::F, C::EF> = if mode == 1 {
            builder.eval(p1 * q2 + p2 * q1)
        } else {
            builder.eval(p1 * c1 + p2 * c2)
        };
        let q_eval: Ext<C::F, C::EF> = if mode == 1 {
            builder.eval(q1 * q2)
        } else {
            builder.eval(q1 * c1 + q2 * c2)
        };

        logup_p_evals.push(p_eval);
        logup_q_evals.push(q_eval);

        let alpha_denominator: Ext<C::F, C::EF> = builder.eval(alpha_acc * alpha);
        let eval_rlc: Ext<C::F, C::EF> =
            builder.eval(alpha_acc * p_eval + alpha_denominator * q_eval);

        builder.assign(&eval_acc, eval_acc + eval_rlc);
        builder.assign(&alpha_acc, alpha_acc * alpha * alpha);
    }

    let r_evals = once(eval_acc)
        .chain(p_evals)
        .chain(logup_p_evals)
        .chain(logup_q_evals)
        .collect::<Vec<_>>();

    let prod_spec_evals_id = builder.hint_load();
    let logup_spec_evals_id = builder.hint_load();

    let next_layer_evals: Array<C, Ext<C::F, C::EF>> = builder.dyn_array(r_evals.len());

    builder.sumcheck_layer_eval(
        &ctx,
        &challenges,
        prod_spec_evals_id,
        logup_spec_evals_id,
        &next_layer_evals,
    );

    for (idx, e) in r_evals.into_iter().enumerate() {
        let next_eval = builder.get(&next_layer_evals, idx);
        builder.assert_ext_eq(next_eval, e);
    }

    builder.halt();
}
