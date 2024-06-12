use afs_derive::AlignedBorrow;

pub const NUM_COLS: usize = 3;

#[repr(C)]
#[derive(AlignedBorrow)]
pub struct RootSignalCols<T> {
    pub root_commitment: Vec<T>,
    pub mult: T,
}

impl<T> RootSignalCols<T> {
    pub fn from_slice(cols: &[T], commitment_len: usize) -> Self
    where
        T: Clone,
    {
        RootSignalCols {
            root_commitment: cols[0..commitment_len].to_vec(),
            mult: cols[commitment_len].clone(),
        }
    }
}
