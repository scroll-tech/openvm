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
    #[getset(get = "pub")]
    range_max: u32,
    #[getset(get = "pub")]
    limb_bits: usize,
    #[getset(get = "pub")]
    decomp: usize,
}

/**
 * This chip computes whether one number is less than another.
 */
#[derive(Default, Getters)]
pub struct IsLessThanChip {
    pub air: IsLessThanAir,

    #[getset(get = "pub")]
    bus_index: usize,

    pub range_checker_gate: RangeCheckerGateChip,
}

impl IsLessThanChip {
    pub fn new(bus_index: usize, range_max: u32, limb_bits: usize, decomp: usize) -> Self {
        let air = IsLessThanAir {
            range_max,
            limb_bits,
            decomp,
        };

        Self {
            air,
            bus_index,
            range_checker_gate: RangeCheckerGateChip::new(bus_index, range_max),
        }
    }

    fn calc_less_than(&self, x: u32, y: u32) -> u32 {
        if x < y {
            1
        } else {
            0
        }
    }
}
