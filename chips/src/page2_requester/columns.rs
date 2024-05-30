use afs_derive::AlignedBorrow;

use crate::MAX_COMMITMENT_LEN;

#[derive(Default, AlignedBorrow)]
pub struct PageRequesterCols<T> {
    pub k: Vec<T>,
    pub v: Vec<T>,
    pub c: Vec<T>,
}

// pub const NUM_XOR_REQUESTER_COLS: usize = size_of::<PageRequesterCols<u8>>();
// pub const XOR_REQUESTER_COL_MAP: PageRequesterCols<usize> = make_col_map();

pub fn make_col_map(key_len: usize, val_len: usize) -> PageRequesterCols<usize> {
    let indices_arr: Vec<usize> = (0..key_len + val_len + MAX_COMMITMENT_LEN)
        .into_iter()
        .collect();
    PageRequesterCols {
        k: indices_arr[0..key_len].to_vec(),
        v: indices_arr[key_len..key_len + val_len].to_vec(),
        c: indices_arr[key_len + val_len..key_len + val_len + MAX_COMMITMENT_LEN].to_vec(),
    }
}
