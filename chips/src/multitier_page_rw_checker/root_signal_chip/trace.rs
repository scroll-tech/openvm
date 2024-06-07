use p3_field::Field;
use p3_matrix::dense::RowMajorMatrix; // Import the constant from columns.rs

use crate::sub_chip::LocalTraceInstructions;
use std::sync::Arc;

use p3_field::{AbstractField, PrimeField64};
use p3_uni_stark::{StarkGenericConfig, Val};

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
