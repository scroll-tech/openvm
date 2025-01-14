use derivative::Derivative;
use openvm_stark_backend::{
    config::{Com, StarkGenericConfig, Val},
    prover::types::Proof,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Derivative)]
#[serde(bound = "")]
#[derivative(Clone(bound = "Com<SC>: Clone"))]
pub struct MinimalVmVerifierInput<SC: StarkGenericConfig> {
    /// Leaf verifier proof
    pub proof: Proof<SC>,
    pub public_values: Vec<Val<SC>>,
}
