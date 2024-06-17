use std::iter;

use crate::final_page::{
    columns::{FinalPageAuxCols, FinalPageCols},
    FinalPageAir,
};

pub struct MyFinalPageCols<T> {
    /// The columns for FinalPageAir, which include the page itself
    /// and the extra columns for ensuting sorting
    pub final_page_cols: FinalPageCols<T>,
    /// The multiplicity with which a row is received on the page_bus
    pub rcv_mult: T,
}

#[derive(Clone, Debug)]
pub struct MyFinalPageAuxCols<T> {
    /// The columns for FinalPageAir, which include the page itself
    /// and the extra columns for ensuting sorting
    pub final_page_aux_cols: FinalPageAuxCols<T>,
    /// The multiplicity with which a row is received on the page_bus
    pub rcv_mult: T,
}

impl<T: Clone> MyFinalPageCols<T> {
    pub fn from_slice(slc: &[T], final_air: &FinalPageAir) -> Self {
        Self {
            final_page_cols: FinalPageCols::from_slice(
                &slc[..slc.len() - 1],
                final_air.idx_len,
                final_air.data_len,
                final_air.idx_limb_bits,
                final_air.idx_decomp,
            ),
            rcv_mult: slc[slc.len() - 1].clone(),
        }
    }
}

impl<T: Clone> MyFinalPageAuxCols<T> {
    pub fn from_slice(
        slc: &[T],
        limb_bits: usize,
        decomp: usize,
        tuple_len: usize,
    ) -> MyFinalPageAuxCols<T> {
        MyFinalPageAuxCols {
            final_page_aux_cols: FinalPageAuxCols::from_slice(
                &slc[0..slc.len() - 1],
                limb_bits,
                decomp,
                tuple_len,
            ),
            rcv_mult: slc[slc.len() - 1].clone(),
        }
    }

    pub fn flatten(&self) -> Vec<T> {
        self.final_page_aux_cols
            .flatten()
            .into_iter()
            .chain(iter::once(self.rcv_mult.clone()))
            .collect()
    }
}
