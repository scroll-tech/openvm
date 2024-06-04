use crate::is_equal_vec::columns::IsEqualVecAuxCols;

// TODO: maybe separate into two structs, io and aux
// TODO: add comments on what those bits do
#[derive(Debug)]
pub struct MiddleChipCols<T> {
    pub is_initial: T,
    pub is_final: T,
    pub clk: T,
    pub page_row: Vec<T>,
    pub op_type: T, // 0 for read, 1 for write

    pub same_key: T,
    pub same_val: T,

    pub is_extra: T, // a bit to indicate if this is an extra row that should be ignored

    pub is_equal_key_aux: IsEqualVecAuxCols<T>,
    pub is_equal_val_aux: IsEqualVecAuxCols<T>,
}

impl<T> MiddleChipCols<T>
where
    T: Clone,
{
    pub fn new(
        is_initial: T,
        is_final: T,
        clk: T,
        page_row: Vec<T>,
        op_type: T,
        same_key: T,
        same_val: T,
        is_extra: T,
        is_equal_key_aux: IsEqualVecAuxCols<T>,
        is_equal_val_aux: IsEqualVecAuxCols<T>,
    ) -> Self {
        Self {
            is_initial,
            is_final,
            clk,
            page_row,
            op_type,
            same_key,
            same_val,
            is_extra,
            is_equal_key_aux,
            is_equal_val_aux,
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
            self.clk.clone(),
        ];
        flattened.extend(self.page_row.clone());
        flattened.extend(vec![
            self.op_type.clone(),
            self.same_key.clone(),
            self.same_val.clone(),
            self.is_extra.clone(),
        ]);

        flattened.extend(self.is_equal_key_aux.flatten());
        flattened.extend(self.is_equal_val_aux.flatten());

        flattened
    }

    // TODO: is there a way to merge from_slice and cols_numbered?
    pub fn from_slice(slc: &[T], page_width: usize, key_len: usize, val_len: usize) -> Self {
        Self {
            is_initial: slc[0].clone(),
            is_final: slc[1].clone(),
            clk: slc[2].clone(),
            page_row: slc[3..3 + page_width].to_vec(),
            op_type: slc[4 + page_width].clone(),
            same_key: slc[5 + page_width].clone(),
            same_val: slc[6 + page_width].clone(),
            is_extra: slc[7 + page_width].clone(),
            is_equal_key_aux: IsEqualVecAuxCols::from_slice(
                &slc[8 + page_width..8 + page_width + 2 * key_len],
                key_len,
            ),
            is_equal_val_aux: IsEqualVecAuxCols::from_slice(
                &slc[8 + page_width + 2 * key_len..8 + page_width + 2 * key_len + 2 * val_len],
                val_len,
            ),
        }
    }

    pub fn cols_numbered(
        cols: &[usize],
        page_width: usize,
        key_len: usize,
        val_len: usize,
    ) -> MiddleChipCols<usize> {
        MiddleChipCols {
            is_initial: cols[0],
            is_final: cols[1],
            clk: cols[2],
            page_row: cols[3..3 + page_width].to_vec(),
            op_type: cols[4 + page_width],
            same_key: cols[5 + page_width],
            same_val: cols[6 + page_width],
            is_extra: cols[7 + page_width],
            is_equal_key_aux: IsEqualVecAuxCols::new(
                cols[8 + page_width..8 + page_width + key_len].to_vec(),
                cols[8 + page_width + key_len..8 + page_width + 2 * key_len].to_vec(),
            ),
            is_equal_val_aux: IsEqualVecAuxCols::new(
                cols[8 + page_width + 2 * key_len..8 + page_width + 2 * key_len + val_len].to_vec(),
                cols[8 + page_width + 2 * key_len + val_len
                    ..8 + page_width + 2 * key_len + 2 * val_len]
                    .to_vec(),
            ),
        }
    }
}
