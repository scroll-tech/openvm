use openvm_native_compiler::{
    ir::{Array, Config, Ext, Felt, Usize},
    prelude::*,
};

use crate::{digest::DigestVariable, fri::types::FriProofVariable, OUTER_DIGEST_SIZE};

pub type OuterDigestVariable<C> = [Var<<C as Config>::N>; OUTER_DIGEST_SIZE];

#[derive(DslVariable, Clone)]
pub struct StarkProofVariable<C: Config> {
    pub commitments: CommitmentsVariable<C>,
    pub opening: OpeningProofVariable<C>,
    pub per_air: Array<C, AirProofDataVariable<C>>,
    /// A permutation of AIR indexes which are sorted by log_degree in descending order.
    pub air_perm_by_height: Array<C, Usize<C::N>>,
}

#[derive(DslVariable, Clone)]
pub struct AirProofDataVariable<C: Config> {
    pub air_id: Usize<C::N>,
    /// height of trace matrix.
    pub log_degree: Usize<C::N>,
    /// For each challenge phase with trace, the values to expose to the verifier in that phase
    #[allow(clippy::type_complexity)]
    pub exposed_values_after_challenge: Array<C, Array<C, Ext<C::F, C::EF>>>,
    // The public values to expose to the verifier
    pub public_values: Array<C, Felt<C::F>>,
}

#[derive(Clone)]
pub struct MultiStarkVerificationAdviceVariable<C: Config> {
    pub per_air: Array<C, StarkVerificationAdviceVariable<C>>,
    /// Shape is as same as the shape of the original VK's `num_challenges_to_sample.
    /// Each element is 0 or 1. 1 means the challenge should be sampled.
    pub num_challenges_to_sample_mask: Vec<Vec<Usize<C::N>>>,
}

#[derive(DslVariable, Clone)]
pub struct StarkVerificationAdviceVariable<C: Config> {
    /// Preprocessed trace data, if any
    pub preprocessed_data: Array<C, DigestVariable<C>>,
    /// Trace sub-matrix widths
    pub width: TraceWidthVariable<C>,
    /// The factor to multiply the trace degree by to get the degree of the quotient polynomial. Determined from the max constraint degree of the AIR constraints.
    /// This is equivalently the number of chunks the quotient polynomial is split into.
    pub log_quotient_degree: Usize<C::N>,
    /// Number of public values for this STARK only
    pub num_public_values: Usize<C::N>,
    /// For only this RAP, how many challenges are needed in each trace challenge phase
    pub num_challenges_to_sample: Array<C, Usize<C::N>>,
    /// Number of values to expose to verifier in each trace challenge phase
    pub num_exposed_values_after_challenge: Array<C, Usize<C::N>>,
}

#[derive(DslVariable, Debug, Clone)]
pub struct TraceWidthVariable<C: Config> {
    pub preprocessed: Array<C, Usize<C::N>>,
    pub cached_mains: Array<C, Usize<C::N>>,
    pub common_main: Usize<C::N>,
    /// Width counted by extension field elements, _not_ base field elements
    pub after_challenge: Array<C, Usize<C::N>>,
}

#[derive(DslVariable, Clone)]
pub struct CommitmentsVariable<C: Config> {
    pub main_trace: Array<C, DigestVariable<C>>,
    pub after_challenge: Array<C, DigestVariable<C>>,
    pub quotient: DigestVariable<C>,
}

#[derive(DslVariable, Clone)]
pub struct OpeningProofVariable<C: Config> {
    pub proof: FriProofVariable<C>,
    pub values: OpenedValuesVariable<C>,
}

#[allow(clippy::type_complexity)]
#[derive(DslVariable, Clone)]
pub struct OpenedValuesVariable<C: Config> {
    pub preprocessed: Array<C, AdjacentOpenedValuesVariable<C>>,
    pub main: Array<C, Array<C, AdjacentOpenedValuesVariable<C>>>,
    pub after_challenge: Array<C, Array<C, AdjacentOpenedValuesVariable<C>>>,
    pub quotient: Array<C, Array<C, Array<C, Ext<C::F, C::EF>>>>,
}

#[derive(DslVariable, Debug, Clone)]
pub struct AdjacentOpenedValuesVariable<C: Config> {
    pub local: Array<C, Ext<C::F, C::EF>>,
    pub next: Array<C, Ext<C::F, C::EF>>,
}
