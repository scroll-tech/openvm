extern crate core;

use std::{fs::read, panic::catch_unwind, path::Path, sync::Arc};

use commit::commit_app_exe;
use config::AppConfig;
use eyre::Result;
use keygen::{AggStarkProvingKey, AppProvingKey, AppVerifyingKey};
use openvm_build::{
    build_guest_package, find_unique_executable, get_package, GuestOptions, TargetFilter,
};
use openvm_circuit::{
    arch::{instructions::exe::VmExe, ExecutionError, VmConfig, VmExecutor},
    system::{memory::tree::public_values::extract_public_values, program::trace::VmCommittedExe},
};
use openvm_native_recursion::{
    halo2::{
        utils::Halo2ParamsReader,
        wrapper::{EvmVerifier, Halo2WrapperProvingKey},
        EvmProof,
    },
    types::InnerConfig,
};
use openvm_stark_backend::{engine::StarkEngine, prover::types::Proof};
use openvm_stark_sdk::{
    config::{
        baby_bear_poseidon2::{BabyBearPoseidon2Config, BabyBearPoseidon2Engine},
        baby_bear_poseidon2_root::BabyBearPoseidon2RootConfig,
        FriParameters,
    },
    engine::StarkFriEngine,
    openvm_stark_backend::{verifier::VerificationError, Chip},
    p3_baby_bear::BabyBear,
};
use openvm_transpiler::{
    elf::Elf,
    openvm_platform::memory::MEM_SIZE,
    transpiler::{Transpiler, TranspilerError},
    FromElf,
};
use prover::vm::ContinuationVmProof;

pub mod commit;
pub mod config;
pub mod prover;
pub mod static_verifier;

pub mod keygen;
pub mod verifier;

mod stdin;
pub use stdin::*;
use verifier::root::types::RootVmVerifierInput;
pub mod fs;

use crate::{
    config::AggConfig,
    keygen::AggProvingKey,
    prover::{AppProver, ContinuationProver, StarkProver},
};

pub(crate) type SC = BabyBearPoseidon2Config;
pub(crate) type C = InnerConfig;
pub(crate) type F = BabyBear;
pub(crate) type RootSC = BabyBearPoseidon2RootConfig;
pub type NonRootCommittedExe = VmCommittedExe<SC>;

pub struct Sdk;

impl Sdk {
    pub fn build<P: AsRef<Path>>(
        &self,
        guest_opts: GuestOptions,
        pkg_dir: P,
        target_filter: &Option<TargetFilter>,
    ) -> Result<Elf> {
        let pkg = get_package(pkg_dir.as_ref());
        let target_dir = match build_guest_package(&pkg, &guest_opts, None, target_filter) {
            Ok(target_dir) => target_dir,
            Err(Some(code)) => {
                return Err(eyre::eyre!("Failed to build guest: code = {}", code));
            }
            Err(None) => {
                return Err(eyre::eyre!(
                    "Failed to build guest (OPENVM_SKIP_BUILD is set)"
                ));
            }
        };

        let elf_path = find_unique_executable(pkg_dir, target_dir, target_filter)?;
        let data = read(&elf_path)?;
        Elf::decode(&data, MEM_SIZE as u32)
    }

    pub fn transpile(
        &self,
        elf: Elf,
        transpiler: Transpiler<F>,
    ) -> Result<VmExe<F>, TranspilerError> {
        VmExe::from_elf(elf, transpiler)
    }

    pub fn execute<VC: VmConfig<F>>(
        &self,
        exe: VmExe<F>,
        vm_config: VC,
        inputs: StdIn,
    ) -> Result<Vec<F>, ExecutionError>
    where
        VC::Executor: Chip<SC>,
        VC::Periphery: Chip<SC>,
    {
        let vm = VmExecutor::new(vm_config);
        let final_memory = vm.execute(exe, inputs)?;
        let public_values = extract_public_values(
            &vm.config.system().memory_config.memory_dimensions(),
            vm.config.system().num_public_values,
            final_memory.as_ref().unwrap(),
        );
        Ok(public_values)
    }

    pub fn commit_app_exe(
        &self,
        app_fri_params: FriParameters,
        exe: VmExe<F>,
    ) -> Result<Arc<NonRootCommittedExe>> {
        let committed_exe = commit_app_exe(app_fri_params, exe);
        Ok(committed_exe)
    }

    pub fn app_keygen<VC: VmConfig<F>>(&self, config: AppConfig<VC>) -> Result<AppProvingKey<VC>>
    where
        VC::Executor: Chip<SC>,
        VC::Periphery: Chip<SC>,
    {
        let app_pk = AppProvingKey::keygen(config);
        Ok(app_pk)
    }

    pub fn generate_app_proof<VC: VmConfig<F>>(
        &self,
        app_pk: Arc<AppProvingKey<VC>>,
        app_committed_exe: Arc<NonRootCommittedExe>,
        inputs: StdIn,
    ) -> Result<ContinuationVmProof<SC>>
    where
        VC::Executor: Chip<SC>,
        VC::Periphery: Chip<SC>,
    {
        let app_prover = AppProver::new(app_pk.app_vm_pk.clone(), app_committed_exe);
        let proof = app_prover.generate_app_proof(inputs);
        Ok(proof)
    }

    pub fn verify_app_proof(
        &self,
        app_vk: &AppVerifyingKey,
        proof: &ContinuationVmProof<SC>,
    ) -> Result<(), VerificationError> {
        let e = BabyBearPoseidon2Engine::new(app_vk.fri_params);
        for seg_proof in &proof.per_segment {
            e.verify(&app_vk.app_vm_vk, seg_proof)?
        }
        // TODO: verify continuation: requires App VC.
        Ok(())
    }

    pub fn verify_app_proof_without_continuations(
        &self,
        app_vk: &AppVerifyingKey,
        proof: &Proof<SC>,
    ) -> Result<(), VerificationError> {
        let e = BabyBearPoseidon2Engine::new(app_vk.fri_params);
        e.verify(&app_vk.app_vm_vk, proof)
    }

    pub fn agg_keygen(
        &self,
        config: AggConfig,
        reader: &impl Halo2ParamsReader,
    ) -> Result<AggProvingKey> {
        let agg_pk = AggProvingKey::keygen(config, reader);
        Ok(agg_pk)
    }

    pub fn generate_root_proof<VC: VmConfig<F>>(
        &self,
        app_pk: Arc<AppProvingKey<VC>>,
        app_exe: Arc<NonRootCommittedExe>,
        agg_stark_pk: AggStarkProvingKey,
        inputs: StdIn,
    ) -> Result<RootVmVerifierInput<SC>>
    where
        VC::Executor: Chip<SC>,
        VC::Periphery: Chip<SC>,
    {
        let stark_prover = StarkProver::new(app_pk, app_exe, agg_stark_pk);
        let proof = stark_prover.generate_proof_for_root_program(inputs);
        Ok(proof)
    }

    pub fn generate_evm_proof<VC: VmConfig<F>>(
        &self,
        reader: &impl Halo2ParamsReader,
        app_pk: Arc<AppProvingKey<VC>>,
        app_exe: Arc<NonRootCommittedExe>,
        agg_pk: AggProvingKey,
        inputs: StdIn,
    ) -> Result<EvmProof>
    where
        VC::Executor: Chip<SC>,
        VC::Periphery: Chip<SC>,
    {
        let e2e_prover = ContinuationProver::new(reader, app_pk, app_exe, agg_pk);
        let proof = e2e_prover.generate_proof_for_evm(inputs);
        Ok(proof)
    }

    pub fn generate_snark_verifier_contract(
        &self,
        reader: &impl Halo2ParamsReader,
        agg_pk: &AggProvingKey,
    ) -> Result<EvmVerifier> {
        let params = reader.read_params(agg_pk.halo2_pk.wrapper.pinning.metadata.config_params.k);
        let evm_verifier = agg_pk.halo2_pk.wrapper.generate_evm_verifier(&params);
        Ok(evm_verifier)
    }

    pub fn verify_evm_proof(&self, evm_verifier: &EvmVerifier, evm_proof: &EvmProof) -> bool {
        // FIXME: we should return the concrete error.
        catch_unwind(|| {
            Halo2WrapperProvingKey::evm_verify(evm_verifier, evm_proof);
        })
        .is_ok()
    }
}
