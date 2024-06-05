use std::sync::Arc;

use getset::Getters;

use crate::{is_less_than::IsLessThanAir, range_gate::RangeCheckerGateChip};

#[cfg(test)]
pub mod tests;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

#[derive(Default, Getters)]
pub struct IsLessThanTupleAir {
    // The bus index for sends to range chip
    #[getset(get = "pub")]
    bus_index: usize,
    // The maximum range for the range checker
    #[getset(get = "pub")]
    range_max: u32,
    // The number of bits to decompose each number into, for less than checking
    #[getset(get = "pub")]
    decomp: usize,
    // IsLessThanAirs for each tuple element
    #[getset(get = "pub")]
    is_lt_airs: Vec<IsLessThanAir>,
}

impl IsLessThanTupleAir {
    pub fn tuple_len(&self) -> usize {
        self.is_lt_airs.len()
    }

    pub fn limb_bits(&self) -> Vec<usize> {
        self.is_lt_airs.iter().map(|air| *air.limb_bits()).collect()
    }
}

/**
 * This Chip constrains that consecutive rows are sorted lexicographically.
 *
 * Each row consists of a key decomposed into limbs with at most limb_bits bits
 */
#[derive(Default, Getters)]
pub struct IsLessThanTupleChip {
    pub air: IsLessThanTupleAir,

    pub range_checker: Arc<RangeCheckerGateChip>,
}

impl IsLessThanTupleChip {
    pub fn new(
        bus_index: usize,
        range_max: u32,
        limb_bits: Vec<usize>,
        decomp: usize,
        range_checker: Arc<RangeCheckerGateChip>,
    ) -> Self {
        let is_lt_airs = limb_bits
            .iter()
            .map(|&limb_bit| IsLessThanAir::new(bus_index, range_max, limb_bit, decomp))
            .collect::<Vec<_>>();

        let air = IsLessThanTupleAir {
            bus_index,
            range_max,
            decomp,
            is_lt_airs,
        };

        Self { air, range_checker }
    }
}
