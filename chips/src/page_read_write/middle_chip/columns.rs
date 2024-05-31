use p3_air::AirBuilder;
use p3_field::{AbstractField, PrimeField};

use crate::page_read_write::checker_controller::OpType;

pub struct MiddleChipCols<T> {
    is_initial: T,
    is_final: T,
    clk: T,
    page_row: Vec<T>,
    op_type: T, // 0 for read, 1 for write
}

impl<T> MiddleChipCols<T>
where
    T: Clone,
{
    pub fn new(is_initial: T, is_final: T, clk: T, page_row: Vec<T>, op_type: T) -> Self {
        Self {
            is_initial,
            is_final,
            clk,
            page_row,
            op_type,
        }
    }
}

impl<T> MiddleChipCols<T>
where
    T: Clone,
{
    pub fn flatten(&self) -> Vec<T> {
        let mut flattened = vec![
            self.is_initial.clone(),
            self.is_final.clone(),
            self.op_type.clone(),
            self.clk.clone(),
        ];
        flattened.extend(self.page_row.clone());

        flattened
    }

    pub fn from_slice(slc: &[T]) -> Self {
        Self {
            is_initial: slc[0].clone(),
            is_final: slc[1].clone(),
            clk: slc[2].clone(),
            page_row: slc[3..slc.len() - 1].to_vec(),
            op_type: slc[slc.len() - 1].clone(),
        }
    }
}
