use p3_matrix::dense::RowMajorMatrix; // Import the constant from columns.rs

use p3_field::PrimeField64;

use super::RootSignalChip;

impl<const COMMITMENT_LEN: usize> RootSignalChip<COMMITMENT_LEN> {
    pub fn generate_trace<F: PrimeField64>(
        &self,
        commit: Vec<u32>,
        mult: u32,
        range: (Vec<u32>, Vec<u32>),
    ) -> RowMajorMatrix<F> {
        assert!(commit.len() == COMMITMENT_LEN);
        RowMajorMatrix::new(
            {
                let mut trace_row = vec![];
                trace_row.extend(commit.clone());
                trace_row.push(mult);
                if !self.is_init {
                    trace_row.extend(range.0.clone());
                    trace_row.extend(range.1.clone());
                    trace_row
                        .into_iter()
                        .map(|i| F::from_wrapped_u32(i))
                        .collect::<Vec<F>>()
                } else {
                    trace_row
                        .into_iter()
                        .map(|i| F::from_wrapped_u32(i))
                        .collect::<Vec<F>>()
                }
            },
            self.air_width(),
        )
    }
}
