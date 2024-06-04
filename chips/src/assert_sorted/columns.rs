use afs_derive::AlignedBorrow;

use crate::is_less_than_tuple::columns::{
    IsLessThanTupleAuxCols, IsLessThanTupleCols, IsLessThanTupleIOCols,
};

// Since AssertSortedChip contains a LessThanChip subchip, a subset of the columns are those of the
// LessThanChip
#[derive(AlignedBorrow)]
pub struct AssertSortedCols<T> {
    pub keys_decomp: Vec<Vec<T>>,
    pub is_less_than_tuple_cols: IsLessThanTupleCols<T>,
}

impl<T: Clone> AssertSortedCols<T> {
    pub fn from_slice(slc: &[T], limb_bits: Vec<usize>, decomp: usize, key_vec_len: usize) -> Self {
        // num_limbs is the number of sublimbs per limb, not including the shifted last sublimb
        let mut curr_start_idx = 0;
        let mut curr_end_idx = key_vec_len;

        // the first key_vec_len elements are the key itself
        let x = slc[curr_start_idx..curr_end_idx].to_vec();
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements are the next key (the following row)
        let y = slc[curr_start_idx..curr_end_idx].to_vec();

        // the next elements are the decomposed key (with each having an extra shifted last sublimb)
        let mut keys_decomp: Vec<Vec<T>> = vec![];

        for curr_limb_bits in limb_bits.iter() {
            let num_limbs = (curr_limb_bits + decomp - 1) / decomp;

            curr_start_idx = curr_end_idx;
            curr_end_idx += num_limbs + 1;

            keys_decomp.push(slc[curr_start_idx..curr_end_idx].to_vec());
        }

        curr_start_idx = curr_end_idx;
        curr_end_idx += 1;

        // the next element is the indicator for whether the key is less than the next key
        let tuple_less_than = slc[curr_start_idx].clone();
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements are the indicators for the individual tuple element less thans
        let less_than = slc[curr_start_idx..curr_end_idx].to_vec();
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements are the values of the lower bits of each intermediate sum
        // (i.e. 2^limb_bits[i] + y[i] - x[i] - 1)
        let lower_bits = slc[curr_start_idx..curr_end_idx].to_vec();

        // the next elements are the decomposed lower bits
        let mut lower_bits_decomp: Vec<Vec<T>> = vec![];
        for curr_limb_bits in limb_bits.iter() {
            let num_limbs = (curr_limb_bits + decomp - 1) / decomp;

            curr_start_idx = curr_end_idx;
            curr_end_idx += num_limbs + 1;

            lower_bits_decomp.push(slc[curr_start_idx..curr_end_idx].to_vec());
        }
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements are the difference between consecutive limbs of rows
        let diff = slc[curr_start_idx..curr_end_idx].to_vec();
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements are the indicator whether the difference is zero; if difference is
        // zero then the two limbs must be equal
        let is_zero = slc[curr_start_idx..curr_end_idx].to_vec();
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements contain the inverses of the corresponding sum of diff and is_zero;
        // note that this sum will always be nonzero so the inverse will exist
        let inverses = slc[curr_start_idx..curr_end_idx].to_vec();

        let io = IsLessThanTupleIOCols {
            x,
            y,
            tuple_less_than,
        };
        let aux = IsLessThanTupleAuxCols {
            less_than,
            lower_bits,
            lower_bits_decomp,
            diff,
            is_zero,
            inverses,
        };

        let is_less_than_tuple_cols = IsLessThanTupleCols { io, aux };

        Self {
            keys_decomp,
            is_less_than_tuple_cols,
        }
    }

    pub fn get_width(limb_bits: Vec<usize>, decomp: usize, key_vec_len: usize) -> usize {
        // there are (limb_bits + decomp - 1) / decomp sublimbs per limb, we add 1 to
        // account for the sublimb itself, and another 1 to account for the shifted
        // last sublimb
        let mut width = 0;
        // for the x and y keys
        width += 2 * key_vec_len;

        // for the decomposed key
        for &limb_bit in limb_bits.iter() {
            let num_limbs = (limb_bit + decomp - 1) / decomp;
            width += num_limbs + 1;
        }

        // for the tuple less than indicator
        width += 1;

        // for the less_than indicators
        width += key_vec_len;

        // for the lower_bits
        width += key_vec_len;

        // for the decomposed lower_bits
        for &limb_bit in limb_bits.iter() {
            let num_limbs = (limb_bit + decomp - 1) / decomp;
            width += num_limbs + 1;
        }

        // for the difference between consecutive rows
        width += key_vec_len;
        // for the indicator whether difference is zero
        width += key_vec_len;
        // for the y such that y * (i + x) = 1
        width += key_vec_len;

        width
    }
}
