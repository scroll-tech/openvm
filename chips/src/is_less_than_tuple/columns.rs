use afs_derive::AlignedBorrow;

#[derive(Default, AlignedBorrow)]
pub struct IsLessThanTupleIOCols<T> {
    pub x: Vec<T>,
    pub y: Vec<T>,
    pub tuple_less_than: T,
}

pub struct IsLessThanTupleAuxCols<T> {
    pub less_than: Vec<T>,
    pub lower_bits: Vec<T>,
    pub lower_bits_decomp: Vec<Vec<T>>,
    pub diff: Vec<T>,
    pub is_zero: Vec<T>,
    pub inverses: Vec<T>,
}

pub struct IsLessThanTupleCols<T> {
    pub io: IsLessThanTupleIOCols<T>,
    pub aux: IsLessThanTupleAuxCols<T>,
}

impl<T: Clone> IsLessThanTupleCols<T> {
    pub fn from_slice(slc: &[T], limb_bits: Vec<usize>, decomp: usize, tuple_len: usize) -> Self {
        let mut x: Vec<T> = vec![];
        let mut y: Vec<T> = vec![];

        let mut less_than: Vec<T> = vec![];
        let mut lower_bits: Vec<T> = vec![];
        let mut lower_bits_decomp: Vec<Vec<T>> = vec![];
        let mut diff: Vec<T> = vec![];
        let mut is_zero: Vec<T> = vec![];
        let mut inverses: Vec<T> = vec![];

        let mut curr_start_idx = 0;
        let mut curr_end_idx = tuple_len;

        // get the actual tuples, which are x and y
        x.extend_from_slice(&slc[curr_start_idx..curr_end_idx]);

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        y.extend_from_slice(&slc[curr_start_idx..curr_end_idx]);

        curr_start_idx = curr_end_idx;
        curr_end_idx += 1;

        // get the indicator for whether x < y, lexicographically
        let tuple_less_than = slc[curr_start_idx].clone();

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        // get the indicators for whether x[i] < y[i] for all indices
        less_than.extend_from_slice(&slc[curr_start_idx..curr_end_idx]);

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        // get the lower bits for each 2^limb_bits[i] + y[i] - x[i] - 1
        lower_bits.extend_from_slice(&slc[curr_start_idx..curr_end_idx]);

        // get the lower bits decompositions
        for &limb_bit in limb_bits.iter() {
            let num_limbs = (limb_bit + decomp - 1) / decomp;
            curr_start_idx = curr_end_idx;
            curr_end_idx += num_limbs + 1;

            let mut lower_bits_curr: Vec<T> = vec![];

            for j in 0..(num_limbs + 1) {
                lower_bits_curr.push(slc[curr_start_idx + j].clone());
            }

            lower_bits_decomp.push(lower_bits_curr);
        }

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        // get the differences y[i] - x[i]
        diff.extend_from_slice(&slc[curr_start_idx..curr_end_idx]);

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        // get whether y[i] - x[i] == 0
        is_zero.extend_from_slice(&slc[curr_start_idx..curr_end_idx]);

        curr_start_idx = curr_end_idx;
        curr_end_idx += tuple_len;

        // get the inverses k such that k * (diff[i] + is_zero[i]) = 1
        inverses.extend_from_slice(&slc[curr_start_idx..curr_end_idx]);

        IsLessThanTupleCols {
            io: IsLessThanTupleIOCols {
                x,
                y,
                tuple_less_than,
            },
            aux: IsLessThanTupleAuxCols {
                less_than,
                lower_bits,
                lower_bits_decomp,
                diff,
                is_zero,
                inverses,
            },
        }
    }

    pub fn flatten(&self) -> Vec<T> {
        let mut flattened = vec![];
        flattened.extend_from_slice(&self.io.x);
        flattened.extend_from_slice(&self.io.y);
        flattened.push(self.io.tuple_less_than.clone());
        flattened.extend_from_slice(&self.aux.less_than);
        flattened.extend_from_slice(&self.aux.lower_bits);
        for i in 0..self.aux.lower_bits_decomp.len() {
            flattened.extend_from_slice(&self.aux.lower_bits_decomp[i]);
        }
        flattened.extend_from_slice(&self.aux.diff);
        flattened.extend_from_slice(&self.aux.is_zero);
        flattened.extend_from_slice(&self.aux.inverses);

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
        // for the lower bits
        width += tuple_len;

        // for the lower bits decomposition
        for &limb_bit in limb_bits.iter() {
            let num_limbs = (limb_bit + decomp - 1) / decomp;
            width += num_limbs + 1;
        }

        // for the difference between consecutive rows
        width += tuple_len;
        // for the indicator whether difference is zero
        width += tuple_len;
        // for the inverses k such that k * (diff[i] + is_zero[i]) = 1
        width += tuple_len;

        width
    }
}
