use std::marker::PhantomData;

use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_circle::CirclePcs;
use p3_commit::ExtensionMmcs;
use p3_field::extension::BinomialExtensionField;
use p3_fri::FriConfig;
use p3_merkle_tree::FieldMerkleTreeMmcs;
use p3_mersenne_31::Mersenne31;
use p3_sha256::{Sha256, Sha256Compress};
use p3_symmetric::SerializingHasher32;
use p3_uni_stark::StarkConfig;

use super::{fri_params::default_fri_params, FriParameters};
use crate::engine::StarkEngine;

type Val = Mersenne31;
type Challenge = BinomialExtensionField<Val, 3>;

type FieldHash = SerializingHasher32<Sha256>;
type Compress = Sha256Compress;

type ValMmcs = FieldMerkleTreeMmcs<Val, u8, FieldHash, Compress, 32>;
type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
type Challenger = SerializingChallenger32<Val, HashChallenger<u8, Sha256, 32>>;

type Pcs = CirclePcs<Val, ValMmcs, ChallengeMmcs>;

pub type Mersenne31Sha256CompressionConfig = StarkConfig<Pcs, Challenge, Challenger>;

/// Engine where the compression function for tree hash is SHA-256 without padding.
/// The hasher for leaf values and challenges is the standard SHA-256 with padding.
/// Uses Circle STARK as the PCS with Mersenne31 as the base field.
pub struct Mersenne31Sha256CompressionEngine {
    pub fri_params: FriParameters,
    pub config: Mersenne31Sha256CompressionConfig,
}

type Config = Mersenne31Sha256CompressionConfig;
type Engine = Mersenne31Sha256CompressionEngine;

impl StarkEngine<Config> for Engine {
    fn config(&self) -> &Config {
        &self.config
    }

    fn new_challenger(&self) -> Challenger {
        Challenger::from_hasher(vec![], Sha256 {})
    }
}

/// `pcs_log_degree` is the upper bound on the log_2(PCS polynomial degree).
pub fn default_engine() -> Engine {
    let fri_params = default_fri_params();
    engine_from_fri_params(fri_params)
}

/// `pcs_log_degree` is the upper bound on the log_2(PCS polynomial degree).
pub fn default_config() -> Config {
    let fri_params = default_fri_params();
    config_from_fri_params(fri_params)
}

pub fn engine_from_fri_params(fri_params: FriParameters) -> Engine {
    let config = config_from_fri_params(fri_params);
    Engine { config, fri_params }
}

pub fn config_from_fri_params(fri_params: FriParameters) -> Config {
    let field_hash = FieldHash::new(Sha256 {});
    let compress = Sha256Compress;
    let val_mmcs = ValMmcs::new(field_hash, compress);
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs);
    let fri_config = FriConfig {
        log_blowup: fri_params.log_blowup,
        num_queries: fri_params.num_queries,
        proof_of_work_bits: fri_params.proof_of_work_bits,
        mmcs: challenge_mmcs,
    };
    let pcs = Pcs {
        mmcs: val_mmcs,
        fri_config,
        _phantom: PhantomData,
    };
    Config::new(pcs)
}
