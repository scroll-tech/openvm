use afs_derive::AlignedBorrow;

use crate::{is_equal::columns::IsEqualAuxCols, is_less_than::columns::IsLessThanAuxCols};

#[derive(Default, AlignedBorrow)]
pub struct IsLessThanTupleIOCols<T> {
    pub x: Vec<T>,
    pub y: Vec<T>,
    pub tuple_less_than: T,
}

pub struct IsLessThanTupleAuxCols<T> {
    pub less_than: Vec<T>,
    pub less_than_aux: Vec<IsLessThanAuxCols<T>>,
    pub is_equal: Vec<T>,
    pub is_equal_aux: Vec<IsEqualAuxCols<T>>,

    pub is_equal_cumulative: Vec<T>,
    pub less_than_cumulative: Vec<T>,
}

pub struct IsLessThanTupleCols<T> {
    pub io: IsLessThanTupleIOCols<T>,
    pub aux: IsLessThanTupleAuxCols<T>,
}

impl<T: Clone> IsLessThanTupleCols<T> {
    pub fn from_slice(slc: &[T], limb_bits: Vec<usize>, decomp: usize, tuple_len: usize) -> Self {
        let mut curr_start_idx = 0;
        let mut curr_end_idx = tuple_len;

        // get the actual tuples, which are x and y
        let x = slc[curr_start_idx..curr_end_idx].to_vec();

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        let y = slc[curr_start_idx..curr_end_idx].to_vec();

        curr_start_idx = curr_end_idx;
        curr_end_idx += 1;

        // get the indicator for whether x < y, lexicographically
        let tuple_less_than = slc[curr_start_idx].clone();

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        // get the indicators for whether x[i] < y[i] for all indices
        let less_than = slc[curr_start_idx..curr_end_idx].to_vec();

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        // get the lower bits for each 2^limb_bits[i] + y[i] - x[i] - 1
        let lower_vec = slc[curr_start_idx..curr_end_idx].to_vec();

        // get the lower bits decompositions
        let mut lower_decomp_vec: Vec<Vec<T>> = vec![];

        for &limb_bit in limb_bits.iter() {
            let num_limbs = (limb_bit + decomp - 1) / decomp;
            curr_start_idx = curr_end_idx;
            curr_end_idx += num_limbs + 1;

            let mut lower_bits_curr: Vec<T> = vec![];

            for j in 0..(num_limbs + 1) {
                lower_bits_curr.push(slc[curr_start_idx + j].clone());
            }

            lower_decomp_vec.push(lower_bits_curr);
        }

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        // get whether y[i] - x[i] == 0
        let is_equal = slc[curr_start_idx..curr_end_idx].to_vec();

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        // get the inverses k such that k * (diff[i] + is_zero[i]) = 1
        let inverses = slc[curr_start_idx..curr_end_idx].to_vec();

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        let is_equal_cumulative = slc[curr_start_idx..curr_end_idx].to_vec();

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        let less_than_cumulative = slc[curr_start_idx..curr_end_idx].to_vec();

        // generate the less_than_aux and is_equal_aux columns
        let mut less_than_aux: Vec<IsLessThanAuxCols<T>> = vec![];
        for i in 0..tuple_len {
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

        IsLessThanTupleCols {
            io: IsLessThanTupleIOCols {
                x,
                y,
                tuple_less_than,
            },
            aux: IsLessThanTupleAuxCols {
                less_than,
                less_than_aux,
                is_equal,
                is_equal_aux,
                is_equal_cumulative,
                less_than_cumulative,
            },
        }
    }

    pub fn flatten(&self) -> Vec<T> {
        let mut flattened = vec![];
        flattened.extend_from_slice(&self.io.x);
        flattened.extend_from_slice(&self.io.y);
        flattened.push(self.io.tuple_less_than.clone());
        flattened.extend_from_slice(&self.aux.less_than);

        for i in 0..self.aux.less_than_aux.len() {
            flattened.push(self.aux.less_than_aux[i].lower.clone());
        }

        for i in 0..self.aux.less_than_aux.len() {
            flattened.extend_from_slice(&self.aux.less_than_aux[i].lower_decomp);
        }

        flattened.extend_from_slice(&self.aux.is_equal);

        for i in 0..self.aux.is_equal_aux.len() {
            flattened.push(self.aux.is_equal_aux[i].inv.clone());
        }

        flattened.extend_from_slice(&self.aux.is_equal_cumulative);
        flattened.extend_from_slice(&self.aux.less_than_cumulative);

        flattened
    }

    pub fn get_width(limb_bits: Vec<usize>, decomp: usize, tuple_len: usize) -> usize {
        let mut width = 0;
        // for the x and y tuples
        width += 2 * tuple_len;
        // for the tuple less than indicator
        width += 1;
        // for the less than indicator
        width += tuple_len;
        // for the lowers
        width += tuple_len;

        // for the decomposed lowers
        for &limb_bit in limb_bits.iter() {
            let num_limbs = (limb_bit + decomp - 1) / decomp;
            width += num_limbs + 1;
        }

        // for the indicator whether difference is zero
        width += tuple_len;
        // for the inverses k such that k * (diff[i] + is_zero[i]) = 1
        width += tuple_len;

        // for the cumulative is_equal and less_than
        width += 2 * tuple_len;

        width
    }
}
