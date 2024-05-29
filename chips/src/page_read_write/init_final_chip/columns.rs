pub struct InitFinalCols<T> {
    pub page_row: Vec<T>,
}

impl<T> InitFinalCols<T> {
    pub fn cols_numbered(cols: &[usize]) -> InitFinalCols<usize> {
        InitFinalCols {
            page_row: cols.to_vec(),
        }
    }
}
