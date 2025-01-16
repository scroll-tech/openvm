use std::sync::Arc;

use openvm_circuit::arch::VmConfig;
use openvm_stark_backend::{prover::types::Proof, Chip};

use crate::{
    keygen::{AggStarkProvingKey, AppProvingKey},
    prover::{agg::AggStarkProver, app::AppProver},
    verifier::root::types::RootVmVerifierInput,
    NonRootCommittedExe, RootSC, StdIn, F, SC,
};

pub struct StarkProver<VC> {
    app_prover: AppProver<VC>,
    agg_prover: AggStarkProver,
}
impl<VC> StarkProver<VC> {
    pub fn new(
        app_pk: Arc<AppProvingKey<VC>>,
        app_committed_exe: Arc<NonRootCommittedExe>,
        agg_stark_pk: AggStarkProvingKey,
    ) -> Self
    where
        VC: VmConfig<F>,
    {
        assert_eq!(
            app_pk.leaf_fri_params, agg_stark_pk.leaf_vm_pk.fri_params,
            "App VM is incompatible with Agg VM because of leaf FRI parameters"
        );
        assert_eq!(
            app_pk.app_vm_pk.vm_config.system().num_public_values,
            agg_stark_pk.num_public_values(),
            "App VM is incompatible with Agg VM  because of the number of public values"
        );

        Self {
            app_prover: AppProver::new(app_pk.app_vm_pk.clone(), app_committed_exe),
            agg_prover: AggStarkProver::new(agg_stark_pk, app_pk.leaf_committed_exe.clone()),
        }
    }
    pub fn set_program_name(&mut self, program_name: impl AsRef<str>) -> &mut Self {
        self.app_prover.set_program_name(program_name);
        self
    }
    pub fn generate_proof_for_outer_recursion(&self, input: StdIn) -> Proof<RootSC>
    where
        VC: VmConfig<F>,
        VC::Executor: Chip<SC>,
        VC::Periphery: Chip<SC>,
    {
        let app_proof = self.app_prover.generate_app_proof(input);
        self.agg_prover.generate_agg_proof(app_proof)
    }
    pub fn generate_proof_for_root_program(&self, input: StdIn) -> RootVmVerifierInput<SC>
    where
        VC: VmConfig<F>,
        VC::Executor: Chip<SC>,
        VC::Periphery: Chip<SC>,
    {
        let app_proof = self.app_prover.generate_app_proof(input);
        self.agg_prover.generate_root_proof(app_proof)
    }
}
