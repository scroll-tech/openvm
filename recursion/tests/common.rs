use afs_compiler::asm::AsmBuilder;
use afs_compiler::ir::Var;
use afs_stark_backend::keygen::types::MultiStarkVerifyingKey;
use p3_baby_bear::BabyBear;
use p3_field::extension::BinomialExtensionField;
use p3_field::AbstractField;
use p3_matrix::dense::RowMajorMatrix;
use p3_matrix::Matrix;
use p3_uni_stark::StarkGenericConfig;
use p3_util::log2_strict_usize;
use stark_vm::cpu::trace::Instruction;
use std::cmp::Reverse;

use afs_compiler::util::execute_program;
use afs_recursion::hints::{Hintable, InnerVal};
use afs_recursion::multi_stark::AggregationVerifierProgram;
use afs_recursion::stark::{sort_chips, DynRapForRecursion, VerifierProgram};
use afs_recursion::types::{
    new_from_multi_vk, AggregationVerifierInput, InnerConfig, VerifierInput,
};
use afs_stark_backend::prover::trace::TraceCommitmentBuilder;
use afs_stark_backend::prover::types::Proof;
use afs_stark_backend::rap::AnyRap;
use afs_stark_backend::verifier::MultiTraceStarkVerifier;
use afs_test_utils::config::baby_bear_poseidon2::{
    default_perm, engine_from_perm, BabyBearPoseidon2Config,
};
use afs_test_utils::config::FriParameters;
use afs_test_utils::engine::StarkEngine;

pub struct VerificationParams<SC: StarkGenericConfig> {
    pub vk: MultiStarkVerifyingKey<SC>,
    pub proof: Proof<SC>,
    pub fri_params: FriParameters,
}

pub fn make_verification_params(
    raps: &[&dyn AnyRap<BabyBearPoseidon2Config>],
    traces: Vec<RowMajorMatrix<BabyBear>>,
    pvs: &[Vec<BabyBear>],
    fri_params: FriParameters,
) -> VerificationParams<BabyBearPoseidon2Config> {
    let num_pvs: Vec<usize> = pvs.iter().map(|pv| pv.len()).collect();

    let trace_heights: Vec<usize> = traces.iter().map(|t| t.height()).collect();
    let log_degree = log2_strict_usize(trace_heights.into_iter().max().unwrap());

    let engine = engine_from_perm(default_perm(), log_degree, fri_params);

    let mut keygen_builder = engine.keygen_builder();
    for (&rap, &num_pv) in raps.iter().zip(num_pvs.iter()) {
        keygen_builder.add_air(rap, num_pv);
    }

    let pk = keygen_builder.generate_pk();
    let vk = pk.vk();

    let prover = engine.prover();
    let mut trace_builder = TraceCommitmentBuilder::new(prover.pcs());
    for trace in traces.clone() {
        trace_builder.load_trace(trace);
    }
    trace_builder.commit_current();

    let main_trace_data = trace_builder.view(&vk, raps.to_vec());

    let mut challenger = engine.new_challenger();
    let proof = prover.prove(&mut challenger, &pk, main_trace_data, pvs);

    let verifier = MultiTraceStarkVerifier::new(prover.config);
    verifier
        .verify(
            &mut engine.new_challenger(),
            &vk,
            raps.to_vec(),
            &proof,
            pvs,
        )
        .expect("proof should verify");

    VerificationParams {
        vk,
        proof,
        fri_params: engine.fri_params,
    }
}

pub fn build_verification_program(
    rec_raps: Vec<&dyn DynRapForRecursion<InnerConfig>>,
    pvs: Vec<Vec<InnerVal>>,
    vparams: VerificationParams<BabyBearPoseidon2Config>,
) -> (Vec<Instruction<BabyBear>>, Vec<Vec<InnerVal>>) {
    let VerificationParams {
        vk,
        proof,
        fri_params,
    } = vparams;

    let advice = new_from_multi_vk(&vk);
    let program = VerifierProgram::build(rec_raps, advice, &fri_params);

    let log_degree_per_air = proof
        .degrees
        .iter()
        .map(|degree| log2_strict_usize(*degree))
        .collect();

    let input = VerifierInput {
        proof,
        log_degree_per_air,
        public_values: pvs.clone(),
    };

    let mut input_stream = Vec::new();
    input_stream.extend(input.write());

    (program, input_stream)
}

pub fn build_continuations_verification_program(
    rec_raps: [Vec<&dyn DynRapForRecursion<InnerConfig>>; 2],
    pvs: [Vec<Vec<InnerVal>>; 2],
    vparams: [VerificationParams<BabyBearPoseidon2Config>; 2],
) -> (Vec<Instruction<BabyBear>>, Vec<Vec<InnerVal>>) {
    let [VerificationParams {
        vk: vk_1,
        proof: proof_1,
        fri_params: fri_params_1,
    }, VerificationParams {
        vk: vk_2,
        proof: proof_2,
        ..
    }] = vparams;
    let [pvs_1, pvs_2] = pvs;

    let advice_1 = new_from_multi_vk(&vk_1);

    let log_degree_per_air_1 = proof_1
        .degrees
        .iter()
        .map(|degree| log2_strict_usize(*degree))
        .collect();

    let input_1 = VerifierInput {
        proof: proof_1,
        log_degree_per_air: log_degree_per_air_1,
        public_values: pvs_1.clone(),
    };

    let advice_2 = new_from_multi_vk(&vk_2);

    let log_degree_per_air_2 = proof_2
        .degrees
        .iter()
        .map(|degree| log2_strict_usize(*degree))
        .collect();

    let input_2 = VerifierInput {
        proof: proof_2,
        log_degree_per_air: log_degree_per_air_2,
        public_values: pvs_2.clone(),
    };

    let input = AggregationVerifierInput {
        verifier_input_1: input_1,
        verifier_input_2: input_2,
    };

    let input_stream = input.write();
    let program = AggregationVerifierProgram::build(rec_raps, [advice_1, advice_2], &fri_params_1);

    (program, input_stream)
}

#[allow(dead_code)]
pub fn run_recursive_test(
    // TODO: find way to de-duplicate parameters
    any_raps: Vec<&dyn AnyRap<BabyBearPoseidon2Config>>,
    rec_raps: Vec<&dyn DynRapForRecursion<InnerConfig>>,
    traces: Vec<RowMajorMatrix<BabyBear>>,
    pvs: Vec<Vec<BabyBear>>,
    fri_params: FriParameters,
) {
    let (any_raps, rec_raps, traces, pvs) = sort_chips(any_raps, rec_raps, traces, pvs);

    let vparams = make_verification_params(&any_raps, traces, &pvs, fri_params);

    let (program, witness_stream) = build_verification_program(rec_raps, pvs, vparams);
    execute_program::<1>(program, witness_stream);
}

#[allow(dead_code)]
pub fn run_continuations_test(
    any_raps: Vec<Vec<&dyn AnyRap<BabyBearPoseidon2Config>>>,
    rec_raps: Vec<Vec<&dyn DynRapForRecursion<InnerConfig>>>,
    traces: Vec<Vec<RowMajorMatrix<BabyBear>>>,
    pvs: Vec<Vec<Vec<BabyBear>>>,
) {
    assert!(any_raps.len() >= 2, "any_raps length is less than 2");
    assert!(rec_raps.len() >= 2, "rec_raps length is less than 2");
    assert!(traces.len() >= 2, "traces length is less than 2");
    assert!(pvs.len() >= 2, "pvs length is less than 2");

    let (any_raps_0, any_raps_1, rec_raps_0, rec_raps_1, traces_0, traces_1, pvs_0, pvs_1) = (
        &any_raps[0],
        &any_raps[1],
        &rec_raps[0],
        &rec_raps[1],
        &traces[0],
        &traces[1],
        &pvs[0],
        &pvs[1],
    );

    let vparams_0 = make_verification_params(any_raps_0, traces_0.clone(), pvs_0);
    let vparams_1 = make_verification_params(any_raps_1, traces_1.clone(), pvs_1);

    let rec_raps_arr = [rec_raps_0.clone(), rec_raps_1.clone()];
    let pvs_arr = [pvs_0.clone(), pvs_1.clone()];
    let vparams_arr = [vparams_0, vparams_1];

    let (program, witness_stream) =
        build_continuations_verification_program(rec_raps_arr, pvs_arr, vparams_arr);
    execute_program::<1>(program, witness_stream);
}

pub fn sort_chips<'a>(
    chips: Vec<&'a dyn AnyRap<BabyBearPoseidon2Config>>,
    rec_raps: Vec<&'a dyn DynRapForRecursion<InnerConfig>>,
    traces: Vec<RowMajorMatrix<BabyBear>>,
    pvs: Vec<Vec<BabyBear>>,
) -> (
    Vec<&'a dyn AnyRap<BabyBearPoseidon2Config>>,
    Vec<&'a dyn DynRapForRecursion<InnerConfig>>,
    Vec<RowMajorMatrix<BabyBear>>,
    Vec<Vec<BabyBear>>,
) {
    let mut groups = izip!(chips, rec_raps, traces, pvs).collect_vec();
    groups.sort_by_key(|(_, _, trace, _)| Reverse(trace.height()));

    let chips = groups.iter().map(|(x, _, _, _)| *x).collect_vec();
    let rec_raps = groups.iter().map(|(_, x, _, _)| *x).collect_vec();
    let pvs = groups.iter().map(|(_, _, _, x)| x.clone()).collect_vec();
    let traces = groups.into_iter().map(|(_, _, x, _)| x).collect_vec();

    (chips, rec_raps, traces, pvs)
}

pub fn sort_chips_with_id<'a>(
    chips: Vec<&'a dyn AnyRap<BabyBearPoseidon2Config>>,
    rec_raps: Vec<&'a dyn DynRapForRecursion<InnerConfig>>,
    traces: Vec<RowMajorMatrix<BabyBear>>,
    pvs: Vec<Vec<BabyBear>>,
    chip_ids: Vec<Vec<usize>>,
) -> (
    Vec<&'a dyn AnyRap<BabyBearPoseidon2Config>>,
    Vec<&'a dyn DynRapForRecursion<InnerConfig>>,
    Vec<RowMajorMatrix<BabyBear>>,
    Vec<Vec<BabyBear>>,
    Vec<Vec<usize>>,
) {
    let mut groups = izip!(chips, rec_raps, traces, pvs, chip_ids).collect_vec();
    groups.sort_by_key(|(_, _, trace, _, _)| Reverse(trace.height()));

    let chips = groups.iter().map(|(x, _, _, _, _)| *x).collect_vec();
    let rec_raps = groups.iter().map(|(_, x, _, _, _)| *x).collect_vec();
    let pvs = groups.iter().map(|(_, _, _, x, _)| x.clone()).collect_vec();
    let chip_ids = groups.iter().map(|(_, _, _, _, x)| x.clone()).collect_vec();
    let traces = groups.into_iter().map(|(_, _, x, _, _)| x).collect_vec();

    (chips, rec_raps, traces, pvs, chip_ids)
}

pub fn fibonacci_program(a: u32, b: u32, n: u32) -> Vec<Instruction<BabyBear>> {
    type F = BabyBear;
    type EF = BinomialExtensionField<BabyBear, 4>;

    let mut builder = AsmBuilder::<F, EF>::default();

    let prev: Var<_> = builder.constant(F::from_canonical_u32(a));
    let next: Var<_> = builder.constant(F::from_canonical_u32(b));

    for _ in 0..n {
        let tmp: Var<_> = builder.uninit();
        builder.assign(tmp, next);
        builder.assign(next, prev + next);
        builder.assign(prev, tmp);
    }

    builder.halt();

    builder.compile_isa::<1>()
}
