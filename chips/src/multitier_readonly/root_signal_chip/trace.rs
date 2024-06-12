use p3_matrix::dense::RowMajorMatrix;

use p3_field::PrimeField64;

use super::RootSignalChip;

impl<const COMMITMENT_LEN: usize> RootSignalChip<COMMITMENT_LEN> {
    pub fn generate_trace<F: PrimeField64>(
        &self,
        commit: Vec<u32>,
        mult: u32,
    ) -> RowMajorMatrix<F> {
        assert!(commit.len() == COMMITMENT_LEN);
        RowMajorMatrix::new(
            {
                let mut trace_row = vec![];
                trace_row.extend(commit.clone());
                trace_row.push(mult);
                trace_row.into_iter().map(F::from_canonical_u32).collect()
            },
            self.air_width(),
        )
    }
}
