use afs_stark_backend::config::Com;
use p3_field::{PrimeField32, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::MAX_COMMITMENT_LEN;

use super::Page2Controller;

impl<SC: StarkGenericConfig> Page2Controller<SC>
where
    Com<SC>: Into<[Val<SC>; MAX_COMMITMENT_LEN]>,
    Val<SC>: PrimeField32,
{
    pub fn generate_trace<F: PrimeField64>(&self) -> Vec<RowMajorMatrix<F>> {
        let mut traces = Vec::new();
        for i in 0..self.page_commitments.len() {
            let com: [Val<SC>; MAX_COMMITMENT_LEN] = self.page_commitments[i].clone().into();
            let com: Vec<u32> = com.into_iter().map(|c| c.as_canonical_u32()).collect();
            let trace: Vec<Vec<u32>> = (0..self.is_t[i].len())
                .map(|j| {
                    let mut row = com.clone();
                    row.push(
                        self.is_p[i][j]
                            * self.is_t[i][j]
                            * self.request_counts[i][j].load(std::sync::atomic::Ordering::Relaxed),
                    );
                    row.push(
                        self.is_p[i][j]
                            * (1 - self.is_t[i][j])
                            * self.request_counts[i][j].load(std::sync::atomic::Ordering::Relaxed),
                    );
                    row.push(
                        self.is_p[i][j]
                            * self.request_counts[i][j].load(std::sync::atomic::Ordering::Relaxed),
                    );
                    row.push(self.request_counts[i][j].load(std::sync::atomic::Ordering::Relaxed));
                    println!("row is {:?}", row);
                    row
                })
                .collect();
            let trace = trace
                .into_iter()
                .flat_map(|row| row.into_iter().map(F::from_canonical_u32))
                .collect();
            traces.push(RowMajorMatrix::new(trace, MAX_COMMITMENT_LEN + 4));
        }
        traces
    }
}
