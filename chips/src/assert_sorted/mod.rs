use std::sync::Arc;

use crate::{is_less_than_tuple::IsLessThanTupleChip, range_gate::RangeCheckerGateChip};
use getset::Getters;

use afs_stark_backend::interaction::Interaction;
use columns::AssertSortedCols;
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

#[cfg(test)]
pub mod tests;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

#[derive(Default, Getters)]
pub struct AssertedSortedAir {
    #[getset(get = "pub")]
    bus_index: usize,
    #[getset(get = "pub")]
    range_max: u32,
    #[getset(get = "pub")]
    limb_bits: Vec<usize>,
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
    air: AssertedSortedAir,
    is_less_than_tuple_chip: IsLessThanTupleChip,
    range_checker: Arc<RangeCheckerGateChip>,
}

impl AssertSortedChip {
    pub fn new(
        bus_index: usize,
        range_max: u32,
        limb_bits: Vec<usize>,
        decomp: usize,
        key_vec_len: usize,
        keys: Vec<Vec<u32>>,
        range_checker: Arc<RangeCheckerGateChip>,
    ) -> Self {
        Self {
            air: AssertedSortedAir {
                bus_index,
                range_max,
                limb_bits: limb_bits.clone(),
                decomp,
                key_vec_len,
                keys,
            },
            is_less_than_tuple_chip: IsLessThanTupleChip::new(
                bus_index,
                range_max,
                limb_bits,
                decomp,
                range_checker.clone(),
            ),
            range_checker,
        }
    }

    pub fn sends_custom<F: PrimeField64>(
        &self,
        cols: &AssertSortedCols<usize>,
    ) -> Vec<Interaction<F>> {
        // num_limbs is the number of sublimbs per limb of key, not including the
        // shifted last sublimb
        let num_keys = *self.air.key_vec_len();

        let mut interactions = vec![];

        // we will range check the decomposed limbs of the key
        for i in 0..num_keys {
            let num_limbs = (self.air.limb_bits()[i] + *self.air.decomp() - 1) / *self.air.decomp();
            // add 1 to account for the shifted last sublimb
            for j in 0..(num_limbs + 1) {
                interactions.push(Interaction {
                    fields: vec![VirtualPairCol::single_main(cols.keys_decomp[i][j])],
                    count: VirtualPairCol::constant(F::one()),
                    argument_index: *self.air.bus_index(),
                });
            }
        }

        interactions
    }
}
