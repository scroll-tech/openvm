use afs_derive::AlignedBorrow;

use crate::less_than::columns::{LessThanAuxCols, LessThanCols, LessThanIOCols};

// Since AssertSortedChip contains a LessThanChip subchip, a subset of the columns are those of the
// LessThanChip
#[derive(AlignedBorrow)]
pub struct AssertSortedCols<T> {
    pub keys_decomp: Vec<Vec<T>>,
    pub less_than_cols: LessThanCols<T>,
}

impl<T: Clone> AssertSortedCols<T> {
    pub fn from_slice(slc: &[T], limb_bits: usize, decomp: usize, key_vec_len: usize) -> Self {
        // num_limbs is the number of sublimbs per limb, not including the shifted last sublimb
        let num_limbs = (limb_bits + decomp - 1) / decomp;
        let mut cur_start_idx = 0;
        let mut cur_end_idx = key_vec_len;

        // the first key_vec_len elements are the key itself
        let key = slc[cur_start_idx..cur_end_idx].to_vec();
        cur_start_idx = cur_end_idx;
        cur_end_idx += key_vec_len * (num_limbs + 1);

        // the next key_vec_len * (num_limbs + 1) elements are the decomposed keys (with each having
        // an extra shifted last sublimb)
        let keys_decomp = slc[cur_start_idx..cur_end_idx]
            .chunks(num_limbs + 1)
            .map(|chunk| chunk.to_vec())
            .collect();
        cur_start_idx = cur_end_idx;
        cur_end_idx += key_vec_len;

        // the next key_vec_len elements are the values of the lower num_limbs bits of the intermediate sum
        let lower_bits = slc[cur_start_idx..cur_end_idx].to_vec();
        cur_start_idx = cur_end_idx;
        cur_end_idx += key_vec_len;

        // the next key_vec_len elements are the values of the upper bit of the intermediate sum; note that
        // b > a <=> upper_bit = 1
        let upper_bit = slc[cur_start_idx..cur_end_idx].to_vec();
        cur_start_idx = cur_end_idx;
        cur_end_idx += key_vec_len * (num_limbs + 1);

        // the next key_vec_len * (num_limbs + 1) elements are the decomposed limbs of the lower bits of the
        // intermediate sum
        let lower_bits_decomp = slc[cur_start_idx..cur_end_idx]
            .chunks(num_limbs + 1)
            .map(|chunk| chunk.to_vec())
            .collect();
        cur_start_idx = cur_end_idx;
        cur_end_idx += key_vec_len;

        // the next key_vec_len elements are the difference between consecutive limbs of rows
        let diff = slc[cur_start_idx..cur_end_idx].to_vec();
        cur_start_idx = cur_end_idx;
        cur_end_idx += key_vec_len;

        // the next key_vec_len elements are the indicator whether the difference is zero; if difference is
        // zero then the two limbs must be equal
        let is_zero = slc[cur_start_idx..cur_end_idx].to_vec();
        cur_start_idx = cur_end_idx;
        cur_end_idx += key_vec_len;

        // the next key_vec_len elements contain the inverses of the corresponding sum of diff and is_zero;
        // note that this sum will always be nonzero so the inverse will exist
        let inverses = slc[cur_start_idx..cur_end_idx].to_vec();

        let io = LessThanIOCols { key };
        let aux = LessThanAuxCols {
            lower_bits,
            upper_bit,
            lower_bits_decomp,
            diff,
            is_zero,
            inverses,
        };

        let less_than_cols = LessThanCols { io, aux };

        Self {
            keys_decomp,
            less_than_cols,
        }
    }

    pub fn get_width(limb_bits: usize, decomp: usize, key_vec_len: usize) -> usize {
        // there are (limb_bits + decomp - 1) / decomp sublimbs per limb, we add 1 to
        // account for the sublimb itself, and another 1 to account for the shifted
        // last sublimb
        let mut width = 0;
        // for the key itself
        width += key_vec_len;
        // for the decomposed keys
        let num_limbs = (limb_bits + decomp - 1) / decomp;
        width += key_vec_len * (num_limbs + 1);
        // for the lower_bits
        width += key_vec_len;
        // for the upper_bit
        width += key_vec_len;
        // for the decomposed lower_bits
        width += key_vec_len * (num_limbs + 1);
        // for the difference between consecutive rows
        width += key_vec_len;
        // for the indicator whether difference is zero
        width += key_vec_len;
        // for the y such that y * (i + x) = 1
        width += key_vec_len;

        width
    }
}
