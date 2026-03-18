use openvm_circuit::system::{
    memory::{offline_checker::MemoryBaseAuxCols, online::TracingMemory, MemoryAuxColsFactory},
    native_adapter::util::tracing_read_native,
};
use p3_field::PrimeField32;

pub(crate) const CASTF_MAX_BITS: usize = 30;
#[cfg(feature = "cuda")]
pub(crate) const OPENVM_NATIVE_GPU_DEBUG_ID: &str = "OVM-NATIVE-GPU-DBG-20260318";

pub(crate) const fn const_max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

#[cfg(feature = "cuda")]
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        hash ^= b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(feature = "cuda")]
pub(crate) fn debug_log_native_gpu_tracegen_input(
    chip_label: &str,
    records: &[u8],
    record_size: usize,
    height: usize,
    padded_height: usize,
    trace_width: usize,
) -> u64 {
    let hash = fnv1a64(records);
    let head_len = records.len().min(64);
    println!(
        "[openvm-gpu-debug][{}][{}] records_bytes={} record_size={} height={} padded_height={} trace_width={} hash=0x{:016x}",
        OPENVM_NATIVE_GPU_DEBUG_ID,
        chip_label,
        records.len(),
        record_size,
        height,
        padded_height,
        trace_width,
        hash
    );
    println!(
        "[openvm-gpu-debug][{}][{}] records_head({})={:?}",
        OPENVM_NATIVE_GPU_DEBUG_ID,
        chip_label,
        head_len,
        &records[..head_len]
    );
    hash
}

/// Fill `MemoryBaseAuxCols`, assuming that the `prev_timestamp` is already set in `base_aux`.
pub(crate) fn mem_fill_helper<F: PrimeField32>(
    mem_helper: &MemoryAuxColsFactory<F>,
    timestamp: u32,
    base_aux: &mut MemoryBaseAuxCols<F>,
) {
    let prev_ts = base_aux.prev_timestamp.as_canonical_u32();
    mem_helper.fill(prev_ts, timestamp, base_aux);
}

pub(crate) fn tracing_read_native_helper<F: PrimeField32, const BLOCK_SIZE: usize>(
    memory: &mut TracingMemory,
    ptr: u32,
    base_aux: &mut MemoryBaseAuxCols<F>,
) -> [F; BLOCK_SIZE] {
    let mut prev_ts = 0;
    let ret = tracing_read_native(memory, ptr, &mut prev_ts);
    base_aux.set_prev(F::from_canonical_u32(prev_ts));
    ret
}

/// Testing framework
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils {
    use std::array;

    use openvm_circuit::{
        arch::{
            execution_mode::Segment,
            testing::{memory::gen_pointer, TestBuilder},
            PreflightExecutionOutput, PreflightExecutor, Streams, VirtualMachine,
            VirtualMachineError, VmBuilder, VmExecutionConfig, VmState,
        },
        utils::test_system_config_without_continuations,
    };
    use openvm_instructions::{
        exe::VmExe,
        program::Program,
        riscv::{RV32_MEMORY_AS, RV32_REGISTER_AS},
    };
    use openvm_native_compiler::conversion::AS;
    use openvm_stark_backend::{
        config::Domain, p3_commit::PolynomialSpace, p3_field::PrimeField32,
    };
    use openvm_stark_sdk::{
        config::{baby_bear_poseidon2::BabyBearPoseidon2Engine, setup_tracing, FriParameters},
        engine::StarkFriEngine,
        p3_baby_bear::BabyBear,
    };
    use rand::{distributions::Standard, prelude::Distribution, rngs::StdRng, Rng};

    use crate::{NativeConfig, NativeCpuBuilder, Rv32WithKernelsConfig};

    // If immediate, returns (value, AS::Immediate). Otherwise, writes to native memory and returns
    // (ptr, AS::Native). If is_imm is None, randomizes it.
    pub fn write_native_or_imm<F: PrimeField32>(
        tester: &mut impl TestBuilder<F>,
        rng: &mut StdRng,
        value: F,
        is_imm: Option<bool>,
    ) -> (F, usize) {
        let is_imm = is_imm.unwrap_or(rng.gen_bool(0.5));
        if is_imm {
            (value, AS::Immediate as usize)
        } else {
            let ptr = gen_pointer(rng, 1);
            tester.write::<1>(AS::Native as usize, ptr, [value]);
            (F::from_canonical_usize(ptr), AS::Native as usize)
        }
    }

    // Writes value to native memory and returns a pointer to the first element together with the
    // value If `value` is None, randomizes it.
    pub fn write_native_array<F: PrimeField32, const N: usize>(
        tester: &mut impl TestBuilder<F>,
        rng: &mut StdRng,
        value: Option<[F; N]>,
    ) -> ([F; N], usize)
    where
        Standard: Distribution<F>, // Needed for `rng.gen`
    {
        let value = value.unwrap_or(array::from_fn(|_| rng.gen()));
        let ptr = gen_pointer(rng, N);
        tester.write::<N>(AS::Native as usize, ptr, value);
        (value, ptr)
    }

    // Besides taking in system_config, this also returns Result and the full
    // (PreflightExecutionOutput, VirtualMachine) for more advanced testing needs.
    #[allow(clippy::type_complexity)]
    pub fn execute_program_with_config<E, VB>(
        program: Program<BabyBear>,
        input_stream: impl Into<Streams<BabyBear>>,
        builder: VB,
        config: VB::VmConfig,
    ) -> Result<
        (
            PreflightExecutionOutput<BabyBear, <VB as VmBuilder<E>>::RecordArena>,
            VirtualMachine<E, VB>,
        ),
        VirtualMachineError,
    >
    where
        E: StarkFriEngine,
        Domain<E::SC>: PolynomialSpace<Val = BabyBear>,
        VB: VmBuilder<E, VmConfig = NativeConfig>,
        <VB::VmConfig as VmExecutionConfig<BabyBear>>::Executor:
            PreflightExecutor<BabyBear, VB::RecordArena>,
    {
        setup_tracing();
        assert!(!config.as_ref().continuation_enabled);
        let input = input_stream.into();

        let engine = E::new(FriParameters::new_for_testing(1));
        let exe = VmExe::new(program);
        let (vm, _) = VirtualMachine::new_with_keygen(engine, builder, config)?;
        let ctx = vm.build_metered_ctx(&exe);
        let (mut segments, _) = vm
            .metered_interpreter(&exe)?
            .execute_metered(input.clone(), ctx)?;
        assert_eq!(segments.len(), 1, "test only supports one segment");
        let Segment {
            instret_start,
            num_insns,
            trace_heights,
        } = segments.pop().unwrap();
        assert_eq!(instret_start, 0);
        let state = vm.create_initial_state(&exe, input);
        let mut preflight_interpreter = vm.preflight_interpreter(&exe)?;
        let output =
            vm.execute_preflight(&mut preflight_interpreter, state, None, &trace_heights)?;
        assert_eq!(
            output.to_state.instret(),
            num_insns,
            "metered execution insn count doesn't match preflight execution"
        );
        Ok((output, vm))
    }

    pub fn execute_program(
        program: Program<BabyBear>,
        input_stream: impl Into<Streams<BabyBear>>,
    ) -> VmState<BabyBear> {
        let mut config = test_native_config();
        config.system.num_public_values = 4;
        // we set max segment len large so it doesn't segment
        let (output, _) = execute_program_with_config::<BabyBearPoseidon2Engine, _>(
            program,
            input_stream,
            NativeCpuBuilder,
            config,
        )
        .unwrap();
        output.to_state
    }

    pub fn test_native_config() -> NativeConfig {
        let mut system = test_system_config_without_continuations();
        system.memory_config.addr_spaces[RV32_REGISTER_AS as usize].num_cells = 0;
        system.memory_config.addr_spaces[RV32_MEMORY_AS as usize].num_cells = 0;
        NativeConfig {
            system,
            native: Default::default(),
        }
    }

    pub fn test_native_continuations_config() -> NativeConfig {
        NativeConfig {
            system: test_system_config_without_continuations().with_continuations(),
            native: Default::default(),
        }
    }

    pub fn test_rv32_with_kernels_config() -> Rv32WithKernelsConfig {
        Rv32WithKernelsConfig {
            system: test_system_config_without_continuations().with_continuations(),
            ..Default::default()
        }
    }
}
