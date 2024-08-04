use p3_baby_bear::BabyBear;
use p3_challenger::{HashChallenger, SerializingChallenger32};
use p3_commit::ExtensionMmcs;
use p3_dft::Radix2DitParallel;
use p3_field::extension::BinomialExtensionField;
use p3_fri::{FriConfig, TwoAdicFriPcs};
use p3_merkle_tree::FieldMerkleTreeMmcs;
use p3_sha256::{Sha256, Sha256Compress};
use p3_symmetric::SerializingHasher32;
use p3_uni_stark::StarkConfig;

use super::{fri_params::default_fri_params, FriParameters};
use crate::engine::StarkEngine;

type Val = BabyBear;
type Challenge = BinomialExtensionField<Val, 4>;

type FieldHash = SerializingHasher32<Sha256>;
type Compress = Sha256Compress;

type ValMmcs = FieldMerkleTreeMmcs<Val, u8, FieldHash, Compress, 32>;
type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
type Dft = Radix2DitParallel;
type Challenger = SerializingChallenger32<Val, HashChallenger<u8, Sha256, 32>>;

type Pcs = TwoAdicFriPcs<Val, Dft, ValMmcs, ChallengeMmcs>;

pub type BabyBearSha256CompressionConfig = StarkConfig<Pcs, Challenge, Challenger>;

/// Engine where the compression function for tree hash is SHA-256 without padding.
/// The hasher for leaf values and challenges is the standard SHA-256 with padding.
pub struct BabyBearSha256CompressionEngine {
    pub fri_params: FriParameters,
    pub config: BabyBearSha256CompressionConfig,
}

impl StarkEngine<BabyBearSha256CompressionConfig> for BabyBearSha256CompressionEngine {
    fn config(&self) -> &BabyBearSha256CompressionConfig {
        &self.config
    }

    fn new_challenger(&self) -> Challenger {
        Challenger::from_hasher(vec![], Sha256 {})
    }
}

/// `pcs_log_degree` is the upper bound on the log_2(PCS polynomial degree).
pub fn default_engine() -> BabyBearSha256CompressionEngine {
    let fri_params = default_fri_params();
    engine_from_fri_params(fri_params)
}

/// `pcs_log_degree` is the upper bound on the log_2(PCS polynomial degree).
pub fn default_config() -> BabyBearSha256CompressionConfig {
    let fri_params = default_fri_params();
    config_from_fri_params(fri_params)
}

pub fn engine_from_fri_params(fri_params: FriParameters) -> BabyBearSha256CompressionEngine {
    let config = config_from_fri_params(fri_params);
    BabyBearSha256CompressionEngine { config, fri_params }
}

pub fn config_from_fri_params(fri_params: FriParameters) -> BabyBearSha256CompressionConfig {
    let field_hash = FieldHash::new(Sha256 {});
    let compress = Sha256Compress;
    let val_mmcs = ValMmcs::new(field_hash, compress);
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs);
    let dft = Dft {};
    let fri_config = FriConfig {
        log_blowup: fri_params.log_blowup,
        num_queries: fri_params.num_queries,
        proof_of_work_bits: fri_params.proof_of_work_bits,
        mmcs: challenge_mmcs,
    };
    let pcs = Pcs::new(dft, val_mmcs, fri_config);
    BabyBearSha256CompressionConfig::new(pcs)
}
