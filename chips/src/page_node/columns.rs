use crate::MAX_COMMITMENT_LEN;

pub struct PageNodeNull {}

pub struct PageNodeBranch<T> {
    pub mult: T,
    pub mult_p: T,
    pub mult_b: T,
    pub mult_t: T,
    pub is_present: T,
    pub is_terminal: T,
    pub key_start: Vec<T>,
    pub key_end: Vec<T>,
    pub child_commitment: Vec<T>,
    pub commitment: Vec<T>,
}

pub struct PageNodeTerminal<T> {
    pub mult: T,
    pub mult_p: T,
    pub mult_b: T,
    pub mult_t: T,
    pub is_present: T,
    pub is_terminal: T,
    pub key: Vec<T>,
    pub val: Vec<T>,
    pub commitment: Vec<T>,
}

pub enum PageNodeReadCols<T> {
    Terminal(PageNodeTerminal<T>),
    Branch(PageNodeBranch<T>),
    Null(PageNodeNull),
}

/// Columns are
/// [start] | [end] | [child commitment] | [commitment] | [mult]
impl<T> PageNodeReadCols<T> {
    pub fn cols_numbered(
        key_len: usize,
        val_len: usize,
        cols: &[usize],
    ) -> PageNodeReadCols<usize> {
        // assert!(cols.len() >= 3 && cols.len() % 2 == 1);
        let read_cols = if cols[0] == 0 {
            PageNodeReadCols::Null(PageNodeNull {})
        } else if cols[1] == 0 {
            PageNodeReadCols::Terminal(Self::as_terminal(key_len, val_len, cols))
        } else {
            PageNodeReadCols::Branch(Self::as_branch(key_len, cols))
        };
        read_cols
    }

    pub fn as_branch(key_len: usize, cols: &[usize]) -> PageNodeBranch<usize> {
        let commit_start = cols.len() - 4 - MAX_COMMITMENT_LEN;
        PageNodeBranch {
            mult: cols[cols.len() - 1],
            mult_p: cols[cols.len() - 2],
            mult_b: cols[cols.len() - 3],
            mult_t: cols[cols.len() - 4],
            is_present: cols[0],
            is_terminal: cols[1],
            key_start: cols[2..key_len + 2].to_vec(),
            key_end: cols[2 + key_len..2 + 2 * key_len].to_vec(),
            child_commitment: cols[2 + 2 * key_len..2 + 2 * key_len + MAX_COMMITMENT_LEN].to_vec(),
            commitment: cols[commit_start..commit_start + MAX_COMMITMENT_LEN].to_vec(),
        }
    }
    pub fn as_terminal(key_len: usize, val_len: usize, cols: &[usize]) -> PageNodeTerminal<usize> {
        let commit_start = cols.len() - 4 - MAX_COMMITMENT_LEN;
        PageNodeTerminal {
            mult: cols[cols.len() - 1],
            mult_p: cols[cols.len() - 2],
            mult_b: cols[cols.len() - 3],
            mult_t: cols[cols.len() - 4],
            is_present: cols[0],
            is_terminal: cols[1],
            commitment: cols[commit_start..commit_start + MAX_COMMITMENT_LEN].to_vec(),
            key: cols[2..2 + key_len].to_vec(),
            val: cols[2 + key_len..2 + key_len + val_len].to_vec(),
        }
    }
}
