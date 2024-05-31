use crate::range_gate::RangeCheckerGateChip;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

/**
 * This Chip constrains that consecutive rows are sorted lexicographically.
 *
 * Each row consists of a key decomposed into limbs, and the chip constrains
 * each limb has at most limb_bits bits, where limb_bits is at most 31. It
 * does this by interacting with a RangeCheckerGateChip. Because the range checker
 * gate can take MAX up to 2^20, we further decompose each limb into sublimbs
 * of size decomp bits.
 */
#[derive(Default)]
pub struct LessThanChip<const MAX: u32> {
    bus_index: usize,
    limb_bits: usize,
    decomp: usize,
    key_vec_len: usize,
    keys: Vec<Vec<u32>>,

    pub range_checker_gate: RangeCheckerGateChip<MAX>,
}

impl<const MAX: u32> LessThanChip<MAX> {
    pub fn new(
        bus_index: usize,
        limb_bits: usize,
        decomp: usize,
        key_vec_len: usize,
        keys: Vec<Vec<u32>>,
    ) -> Self {
        Self {
            bus_index,
            limb_bits,
            decomp,
            key_vec_len,
            keys,
            range_checker_gate: RangeCheckerGateChip::<MAX>::new(bus_index),
        }
    }

    pub fn bus_index(&self) -> usize {
        self.bus_index
    }

    pub fn limb_bits(&self) -> usize {
        self.limb_bits
    }

    pub fn decomp(&self) -> usize {
        self.decomp
    }

    pub fn key_vec_len(&self) -> usize {
        self.key_vec_len
    }

    pub fn keys(&self) -> Vec<Vec<u32>> {
        self.keys.clone()
    }
}
