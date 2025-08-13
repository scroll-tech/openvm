use itertools::Itertools;
use openvm_circuit::{
    arch::{
        instructions::program::Program, PreflightExecutionOutput, PreflightExecutor, VmBuilder,
        VmCircuitConfig, VmExecutionConfig,
    },
    utils::TestStarkEngine,
};
use openvm_native_circuit::{
    execute_program_with_config, test_native_config, NativeBuilder, NativeConfig,
};
use openvm_native_compiler::{asm::AsmBuilder, ir::Felt};
use openvm_native_recursion::testing_utils::inner::run_recursive_test;
use openvm_stark_backend::{
    config::{Domain, StarkGenericConfig, Val},
    p3_commit::PolynomialSpace,
    p3_field::{extension::BinomialExtensionField, FieldAlgebra},
    prover::{
        cpu::{CpuBackend, CpuDevice},
        hal::{DeviceDataTransporter, ProverBackend, TraceCommitter},
        types::AirProvingContext,
    },
};
use openvm_stark_sdk::{
    config::{
        baby_bear_poseidon2::{BabyBearPoseidon2Config, BabyBearPoseidon2Engine},
        FriParameters,
    },
    engine::StarkFriEngine,
    p3_baby_bear::BabyBear,
    utils::ProofInputForTest,
};

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

// We need this for both BabyBearPoseidon2Config and BabyBearPoseidon2RootConfig
pub(crate) fn fibonacci_program_test_proof_input<SC, E, CpuEngine, const LOG_BLOWUP: usize>(
    a: u32,
    b: u32,
    n: u32,
) -> ProofInputForTest<SC>
where
    SC: StarkGenericConfig,
    E: StarkFriEngine<SC = SC>,
    CpuEngine: StarkFriEngine<SC = SC, PB = CpuBackend<SC>, PD = CpuDevice<SC>>,
    Domain<SC>: PolynomialSpace<Val = BabyBear>,
    NativeBuilder: VmBuilder<E, VmConfig = NativeConfig>,
    <NativeConfig as VmExecutionConfig<BabyBear>>::Executor:
        PreflightExecutor<BabyBear, <NativeBuilder as VmBuilder<E>>::RecordArena>,
    E::PB: ProverBackend<Val = Val<SC>>,
{
    let fib_program = fibonacci_program(a, b, n);
    let mut config = test_native_config();
    let fri_params = FriParameters::new_for_testing(LOG_BLOWUP);
    let engine = E::new(fri_params);
    let cpu_engine = CpuEngine::new(fri_params);
    config.as_mut().num_public_values = 3;

    let (output, mut vm) = execute_program_with_config::<E, _>(
        fib_program.clone(),
        vec![],
        NativeBuilder::default(),
        config.clone(),
    )
    .unwrap();
    let PreflightExecutionOutput {
        system_records,
        record_arenas,
        ..
    } = output;
    let cached_program_trace = vm.commit_program_on_device(&fib_program);
    vm.load_program(cached_program_trace);
    let ctx = vm
        .generate_proving_ctx(system_records, record_arenas)
        .unwrap();

    let airs = config.create_airs().unwrap().into_airs().collect_vec();
    let (used_airs, per_air): (Vec<_>, Vec<_>) = ctx
        .per_air
        .into_iter()
        .map(|(air_id, air_ctx)| {
            let AirProvingContext {
                cached_mains,
                common_main,
                public_values,
            } = air_ctx;
            let cached_mains = cached_mains
                .into_iter()
                .map(|com| {
                    let trace = engine
                        .device()
                        .transport_matrix_from_device_to_host(&com.trace);
                    let (commitment, data) = cpu_engine.device().commit(&[trace.clone()]);
                    cpu_engine
                        .device()
                        .transport_committed_trace_to_device(commitment, &trace, &data.data)
                })
                .collect::<Vec<_>>();
            let out_ctx = AirProvingContext::<CpuBackend<SC>>::new(
                cached_mains,
                common_main.map(|m| engine.device().transport_matrix_from_device_to_host(&m)),
                public_values,
            );
            (airs[air_id].clone(), out_ctx)
        })
        .unzip();
    ProofInputForTest {
        airs: used_airs,
        per_air,
    }
}

#[test]
fn test_fibonacci_program_verify() {
    let fib_program_stark = fibonacci_program_test_proof_input::<
        BabyBearPoseidon2Config,
        TestStarkEngine,
        BabyBearPoseidon2Engine,
        1, // log_blowup to match hard-coded value in `run_recursive_test`
    >(0, 1, 32);
    run_recursive_test(fib_program_stark, FriParameters::new_for_testing(3));
}

#[cfg(all(feature = "static-verifier", not(feature = "cuda")))]
#[test]
#[ignore = "needs params files"]
fn test_fibonacci_program_halo2_verify() {
    use openvm_native_recursion::halo2::testing_utils::run_static_verifier_test;
    use openvm_stark_sdk::config::baby_bear_poseidon2_root::{
        BabyBearPoseidon2RootConfig, BabyBearPoseidon2RootEngine,
    };

    const LOG_BLOWUP: usize = 3;
    let fib_program_stark = fibonacci_program_test_proof_input::<
        BabyBearPoseidon2RootConfig,
        BabyBearPoseidon2RootEngine,
        BabyBearPoseidon2RootEngine,
        { LOG_BLOWUP },
    >(0, 1, 32);
    run_static_verifier_test(
        fib_program_stark,
        FriParameters::new_for_testing(LOG_BLOWUP),
    );
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
    let challenger = DuplexChallengerVariable::new(builder);

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
    let challenger = DuplexChallengerVariable::new(builder);

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
