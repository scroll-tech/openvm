// TODO: maybe separate into two structs, io and aux
pub struct MiddleChipCols<T> {
    pub is_initial: T,
    pub is_final: T,
    pub clk: T,
    pub page_row: Vec<T>,
    pub op_type: T, // 0 for read, 1 for write

    pub same_key: T,
    pub same_val: T,

    pub is_extra: T, // a bit to indicate if this is an extra row that should be ignored
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
        flattened.push(self.same_key.clone());
        flattened.push(self.same_val.clone());
        flattened.push(self.is_extra.clone());

        println!("page_row's len is {}", self.page_row.len());
        println!("after flattening: {:?}", flattened.len());

        flattened
    }

    // TODO: is there a way to merge from_slice and cols_numbered?
    pub fn from_slice(slc: &[T]) -> Self {
        Self {
            is_initial: slc[0].clone(),
            is_final: slc[1].clone(),
            clk: slc[2].clone(),
            page_row: slc[3..slc.len() - 4].to_vec(),
            op_type: slc[slc.len() - 4].clone(),
            same_key: slc[slc.len() - 3].clone(),
            same_val: slc[slc.len() - 2].clone(),
            is_extra: slc[slc.len() - 1].clone(),
        }
    }

    pub fn cols_numbered(cols: &[usize]) -> MiddleChipCols<usize> {
        MiddleChipCols {
            is_initial: cols[0],
            is_final: cols[1],
            clk: cols[2],
            page_row: cols[3..cols.len() - 4].to_vec(),
            op_type: cols[cols.len() - 4],
            same_key: cols[cols.len() - 3],
            same_val: cols[cols.len() - 2],
            is_extra: cols[cols.len() - 1],
        }
    }
}
