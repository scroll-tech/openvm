use openvm_circuit::arch::{instructions::program::Program, SystemConfig, VmConfig, VmExecutor, verify_single, VirtualMachine,};
use openvm_native_circuit::{Native, NativeConfig};
use openvm_native_compiler::{
    prelude::*,
    asm::{AsmBuilder, AsmCompiler}, ir::Felt,
    conversion::{convert_program, CompilerOptions}, 
};
use openvm_native_recursion::{testing_utils::inner::run_recursive_test, challenger::{duplex::DuplexChallengerVariable, CanObserveVariable}};
use openvm_stark_backend::{
    config::{Domain, StarkGenericConfig},
    p3_commit::PolynomialSpace,
    p3_field::{extension::BinomialExtensionField, FieldAlgebra},
};
use openvm_stark_sdk::{
    config::FriParameters, 
    p3_baby_bear::BabyBear, 
    utils::ProofInputForTest,
    config::{
        baby_bear_poseidon2::BabyBearPoseidon2Engine,
        fri_params::standard_fri_params_with_100_bits_conjectured_security,
    },
    engine::StarkFriEngine,
    utils::create_seeded_rng,
};
use rand::Rng;
pub type F = BabyBear;
pub type E = BinomialExtensionField<F, 4>;

fn fibonacci_program(a: u32, b: u32, n: u32) -> Program<BabyBear> {
    type F = BabyBear;
    type EF = BinomialExtensionField<BabyBear, 4>;

    let mut builder = AsmBuilder::<F, EF>::default();

    let prev: Felt<_> = builder.constant(F::from_canonical_u32(a));
    let next: Felt<_> = builder.constant(F::from_canonical_u32(b));

    builder.commit_public_value(prev);
    builder.commit_public_value(next);

    for _ in 2..n {
        let tmp: Felt<_> = builder.uninit();
        builder.assign(&tmp, next);
        builder.assign(&next, prev + next);
        builder.assign(&prev, tmp);
    }

    builder.commit_public_value(next);

    builder.halt();

    builder.compile_isa()
}

pub(crate) fn fibonacci_program_test_proof_input<SC: StarkGenericConfig>(
    a: u32,
    b: u32,
    n: u32,
) -> ProofInputForTest<SC>
where
    Domain<SC>: PolynomialSpace<Val = BabyBear>,
{
    let fib_program = fibonacci_program(a, b, n);
    let vm_config = NativeConfig::new(SystemConfig::default().with_public_values(3), Native);
    let airs = vm_config.create_chip_complex().unwrap().airs();

    let executor = VmExecutor::<BabyBear, NativeConfig>::new(vm_config);

    let mut result = executor.execute_and_generate(fib_program, vec![]).unwrap();
    assert_eq!(result.per_segment.len(), 1, "unexpected continuation");
    let proof_input = result.per_segment.remove(0);
    // Filter out unused AIRS (where trace is empty)
    let (used_airs, per_air) = proof_input
        .per_air
        .into_iter()
        .map(|(air_id, x)| (airs[air_id].clone(), x))
        .unzip();
    ProofInputForTest {
        airs: used_airs,
        per_air,
    }
}

#[test]
fn test_fibonacci_program_verify() {
    let fib_program_stark = fibonacci_program_test_proof_input(0, 1, 32);
    run_recursive_test(fib_program_stark, FriParameters::new_for_testing(3));
}

#[cfg(feature = "static-verifier")]
#[test]
#[ignore = "slow"]
fn test_fibonacci_program_halo2_verify() {
    use openvm_native_recursion::halo2::testing_utils::run_static_verifier_test;

    let fib_program_stark = fibonacci_program_test_proof_input(0, 1, 32);
    run_static_verifier_test(fib_program_stark, FriParameters::new_for_testing(3));
}

#[test]
fn test_multi_observe() {
    let mut builder = AsmBuilder::<BabyBear, BinomialExtensionField<F, 4>>::default();

    build_test_program(&mut builder);

    // Fill in test program logic
    builder.halt();

    let compilation_options = CompilerOptions::default().with_cycle_tracker();
    let mut compiler = AsmCompiler::new(compilation_options.word_size);
    compiler.build(builder.operations);
    let asm_code = compiler.code();

    // let program = Program::from_instructions(&instructions);
    let program: Program<_> = convert_program(asm_code, compilation_options);

    let poseidon2_max_constraint_degree = 3;
    
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

    let engine = BabyBearPoseidon2Engine::new(fri_params);
    let mut config = NativeConfig::aggregation(0, poseidon2_max_constraint_degree);
    config.system.memory_config.max_access_adapter_n = 16;

    let vm = VirtualMachine::new(engine, config);

    let pk = vm.keygen();
    let result = vm.execute_and_generate(program, vec![]).unwrap();
    let proofs = vm.prove(&pk, result);
    for proof in proofs {
        verify_single(&vm.engine, &pk.get_vk(), &proof).expect("Verification failed");
    }
}

fn build_test_program<C: Config>(
    builder: &mut Builder<C>,
) {
    let sample_lens: Vec<usize> = vec![10, 2, 0, 3, 20];
    
    let mut rng = create_seeded_rng();
    let mut challenger = DuplexChallengerVariable::new(builder);

    // Observe a setup label
    let label_f: Vec<u64> = vec![128, 3098, 192, 394, 1662, 928, 374, 281, 598, 182, 475, 729];
    for n in label_f {
        let f: Felt<C::F> = builder.constant(C::F::from_canonical_u64(n));
        challenger.observe(builder, f);
    }

    for l in sample_lens {
        let sample_input: Array<C, Felt<C::F>> = builder.dyn_array(l);
        builder.range(0, l).for_each(|idx_vec, builder| {
            let f_u32: u32 = rng.gen_range(1..1 << 30);
            builder.set(&sample_input, idx_vec[0], C::F::from_canonical_u32(f_u32));
        });

        let next_input_ptr = builder.poseidon2_multi_observe(&challenger.sponge_state, challenger.input_ptr, &sample_input);

        builder.assign(
            &challenger.input_ptr,
            challenger.io_empty_ptr + next_input_ptr.clone(),
        );
        builder.if_ne(next_input_ptr, Usize::from(0)).then_or_else(
            |builder| {
                builder.assign(&challenger.output_ptr, challenger.io_empty_ptr);
            },
            |builder| {
                builder.assign(&challenger.output_ptr, challenger.io_full_ptr);
            },
        );
    }
}
