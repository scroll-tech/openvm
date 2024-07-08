use std::sync::Arc;

use afs_chips::{
    common::page::Page, is_less_than_tuple::columns::IsLessThanTupleCols,
    range_gate::RangeCheckerGateChip, sub_chip::LocalTraceInstructions,
};
use p3_field::{AbstractField, PrimeField, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{StarkGenericConfig, Val};

use super::FullIndexedOutputPageAir;

impl FullIndexedOutputPageAir {
    // The trace is the whole page (including the is_alloc column)
    pub fn generate_cached_trace<F: PrimeField64>(&self, page: Vec<Vec<u32>>) -> RowMajorMatrix<F> {
        RowMajorMatrix::new(
            page.into_iter()
                .flat_map(|row| row.into_iter().map(F::from_wrapped_u32).collect::<Vec<F>>())
                .collect(),
            1 + self.idx_len + self.data_len,
        )
    }

    pub fn generate_main_trace<SC: StarkGenericConfig>(
        &self,
        page: &Page,
        range: (Vec<u32>, Vec<u32>),
        range_checker: Arc<RangeCheckerGateChip>,
    ) -> RowMajorMatrix<Val<SC>>
    where
        Val<SC>: PrimeField64 + PrimeField,
    {
        let mut final_page_aux_rows = self
            .page_chip
            .gen_aux_trace::<SC>(page, range_checker.clone());
        RowMajorMatrix::new(
            page.rows
                .iter()
                .enumerate()
                .flat_map(|(i, row)| {
                    let mut trace_row = vec![];
                    trace_row.extend(range.0.clone());
                    trace_row.extend(range.1.clone());
                    trace_row.extend(vec![0; 2]);
                    let mut trace_row: Vec<Val<SC>> = trace_row
                        .into_iter()
                        .map(Val::<SC>::from_canonical_u32)
                        .collect();
                    {
                        let tuple: IsLessThanTupleCols<Val<SC>> = self
                            .is_less_than_tuple_air
                            .clone()
                            .idx_start
                            .generate_trace_row((
                                row.idx.to_vec(),
                                range.0.clone(),
                                range_checker.clone(),
                            ));
                        let aux = tuple.aux;
                        let io = tuple.io;
                        trace_row[2 * range.0.len()] = io.tuple_less_than;
                        trace_row.extend(aux.flatten());
                    }
                    {
                        let tuple: IsLessThanTupleCols<Val<SC>> = self
                            .is_less_than_tuple_air
                            .clone()
                            .end_idx
                            .generate_trace_row((
                                range.1.clone(),
                                row.idx.to_vec(),
                                range_checker.clone(),
                            ));
                        let aux = tuple.aux;
                        let io = tuple.io;
                        trace_row[2 * range.0.len() + 1] = io.tuple_less_than;
                        trace_row.extend(aux.flatten());
                    }
                    {
                        trace_row.append(&mut final_page_aux_rows.row_mut(i).to_vec());
                    }
                    trace_row
                })
                .collect(),
            self.air_width() - (1 + self.idx_len + self.data_len),
        )
    }
}
