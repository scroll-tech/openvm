use crate::is_equal_vec::columns::IsEqualVecAuxCols;

#[derive(Debug)]
pub struct OfflineCheckerCols<T> {
    pub is_initial: T, // this bit indicates if this row comes from the initial page
    pub is_final: T, // this bit indicates if this row should go to the final page (last row for the key)

    pub clk: T,           // timestamp for the operation
    pub page_row: Vec<T>, // the row of the page: starts with is_alloc bit, then key, then val
    pub op_type: T,       // 0 for read, 1 for write

    pub same_key: T, // this bit indicates if the key matches the one in the previous row (should be 0 in first row)
    pub same_val: T, // this bit indicates if the val matches the one in the previous row (should be 0 in first row)

    pub is_extra: T, // a bit to indicate if this is an extra row that should be ignored

    pub is_equal_key_aux: IsEqualVecAuxCols<T>, // auxiliary columns used for same_key
    pub is_equal_val_aux: IsEqualVecAuxCols<T>, // auxiliary columns used for same_val
}

impl<T> OfflineCheckerCols<T>
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

impl<T> OfflineCheckerCols<T>
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

    pub fn from_slice(slc: &[T], page_width: usize, key_len: usize, val_len: usize) -> Self {
        Self {
            is_initial: slc[0].clone(),
            is_final: slc[1].clone(),
            clk: slc[2].clone(),
            page_row: slc[3..3 + page_width].to_vec(),
            op_type: slc[3 + page_width].clone(),
            same_key: slc[4 + page_width].clone(),
            same_val: slc[5 + page_width].clone(),
            is_extra: slc[6 + page_width].clone(),
            is_equal_key_aux: IsEqualVecAuxCols::from_slice(
                &slc[7 + page_width..7 + page_width + 2 * key_len],
                key_len,
            ),
            is_equal_val_aux: IsEqualVecAuxCols::from_slice(
                &slc[7 + page_width + 2 * key_len..7 + page_width + 2 * key_len + 2 * val_len],
                val_len,
            ),
        }
    }
}
