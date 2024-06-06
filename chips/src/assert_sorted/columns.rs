use afs_derive::AlignedBorrow;

use crate::{
    is_equal::columns::IsEqualAuxCols, is_less_than::columns::IsLessThanAuxCols,
    is_less_than_tuple::columns::IsLessThanTupleAuxCols,
};

// Since AssertSortedChip contains a LessThanChip subchip, a subset of the columns are those of the
// LessThanChip
#[derive(AlignedBorrow)]
pub struct AssertSortedCols<T> {
    pub key: Vec<T>,
    pub less_than_next_key: T,
    pub is_less_than_tuple_aux: IsLessThanTupleAuxCols<T>,
}

impl<T: Clone> AssertSortedCols<T> {
    pub fn from_slice(slc: &[T], limb_bits: Vec<usize>, decomp: usize, key_vec_len: usize) -> Self {
        let mut curr_start_idx = 0;
        let mut curr_end_idx = key_vec_len;

        // the first key_vec_len elements are the key itself
        let key = slc[curr_start_idx..curr_end_idx].to_vec();

        curr_start_idx = curr_end_idx;
        curr_end_idx += 1;

        // the next element is the indicator for whether the key is less than the next key
        let less_than_next_key = slc[curr_start_idx].clone();
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements are the indicators for the individual tuple element less thans
        let less_than = slc[curr_start_idx..curr_end_idx].to_vec();
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements are the values of the lower bits of each intermediate sum
        // (i.e. 2^limb_bits[i] + y[i] - x[i] - 1)
        let lower_vec = slc[curr_start_idx..curr_end_idx].to_vec();

        // the next elements are the decomposed lower bits
        let mut lower_decomp_vec: Vec<Vec<T>> = vec![];
        for curr_limb_bits in limb_bits.iter() {
            let num_limbs = (curr_limb_bits + decomp - 1) / decomp;

            curr_start_idx = curr_end_idx;
            curr_end_idx += num_limbs + 1;

            lower_decomp_vec.push(slc[curr_start_idx..curr_end_idx].to_vec());
        }
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements are the indicator whether the difference is zero; if difference is
        // zero then the two limbs must be equal
        let is_equal = slc[curr_start_idx..curr_end_idx].to_vec();
        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        // the next key_vec_len elements contain the inverses of the corresponding sum of diff and is_zero;
        // note that this sum will always be nonzero so the inverse will exist
        let inverses = slc[curr_start_idx..curr_end_idx].to_vec();

        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        let mut less_than_aux: Vec<IsLessThanAuxCols<T>> = vec![];
        for i in 0..key_vec_len {
            let less_than_col = IsLessThanAuxCols {
                lower: lower_vec[i].clone(),
                lower_decomp: lower_decomp_vec[i].clone(),
            };
            less_than_aux.push(less_than_col);
        }

        let mut is_equal_aux: Vec<IsEqualAuxCols<T>> = vec![];
        for inv in inverses.iter() {
            let is_equal_col = IsEqualAuxCols { inv: inv.clone() };
            is_equal_aux.push(is_equal_col);
        }

        let mut is_equal_cumulative: Vec<T> = vec![];
        let mut less_than_cumulative: Vec<T> = vec![];

        is_equal_cumulative.extend_from_slice(&slc[curr_start_idx..curr_end_idx]);

        curr_start_idx = curr_end_idx;
        curr_end_idx += key_vec_len;

        less_than_cumulative.extend_from_slice(&slc[curr_start_idx..curr_end_idx]);

        let is_less_than_tuple_aux = IsLessThanTupleAuxCols {
            less_than,
            less_than_aux,
            is_equal,
            is_equal_aux,
            is_equal_cumulative,
            less_than_cumulative,
        };

        Self {
            key,
            less_than_next_key,
            is_less_than_tuple_aux,
        }
    }

    pub fn get_width(limb_bits: Vec<usize>, decomp: usize, key_vec_len: usize) -> usize {
        // there are (limb_bits + decomp - 1) / decomp sublimbs per limb, we add 1 to
        // account for the sublimb itself, and another 1 to account for the shifted
        // last sublimb
        let mut width = 0;
        
        // for the key itself
        width += key_vec_len;

        // for the less than next key indicator
        width += 1;

        // for the less_than indicators
        width += key_vec_len;

        // for the lowers
        width += key_vec_len;

        // for the decomposed lowers
        for &limb_bit in limb_bits.iter() {
            let num_limbs = (limb_bit + decomp - 1) / decomp;
            width += num_limbs + 1;
        }

        // for the is_equal indicators
        width += key_vec_len;

        // for the inverses
        width += key_vec_len;

        // for the cumulative is_equal and less_than
        width += 2 * key_vec_len;

        width
    }
}
