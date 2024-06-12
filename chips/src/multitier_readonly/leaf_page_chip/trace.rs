use super::LeafPageChip;
use p3_field::{AbstractField, PrimeField64};
use p3_matrix::dense::{DenseMatrix, RowMajorMatrix};
use p3_uni_stark::{StarkGenericConfig, Val};

impl<const COMMITMENT_LEN: usize> LeafPageChip<COMMITMENT_LEN> {
    pub fn generate_cached_trace<SC: StarkGenericConfig>(
        &self,
        page: Vec<Vec<u32>>,
    ) -> DenseMatrix<Val<SC>>
    where
        Val<SC>: AbstractField,
    {
        assert!(!page.is_empty());

        RowMajorMatrix::new(
            page.iter()
                .flat_map(|row| row.iter().map(|r| Val::<SC>::from_wrapped_u32(*r)))
                .collect(),
            2 + self.idx_len + self.data_len,
        )
    }
    pub fn generate_main_trace<SC: StarkGenericConfig>(
        &self,
        page: Vec<Vec<u32>>,
        commit: Vec<u32>,
        mult: Vec<u32>,
    ) -> DenseMatrix<Val<SC>>
    where
        Val<SC>: AbstractField + PrimeField64,
    {
        assert!(commit.len() == COMMITMENT_LEN);
        RowMajorMatrix::new(
            page.iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    let mut trace_row = vec![];
                    trace_row.extend(commit.clone());
                    trace_row.push(mult[i]);
                    trace_row.push(mult[i] * row[1]);
                    trace_row.into_iter().map(Val::<SC>::from_wrapped_u32)
                })
                .collect(),
            2 + self.idx_len + self.data_len,
        )
    }
}
