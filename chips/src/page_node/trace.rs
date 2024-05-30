use std::cmp::max;

use crate::MAX_COMMITMENT_LEN;

use super::PageNodeReadChip;
use p3_field::AbstractField;
use p3_matrix::dense::{DenseMatrix, RowMajorMatrix};
use p3_uni_stark::{StarkGenericConfig, Val};

impl PageNodeReadChip {
    pub fn get_page_trace<SC: StarkGenericConfig>(&self) -> DenseMatrix<Val<SC>>
    where
        Val<SC>: AbstractField,
    {
        assert!(!self.page_data.is_empty());

        RowMajorMatrix::new(
            self.page_data
                .clone()
                .into_iter()
                .flat_map(|row| row.into_iter().map(Val::<SC>::from_wrapped_u32))
                .collect(),
            2 + max(
                MAX_COMMITMENT_LEN + 2 * self.key_len,
                self.key_len + self.val_len,
            ),
        )
    }
}
