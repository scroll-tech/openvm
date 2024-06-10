use std::sync::Arc;

use afs_stark_backend::interaction::trace;
use itertools::Itertools;
use p3_field::{AbstractField, PrimeField64};
use p3_matrix::dense::RowMajorMatrix;
use p3_uni_stark::{StarkGenericConfig, Val};

use crate::{
    is_less_than_tuple::columns::IsLessThanTupleAuxCols, range_gate::RangeCheckerGateChip,
    sub_chip::LocalTraceInstructions,
};

use super::InternalPageChip;

impl<const COMMITMENT_LEN: usize> InternalPageChip<COMMITMENT_LEN> {
    // The trace is the whole page (including the is_alloc column)
    pub fn generate_cached_trace<F: PrimeField64>(&self, page: Vec<Vec<u32>>) -> RowMajorMatrix<F> {
        RowMajorMatrix::new(
            page.into_iter()
                .flat_map(|row| row.into_iter().map(F::from_wrapped_u32).collect::<Vec<F>>())
                .collect(),
            1 + 2 * self.idx_len + COMMITMENT_LEN,
        )
    }

    pub fn generate_main_trace<F: PrimeField64>(
        &self,
        page: Vec<Vec<u32>>,
        commit: Vec<u32>,
        mults: Vec<u32>,
        range: (Vec<u32>, Vec<u32>),
        range_checker: Arc<RangeCheckerGateChip>,
    ) -> RowMajorMatrix<F> {
        assert!(commit.len() == COMMITMENT_LEN);
        RowMajorMatrix::new(
            page.iter()
                .zip(mults)
                .enumerate()
                .flat_map(|(i, (row, mult))| {
                    let mut trace_row = vec![];
                    trace_row.extend(commit.clone());
                    trace_row.push(mult);
                    trace_row.push(mult * row[0]);
                    // dummy value
                    trace_row.push((mult * row[0] == 1) as u32);
                    trace_row.push(0);
                    let next = if i < page.len() - 1 {
                        page[i + 1][1..1 + self.idx_len].to_vec()
                    } else {
                        vec![0; self.idx_len]
                    };
                    if !self.is_init {
                        trace_row.extend(next.clone());
                        trace_row.push(1);
                        trace_row.push(0);
                        trace_row.extend(range.0.clone());
                        trace_row.extend(range.1.clone());
                        trace_row.extend(vec![0; 4]);
                        let mut trace_row: Vec<F> = trace_row
                            .into_iter()
                            .map(|i| F::from_wrapped_u32(i))
                            .collect();
                        let mut gen_aux = |idx1: Vec<u32>, idx2: Vec<u32>, lt_res_idx: usize| {
                            let lt_cols = self
                                .is_less_than_tuple_air
                                .clone()
                                .unwrap()
                                .key1_start
                                .generate_trace_row((idx1, idx2, range_checker.clone()));
                            trace_row.extend(lt_cols.aux.flatten());
                            trace_row[COMMITMENT_LEN + 4 + self.idx_len + lt_res_idx] =
                                lt_cols.io.tuple_less_than;
                        };
                        gen_aux(
                            row[1..1 + self.idx_len].to_vec(),
                            range.0.clone(),
                            2 + 2 * self.idx_len,
                        );
                        gen_aux(
                            range.1.clone(),
                            row[1..1 + self.idx_len].to_vec(),
                            2 + 2 * self.idx_len + 1,
                        );
                        gen_aux(
                            row[1 + self.idx_len..1 + 2 * self.idx_len].to_vec(),
                            range.0.clone(),
                            2 + 2 * self.idx_len + 2,
                        );
                        gen_aux(
                            range.1.clone(),
                            row[1 + self.idx_len..1 + 2 * self.idx_len].to_vec(),
                            2 + 2 * self.idx_len + 3,
                        );
                        gen_aux(
                            row[1 + self.idx_len..1 + 2 * self.idx_len].to_vec(),
                            next.clone(),
                            0,
                        );
                        gen_aux(
                            row[1 + self.idx_len..1 + 2 * self.idx_len].to_vec(),
                            row[1..1 + self.idx_len].to_vec(),
                            1,
                        );
                        trace_row.push(
                            self.is_less_than_tuple_air
                                .clone()
                                .unwrap()
                                .mult_is_1
                                .generate_trace_row(F::from_wrapped_u32(mult * row[0]) - F::one())
                                .inv,
                        );
                        // println!("{:?}", *trace_row.last().unwrap());
                        trace_row[COMMITMENT_LEN + 3] = trace_row[COMMITMENT_LEN + 1] - F::one();
                        // println!("{:?}", trace_row[COMMITMENT_LEN + 1] - F::one());
                        // println!("TRACE_LEN {:?}", trace_row.len());
                        trace_row
                    } else {
                        let mut trace_row = trace_row
                            .into_iter()
                            .map(|i| F::from_wrapped_u32(i))
                            .collect_vec();
                        trace_row[COMMITMENT_LEN + 3] = trace_row[COMMITMENT_LEN + 1] - F::one();
                        trace_row
                    }
                })
                .collect(),
            self.air_width() - (1 + 2 * self.idx_len + COMMITMENT_LEN),
        )
    }
}

// #[derive(Clone)]
// pub struct PtrPageCols<T> {
//     pub is_alloc: T,
//     pub start: Vec<T>,
//     pub end: Vec<T>,
//     pub commitment: Vec<T>,
// }

// #[derive(Clone)]
// pub struct InternalPageSubAirCols<T> {
//     pub key1_start: IsLessThanTupleAuxCols<T>,
//     pub end_key1: IsLessThanTupleAuxCols<T>,
//     pub key2_start: IsLessThanTupleAuxCols<T>,
//     pub end_key2: IsLessThanTupleAuxCols<T>,
//     pub end_start: IsLessThanTupleAuxCols<T>,
//     pub end_next: IsLessThanTupleAuxCols<T>,
// }

// // pub struct LeafPageCacheCols<T> {
// //     pub is_alloc: T, // indicates if row is allocated
// //     pub idx: Vec<T>,
// //     pub data: Vec<T>,
// // }
// #[derive(Clone)]
// pub struct TwoRangeInclusionCols<T> {
//     pub start: Vec<T>,
//     pub end: Vec<T>,
//     pub less_than_start: (T, T),
//     pub greater_than_end: (T, T),
// }

// #[derive(Clone)]
// pub struct ProveSortCols<T> {
//     pub next_key: Vec<T>,
//     // we want this to be true
//     pub end_less_than_next: T,
//     // we want this to be false
//     pub end_less_than_start: T,
// }

// #[derive(Clone)]
// pub struct InternalPageMetadataCols<T> {
//     pub own_commitment: Vec<T>,
//     pub mult: T,
//     pub mult_alloc: T,
//     pub mult_alloc_minus_one: T,
//     pub mult_alloc_is_1: T,
//     pub prove_sort_cols: Option<ProveSortCols<T>>,
//     pub range_inclusion_cols: Option<TwoRangeInclusionCols<T>>,
//     pub subchip_aux_cols: Option<InternalPageSubAirCols<T>>,
// }
