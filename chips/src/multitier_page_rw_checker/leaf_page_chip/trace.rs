use std::sync::Arc;

use p3_field::PrimeField64;
use p3_matrix::dense::RowMajorMatrix;

use crate::{
    is_less_than_tuple::columns::IsLessThanTupleCols, range_gate::RangeCheckerGateChip,
    sub_chip::LocalTraceInstructions,
};

use super::LeafPageChip;

impl<const COMMITMENT_LEN: usize> LeafPageChip<COMMITMENT_LEN> {
    // The trace is the whole page (including the is_alloc column)
    pub fn generate_cached_trace<F: PrimeField64>(&self, page: Vec<Vec<u32>>) -> RowMajorMatrix<F> {
        RowMajorMatrix::new(
            page.into_iter()
                .flat_map(|row| row.into_iter().map(F::from_wrapped_u32).collect::<Vec<F>>())
                .collect(),
            2 + self.idx_len + self.data_len,
        )
    }

    pub fn generate_main_trace<F: PrimeField64>(
        &self,
        page: Vec<Vec<u32>>,
        commit: Vec<u32>,
        range: (Vec<u32>, Vec<u32>),
        range_checker: Arc<RangeCheckerGateChip>,
    ) -> RowMajorMatrix<F> {
        assert!(commit.len() == COMMITMENT_LEN);
        RowMajorMatrix::new(
            page.iter()
                .flat_map(|row| {
                    let mut trace_row = vec![];
                    trace_row.extend(commit.clone());
                    if !self.is_init {
                        trace_row.extend(range.0.clone());
                        trace_row.extend(range.1.clone());
                        trace_row.extend(vec![0; 2]);
                        let mut trace_row: Vec<F> = trace_row
                            .into_iter()
                            .map(|i| F::from_wrapped_u32(i))
                            .collect();
                        {
                            let tuple: IsLessThanTupleCols<F> = self
                                .is_less_than_tuple_air
                                .clone()
                                .unwrap()
                                .key_start
                                .generate_trace_row((
                                    row[2..2 + self.idx_len].to_vec(),
                                    range.0.clone(),
                                    range_checker.clone(),
                                ));
                            let aux = tuple.aux;
                            let io = tuple.io;
                            trace_row[COMMITMENT_LEN + 2 * range.0.len()] = io.tuple_less_than;
                            trace_row.extend(aux.flatten());
                        }
                        {
                            let tuple: IsLessThanTupleCols<F> = self
                                .is_less_than_tuple_air
                                .clone()
                                .unwrap()
                                .end_key
                                .generate_trace_row((
                                    range.1.clone(),
                                    row[2..2 + self.idx_len].to_vec(),
                                    range_checker.clone(),
                                ));
                            let aux = tuple.aux;
                            let io = tuple.io;
                            trace_row[COMMITMENT_LEN + 2 * range.0.len() + 1] = io.tuple_less_than;
                            trace_row.extend(aux.flatten());
                        }
                        trace_row
                    } else {
                        trace_row
                            .into_iter()
                            .map(|i| F::from_wrapped_u32(i))
                            .collect::<Vec<F>>()
                    }
                })
                .collect(),
            self.air_width() - (2 + self.idx_len + self.data_len),
        )
    }
}

// #[derive(Clone)]
// pub struct LeafPageCols<T> {
//     pub cache_cols: PageCols<T>,
//     pub metadata: LeafPageMetadataCols<T>,
// }

// #[derive(Clone)]
// pub struct LeafPageSubAirCols<T> {
//     pub key_start: IsLessThanTupleAuxCols<T>,
//     pub end_key: IsLessThanTupleAuxCols<T>,
// }

// #[derive(Clone)]
// pub struct RangeInclusionCols<T> {
//     pub start: Vec<T>,
//     pub end: Vec<T>,
//     pub less_than_start: T,
//     pub greater_than_end: T,
// }

// #[derive(Clone)]
// pub struct LeafPageMetadataCols<T> {
//     pub own_commitment: Vec<T>,
//     pub range_inclusion_cols: Option<RangeInclusionCols<T>>,
//     pub subchip_aux_cols: Option<LeafPageSubAirCols<T>>,
// }
