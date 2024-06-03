pub struct InitFinalCols<T> {
    pub is_alloc: T,
    pub page_row: Vec<T>,
}

impl<T> InitFinalCols<T> {
    pub fn cols_numbered(cols: &[usize]) -> InitFinalCols<usize> {
        InitFinalCols {
            is_alloc: cols[0],
            page_row: cols[1..].to_vec(),
        }
    }
}
