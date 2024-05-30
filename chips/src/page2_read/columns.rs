use crate::MAX_COMMITMENT_LEN;

pub struct Page2ReadCols<T> {
    pub is_present: T,
    pub mult: T,
    pub mult_b: T,
    pub key_start: Vec<T>,
    pub key_end: Vec<T>,
    pub child_commitment: Vec<T>,
    pub commitment: Vec<T>,
}

/// Columns are
/// [start] | [end] | [child commitment] | [commitment] | [mult]
impl<T> Page2ReadCols<T> {
    pub fn cols_numbered(key_len: usize, cols: &[usize]) -> Page2ReadCols<usize> {
        assert!(cols.len() == 6 + 2 * MAX_COMMITMENT_LEN + 2 * key_len);
        Page2ReadCols {
            is_present: cols[0],
            mult: cols[cols.len() - 1],
            mult_b: cols[cols.len() - 3],
            commitment: cols
                [2 + MAX_COMMITMENT_LEN + 2 * key_len..2 + 2 * MAX_COMMITMENT_LEN + 2 * key_len]
                .to_vec(),
            key_start: cols[2..2 + key_len].to_vec(),
            key_end: cols[2 + key_len..2 + 2 * key_len].to_vec(),
            child_commitment: cols[2 + 2 * key_len..2 + 2 * key_len + MAX_COMMITMENT_LEN].to_vec(),
        }
    }
}
