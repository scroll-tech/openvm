pub struct PageCols<T> {
    pub is_alloc: T,      // indicates if row is allocated
    pub page_row: Vec<T>, // key followed by value in the row
}

impl<T> PageCols<T> {
    pub fn cols_numbered(cols: &[usize]) -> PageCols<usize> {
        PageCols {
            is_alloc: cols[0],
            page_row: cols[1..].to_vec(),
        }
    }
}
