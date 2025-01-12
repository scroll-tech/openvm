use openvm_circuit::arch::{SingleSegmentVmExecutor, Streams};
use openvm_native_circuit::NativeConfig;
use openvm_native_recursion::hints::Hintable;
use openvm_stark_backend::prover::types::Proof;
use openvm_stark_sdk::{
    config::{baby_bear_poseidon2_root::BabyBearPoseidon2RootEngine, FriParameters},
    engine::{StarkEngine, StarkFriEngine},
};

use super::{vm::SingleSegmentVmProver, F, SC};
use crate::{keygen::MinimalProvingKey, verifier::root::types::RootVmVerifierInput, RootSC};

pub struct MinimalProver<VC> {
    pub minimal_verifier_pk: MinimalProvingKey<VC>,
    executor_for_heights: SingleSegmentVmExecutor<F, NativeConfig>,
}

impl<VC> MinimalProver<VC> {
    pub fn new(minimal_verifier_pk: MinimalProvingKey<VC>) -> Self {
        let executor_for_heights = SingleSegmentVmExecutor::<F, _>::new(
            minimal_verifier_pk.root_verifier_pk.vm_pk.vm_config.clone(),
        );
        Self {
            minimal_verifier_pk,
            executor_for_heights,
        }
    }

    pub fn execute_for_air_heights(&self, input: RootVmVerifierInput<SC>) -> Vec<usize> {
        let result = self
            .executor_for_heights
            .execute_and_compute_heights(
                self.minimal_verifier_pk
                    .root_verifier_pk
                    .root_committed_exe
                    .exe
                    .clone(),
                input.write(),
            )
            .unwrap();
        result.air_heights
    }

    pub fn vm_config(&self) -> &NativeConfig {
        &self.minimal_verifier_pk.root_verifier_pk.vm_pk.vm_config
    }

    pub(crate) fn fri_params(&self) -> &FriParameters {
        &self.minimal_verifier_pk.root_verifier_pk.vm_pk.fri_params
    }
}

impl<VC> SingleSegmentVmProver<RootSC> for MinimalProver<VC> {
    fn prove(&self, input: impl Into<Streams<F>>) -> Proof<RootSC> {
        let input = input.into();
        let vm = SingleSegmentVmExecutor::new(self.vm_config().clone());
        let mut proof_input = vm
            .execute_and_generate(
                self.minimal_verifier_pk
                    .root_verifier_pk
                    .root_committed_exe
                    .clone(),
                input,
            )
            .unwrap();
        assert_eq!(
            proof_input.per_air.len(),
            self.minimal_verifier_pk.root_verifier_pk.air_heights.len(),
            "All AIRs of root verifier should present"
        );
        proof_input.per_air.iter().for_each(|(air_id, input)| {
            assert_eq!(
                input.main_trace_height(),
                self.minimal_verifier_pk.root_verifier_pk.air_heights[*air_id],
                "Trace height doesn't match"
            );
        });
        // Reorder the AIRs by heights.
        let air_id_perm = self
            .minimal_verifier_pk
            .root_verifier_pk
            .air_id_permutation();
        air_id_perm.permute(&mut proof_input.per_air);
        for i in 0..proof_input.per_air.len() {
            // Overwrite the AIR ID.
            proof_input.per_air[i].0 = i;
        }
        let e = BabyBearPoseidon2RootEngine::new(*self.fri_params());
        e.prove(
            &self.minimal_verifier_pk.root_verifier_pk.vm_pk.vm_pk,
            proof_input,
        )
    }
}
