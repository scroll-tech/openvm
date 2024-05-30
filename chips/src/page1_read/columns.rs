use crate::MAX_COMMITMENT_LEN;

pub struct Page1ReadCols<T> {
    pub is_present: T,
    pub mult: T,
    pub mult_t: T,
    pub commitment: Vec<T>,
    pub key: Vec<T>,
    pub page_row: Vec<T>,
}

/// Columns are
/// [key] | [value] | [commitment] | [mult]
impl<T> Page1ReadCols<T> {
    pub fn cols_numbered(key_len: usize, val_len: usize, cols: &[usize]) -> Page1ReadCols<usize> {
        assert!(cols.len() == 6 + key_len + val_len + MAX_COMMITMENT_LEN);
        Page1ReadCols {
            is_present: cols[0],
            mult: cols[cols.len() - 1],
            mult_t: cols[cols.len() - 4],
            commitment: cols[2 + key_len + val_len..2 + key_len + val_len + MAX_COMMITMENT_LEN]
                .to_vec(),
            key: cols[2..2 + key_len].to_vec(),
            page_row: cols[2 + key_len..2 + key_len + val_len].to_vec(),
        }
    }
}
