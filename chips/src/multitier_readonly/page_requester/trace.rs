use std::sync::Arc;

use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use crate::multitier_readonly::page_controller::PageController;

use super::PageRequester;

impl PageRequester {
    pub fn generate_trace<F: PrimeField64, const COMMITMENT_LEN: usize>(
        &self,
        requests: Vec<Vec<u32>>,
        page_controller: Arc<&mut PageController<COMMITMENT_LEN>>,
    ) -> RowMajorMatrix<F> {
        let mut rows = vec![];
        for idx in requests.iter() {
            let mut row = vec![];

            let data = page_controller.request(idx);
            row.extend(idx.clone());
            row.extend(data.unwrap());
            let row: Vec<F> = row.into_iter().map(|u| F::from_canonical_u32(u)).collect();
            rows.push(row);
        }

        RowMajorMatrix::new(rows.concat(), self.idx_len + self.data_len)
    }
}
