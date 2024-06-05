use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use crate::sub_chip::LocalTraceInstructions;

use super::{
    columns::{IsLessThanAuxCols, IsLessThanCols, IsLessThanIOCols},
    IsLessThanChip,
};

impl IsLessThanChip {
    pub fn generate_trace<F: PrimeField64>(&self, x: u32, y: u32) -> RowMajorMatrix<F> {
        let num_cols: usize =
            IsLessThanCols::<F>::get_width(*self.air.limb_bits(), *self.air.decomp());

        let row = self.generate_trace_row((x, y)).flatten();

        RowMajorMatrix::new(row, num_cols)
    }
}

impl<F: PrimeField64> LocalTraceInstructions<F> for IsLessThanChip {
    type LocalInput = (u32, u32);

    fn generate_trace_row(&self, input: (u32, u32)) -> Self::Cols<F> {
        let (x, y) = input;
        let less_than = self.calc_less_than(x, y);

        // to range check the last limb of the decomposed lower_bits, we need to shift it to make sure it is in
        // the correct range
        let last_limb_shift =
            (self.air.decomp() - (self.air.limb_bits() % self.air.decomp())) % self.air.decomp();

        // obtain the lower_bits
        let check_less_than = (1 << self.air.limb_bits()) + y - x - 1;
        let lower = F::from_canonical_u32(check_less_than & ((1 << self.air.limb_bits()) - 1));
        let lower_u32 = check_less_than & ((1 << self.air.limb_bits()) - 1);

        // decompose lower_bits into limbs and range check
        let mut lower_decomp: Vec<F> = vec![];
        for i in 0..*self.air.num_limbs() {
            let bits = (lower_u32 >> (i * self.air.decomp())) & ((1 << self.air.decomp()) - 1);
            lower_decomp.push(F::from_canonical_u32(bits));
            self.range_checker.add_count(bits);
        }

        // shift the last limb and range check
        let bits = (lower_u32 >> ((self.air.num_limbs() - 1) * self.air.decomp()))
            & ((1 << self.air.decomp()) - 1);
        if (bits << last_limb_shift) < *self.air.range_max() {
            self.range_checker.add_count(bits << last_limb_shift);
        }
        lower_decomp.push(F::from_canonical_u32(bits << last_limb_shift));

        let io = IsLessThanIOCols {
            x: F::from_canonical_u32(x),
            y: F::from_canonical_u32(y),
            less_than: F::from_canonical_u32(less_than),
        };
        let aux = IsLessThanAuxCols {
            lower,
            lower_decomp,
        };

        IsLessThanCols { io, aux }
    }
}
