use std::array;

use ax_circuit_primitives::encoder::Encoder;
use ax_stark_backend::interaction::InteractionBuilder;
use p3_field::AbstractField;
use rand::{rngs::StdRng, Rng};

use super::{Sha256DigestCols, Sha256RoundCols};

// ==== Do not change these constants! ====
/// Number of bits in a SHA256 word
pub const SHA256_WORD_BITS: usize = 32;
/// Number of 16-bit limbs in a SHA256 word
pub const SHA256_WORD_U16S: usize = SHA256_WORD_BITS / 16;
/// Number of 8-bit limbs in a SHA256 word
pub const SHA256_WORD_U8S: usize = SHA256_WORD_BITS / 8;
/// Number of words in a SHA256 block
pub const SHA256_BLOCK_WORDS: usize = 16;
/// Number of bits in a SHA256 block
pub const SHA256_BLOCK_BITS: usize = SHA256_BLOCK_WORDS * SHA256_WORD_BITS;
/// Number of rows per block
pub const SHA256_ROWS_PER_BLOCK: usize = 17;
/// Number of rounds per row
pub const SHA256_ROUNDS_PER_ROW: usize = 4;
/// Number of words in a SHA256 hash
pub const SHA256_HASH_WORDS: usize = 8;
/// Number of vars needed to encode the row index with [Encoder]
pub const SHA256_ROW_VAR_CNT: usize = 5;
/// Width of the Sha256RoundCols
pub const SHA256_ROUND_WIDTH: usize = Sha256RoundCols::<u8>::width();
/// Width of the Sha256DigestCols
pub const SHA256_DIGEST_WIDTH: usize = Sha256DigestCols::<u8>::width();
/// Width of the Sha256Cols
pub const SHA256_WIDTH: usize = if SHA256_ROUND_WIDTH > SHA256_DIGEST_WIDTH {
    SHA256_ROUND_WIDTH
} else {
    SHA256_DIGEST_WIDTH
};
/// SHA256 constant K's
pub const SHA256_K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

/// SHA256 initial hash values
pub const SHA256_H: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

/// Convert a u32 into a list of limbs in little endian
pub fn u32_into_limbs<const NUM_LIMBS: usize>(num: u32) -> [u32; NUM_LIMBS] {
    let limb_bits = 32 / NUM_LIMBS;
    array::from_fn(|i| (num >> (limb_bits * i)) & ((1 << limb_bits) - 1))
}

/// Convert a list of limbs in little endian into a u32
pub fn limbs_into_u32<const NUM_LIMBS: usize>(limbs: [u32; NUM_LIMBS]) -> u32 {
    let limb_bits = 32 / NUM_LIMBS;
    limbs
        .iter()
        .fold(0, |acc, &limb| acc | (acc << limb_bits) | limb)
}

/// Choose function from SHA256
pub fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ ((!x) & z)
}

/// Majority function from SHA256
pub fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

/// Big sigma_0 function from SHA256
pub fn big_sig0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

/// Big sigma_1 function from SHA256
pub fn big_sig1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

/// Small sigma_0 function from SHA256
pub fn small_sig0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

/// Small sigma_1 function from SHA256
pub fn small_sig1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

/// Generate a random message of a given length
pub fn get_random_message(rng: &mut StdRng, len: usize) -> Vec<u8> {
    let mut random_message: Vec<u8> = vec![0u8; len];
    rng.fill(&mut random_message[..]);
    random_message
}

/// Composes a list of limb values into a single field element
#[inline]
pub fn compose<AB: InteractionBuilder>(a: &[AB::Expr], limb_size: usize) -> AB::Expr {
    a.iter().enumerate().fold(AB::Expr::ZERO, |acc, (i, x)| {
        acc + x.clone() * AB::Expr::from_canonical_usize(1 << (i * limb_size))
    })
}

/// Wrapper of `get_flag_pt` to get the flag pointer as an array
pub fn get_flag_pt_array<const N: usize>(encoder: &Encoder, flag_idx: usize) -> [u32; N] {
    encoder.get_flag_pt(flag_idx).try_into().unwrap()
}

/// TODO: make this as a method of [Encoder]
/// Returns an expression that is one if `flag_idxs` contains the encoded flag
pub fn contains_flag<AB: InteractionBuilder>(
    encoder: &Encoder,
    vars: &[AB::Var],
    flag_idxs: &[usize],
) -> AB::Expr {
    flag_idxs.into_iter().fold(AB::Expr::ZERO, |acc, flag_idx| {
        acc + encoder.get_flag_expr::<AB>(*flag_idx, vars)
    })
}

/// TODO: make this as a method of [Encoder]
/// Returns an expression that is zero if `flag_idxs` doesn't contain the encoded flag
/// and the corresponding value if it does
pub fn flag_with_val<AB: InteractionBuilder>(
    encoder: &Encoder,
    vars: &[AB::Var],
    flag_idxs: &[(usize, usize)],
) -> AB::Expr {
    flag_idxs
        .into_iter()
        .fold(AB::Expr::ZERO, |acc, (flag_idx, val)| {
            acc + encoder.get_flag_expr::<AB>(*flag_idx, vars)
                * AB::Expr::from_canonical_usize(*val)
        })
}
