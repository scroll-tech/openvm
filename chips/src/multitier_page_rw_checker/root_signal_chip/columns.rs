use afs_derive::AlignedBorrow;

pub const NUM_COLS: usize = 3;

#[repr(C)]
#[derive(AlignedBorrow)]
pub struct RootSignalCols<T> {
    pub root_commitment: Vec<T>,
    pub mult: T,
    pub range: Option<(Vec<T>, Vec<T>)>,
}

impl<T> RootSignalCols<T> {
    pub fn from_slice(cols: &[T], idx_len: usize, commitment_len: usize, is_init: bool) -> Self
    where
        T: Clone,
    {
        if is_init {
            RootSignalCols {
                root_commitment: cols[0..commitment_len].to_vec(),
                mult: cols[commitment_len].clone(),
                range: None,
            }
        } else {
            RootSignalCols {
                root_commitment: cols[0..commitment_len].to_vec(),
                mult: cols[commitment_len].clone(),
                range: Some((
                    cols[commitment_len + 1..commitment_len + 1 + idx_len].to_vec(),
                    cols[commitment_len + 1 + idx_len..commitment_len + 1 + 2 * idx_len].to_vec(),
                )),
            }
        }
    }
}
