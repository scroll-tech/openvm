use afs_derive::AlignedBorrow;
#[derive(Default, AlignedBorrow)]
pub struct PageRequesterCols<T> {
    pub idx: Vec<T>,
    pub data: Vec<T>,
}

// pub const NUM_XOR_REQUESTER_COLS: usize = size_of::<PageRequesterCols<u8>>();
// pub const XOR_REQUESTER_COL_MAP: PageRequesterCols<usize> = make_col_map();

pub fn make_col_map(idx_len: usize, data_len: usize) -> PageRequesterCols<usize> {
    let indices_arr: Vec<usize> = (0..idx_len + data_len).into_iter().collect();
    PageRequesterCols {
        idx: indices_arr[0..idx_len].to_vec(),
        data: indices_arr[idx_len..idx_len + data_len].to_vec(),
    }
}
