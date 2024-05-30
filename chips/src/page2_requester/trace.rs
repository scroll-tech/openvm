use afs_stark_backend::config::Com;
use p3_field::{PrimeField32, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::{page2_controller::Page2Controller, MAX_COMMITMENT_LEN};

use super::Page2Requester;

impl Page2Requester {
    pub fn generate_trace<F: PrimeField64, SC: StarkGenericConfig>(
        &self,
        page_controller: &Page2Controller<SC>,
    ) -> RowMajorMatrix<F>
    where
        Com<SC>: Into<[Val<SC>; MAX_COMMITMENT_LEN]>,
        Val<SC>: PrimeField32,
    {
        let mut rows = vec![];
        for k in self.requests.iter() {
            let mut row = vec![];

            let mut val = page_controller.request(k);
            let mut com = page_controller.root_commitment();
            row.append(&mut k.clone());
            row.append(&mut val);
            row.append(&mut com);
            let row: Vec<F> = row.into_iter().map(|u| F::from_canonical_u32(u)).collect();
            rows.push(row);
        }

        RowMajorMatrix::new(
            rows.concat(),
            self.key_len + self.val_len + MAX_COMMITMENT_LEN,
        )
    }
}
