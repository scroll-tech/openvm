use std::sync::Arc;

use crate::{is_less_than_tuple::IsLessThanTupleAir, range_gate::RangeCheckerGateChip};
use getset::Getters;

#[cfg(test)]
pub mod tests;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

#[derive(Default, Getters)]
pub struct AssertSortedAir {
    #[getset(get = "pub")]
    is_less_than_tuple_air: IsLessThanTupleAir,
    // The keys to check for sortedness
    #[getset(get = "pub")]
    keys: Vec<Vec<u32>>,
}

/**
 * This Chip constrains that consecutive rows are sorted lexicographically.
 *
 * Each row consists of a key decomposed into limbs, and the chip constrains
 * each limb has at most limb_bits bits, where limb_bits is at most 31. It
 * does this by interacting with a RangeCheckerGateChip. Because the range checker
 * gate can take MAX up to 2^20, we further decompose each limb into sublimbs
 * of size decomp bits.
 *
 * The AssertSortedChip contains a LessThanChip subchip, which is used to constrain
 * that the rows are sorted lexicographically.
 */
#[derive(Default)]
pub struct AssertSortedChip {
    air: AssertSortedAir,
    range_checker: Arc<RangeCheckerGateChip>,
}

impl AssertSortedChip {
    pub fn new(
        bus_index: usize,
        range_max: u32,
        limb_bits: Vec<usize>,
        decomp: usize,
        keys: Vec<Vec<u32>>,
        range_checker: Arc<RangeCheckerGateChip>,
    ) -> Self {
        Self {
            air: AssertSortedAir {
                is_less_than_tuple_air: IsLessThanTupleAir::new(
                    bus_index, range_max, limb_bits, decomp,
                ),
                keys,
            },
            range_checker,
        }
    }
}
