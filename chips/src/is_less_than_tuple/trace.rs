use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use crate::{is_less_than::IsLessThanChip, sub_chip::LocalTraceInstructions};

use super::{
    columns::{IsLessThanTupleAuxCols, IsLessThanTupleCols, IsLessThanTupleIOCols},
    IsLessThanTupleChip,
};

impl IsLessThanTupleChip {
    pub fn generate_trace<F: PrimeField64>(&self, x: Vec<u32>, y: Vec<u32>) -> RowMajorMatrix<F> {
        let num_cols: usize = IsLessThanTupleCols::<F>::get_width(
            self.air.limb_bits().clone(),
            *self.air.decomp(),
            self.air.tuple_len(),
        );

        let row: Vec<F> = self.generate_trace_row((x, y)).flatten();

        RowMajorMatrix::new(row, num_cols)
    }
}

impl<F: PrimeField64> LocalTraceInstructions<F> for IsLessThanTupleChip {
    type LocalInput = (Vec<u32>, Vec<u32>);

    fn generate_trace_row(&self, input: Self::LocalInput) -> Self::Cols<F> {
        let (x, y) = input;

        let mut less_than: Vec<F> = vec![];
        let mut lower_bits: Vec<F> = vec![];
        let mut lower_bits_decomp: Vec<Vec<F>> = vec![];

        let mut valid = true;
        let mut tuple_less_than = F::zero();

        // use subchip to generate relevant columns
        for i in 0..x.len() {
            let is_less_than_chip = IsLessThanChip::new(
                *self.air.bus_index(),
                *self.air.range_max(),
                self.air.limb_bits()[i],
                *self.air.decomp(),
                self.range_checker.clone(),
            );

            let curr_less_than_row =
                LocalTraceInstructions::generate_trace_row(&is_less_than_chip, (x[i], y[i]))
                    .flatten();
            less_than.push(curr_less_than_row[2]);
            lower_bits.push(curr_less_than_row[3]);
            lower_bits_decomp.push(curr_less_than_row[4..].to_vec());
        }

        // compute whether the x < y
        for i in (0..x.len()).rev() {
            if x[i] < y[i] && valid {
                tuple_less_than = F::one();
            } else if x[i] > y[i] && valid {
                valid = false;
            }
        }

        // contains the difference between consecutive rows
        let mut diff: Vec<F> = vec![];
        // contains indicator whether difference is zero
        let mut is_zero: Vec<F> = vec![];
        // contains y such that y * (i + x) = 1
        let mut inverses: Vec<F> = vec![];

        // we compute the indicators, which only matter if the row is not the last
        for (i, &val) in x.iter().enumerate() {
            let next_val = y[i];

            // the difference between the two limbs
            let curr_diff = F::from_canonical_u32(next_val) - F::from_canonical_u32(val);
            diff.push(curr_diff);

            // compute the equal indicator and inverses
            if next_val == val {
                is_zero.push(F::one());
                inverses.push((curr_diff + F::one()).inverse());
            } else {
                is_zero.push(F::zero());
                inverses.push(curr_diff.inverse());
            }
        }

        let io = IsLessThanTupleIOCols {
            x: x.into_iter().map(F::from_canonical_u32).collect(),
            y: y.into_iter().map(F::from_canonical_u32).collect(),
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

        IsLessThanTupleCols { io, aux }
    }
}
