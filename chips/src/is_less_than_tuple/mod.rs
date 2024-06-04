use getset::Getters;

use crate::is_less_than::IsLessThanChip;

#[cfg(test)]
pub mod tests;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

#[derive(Default, Getters)]
pub struct IsLessThanTupleAir {
    #[getset(get = "pub")]
    bus_index: usize,
    #[getset(get = "pub")]
    range_max: u32,
    #[getset(get = "pub")]
    limb_bits: Vec<usize>,
    #[getset(get = "pub")]
    decomp: usize,
    #[getset(get = "pub")]
    tuple_len: usize,
}

/**
 * This Chip constrains that consecutive rows are sorted lexicographically.
 *
 * Each row consists of a key decomposed into limbs with at most limb_bits bits
 */
#[derive(Default, Getters)]
pub struct IsLessThanTupleChip {
    pub air: IsLessThanTupleAir,

    pub is_less_than_chips: Vec<IsLessThanChip>,
}

impl IsLessThanTupleChip {
    pub fn new(
        bus_index: usize,
        range_max: u32,
        limb_bits: Vec<usize>,
        decomp: usize,
        tuple_len: usize,
    ) -> Self {
        let air = IsLessThanTupleAir {
            bus_index,
            range_max,
            limb_bits: limb_bits.clone(),
            decomp,
            tuple_len,
        };

        // create less_than_chips which will be used to compare individual tuple elements
        let is_less_than_chips = limb_bits
            .iter()
            .map(|&limb_bit| IsLessThanChip::new(bus_index, range_max, limb_bit, decomp))
            .collect::<Vec<_>>();

        Self {
            air,
            is_less_than_chips,
        }
    }
}
