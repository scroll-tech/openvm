use crate::range_gate::RangeCheckerGateChip;
use getset::Getters;

#[cfg(test)]
pub mod tests;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

#[derive(Default, Getters)]
pub struct LessThanAir<const MAX: u32> {
    #[getset(get = "pub")]
    limb_bits: usize,
    #[getset(get = "pub")]
    decomp: usize,
    #[getset(get = "pub")]
    key_vec_len: usize,
    #[getset(get = "pub")]
    keys: Vec<Vec<u32>>,
}

/**
 * This Chip constrains that consecutive rows are sorted lexicographically.
 *
 * Each row consists of a key decomposed into limbs with at most limb_bits bits
 */
#[derive(Default, Getters)]
pub struct LessThanChip<const MAX: u32> {
    pub air: LessThanAir<MAX>,

    #[getset(get = "pub")]
    bus_index: usize,

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
        let air = LessThanAir {
            limb_bits,
            decomp,
            key_vec_len,
            keys,
        };

        Self {
            air,
            bus_index,
            range_checker_gate: RangeCheckerGateChip::<MAX>::new(bus_index),
        }
    }
}
