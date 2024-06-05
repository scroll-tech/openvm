use std::sync::Arc;

use crate::range_gate::RangeCheckerGateChip;
use getset::Getters;

#[cfg(test)]
pub mod tests;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

#[derive(Default, Getters)]
pub struct IsLessThanAir {
    // The bus index
    #[getset(get = "pub")]
    bus_index: usize,
    // The maximum range for the range checker
    #[getset(get = "pub")]
    range_max: u32,
    // The maximum number of bits for the numbers to compare
    #[getset(get = "pub")]
    limb_bits: usize,
    // The number of bits to decompose each number into, for less than checking
    #[getset(get = "pub")]
    decomp: usize,
    // num_limbs is the number of limbs we decompose each input into, not including the last shifted limb
    #[getset(get)]
    num_limbs: usize,
}

impl IsLessThanAir {
    pub fn new(bus_index: usize, range_max: u32, limb_bits: usize, decomp: usize) -> Self {
        Self {
            bus_index,
            range_max,
            limb_bits,
            decomp,
            num_limbs: (limb_bits + decomp - 1) / decomp,
        }
    }
}

/**
 * This chip computes whether one number is less than another.
 */
#[derive(Default, Getters)]
pub struct IsLessThanChip {
    pub air: IsLessThanAir,

    pub range_checker: Arc<RangeCheckerGateChip>,
}

impl IsLessThanChip {
    pub fn new(
        bus_index: usize,
        range_max: u32,
        limb_bits: usize,
        decomp: usize,
        range_checker: Arc<RangeCheckerGateChip>,
    ) -> Self {
        let air = IsLessThanAir {
            bus_index,
            range_max,
            limb_bits,
            decomp,
            num_limbs: (limb_bits + decomp - 1) / decomp,
        };

        Self { air, range_checker }
    }
}
