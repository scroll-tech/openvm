use afs_derive::AlignedBorrow;

#[derive(Default, AlignedBorrow)]
pub struct SortedLimbsCols<T> {
    pub key: Vec<T>,
    pub keys_decomp: Vec<Vec<T>>,
}

impl<T: Clone> SortedLimbsCols<T> {
    pub fn from_slice(slc: &[T], limb_bits: usize, decomp: usize, key_vec_len: usize) -> Self {
        // num_limbs is the number of sublimbs per limb, not including the shifted last sublimb
        let num_limbs = (limb_bits + decomp - 1) / decomp;

        let key = slc[..key_vec_len].to_vec();
        // num_limbs + 1 to account for the shifted last sublimb
        let keys_decomp = slc[key_vec_len..]
            .chunks(num_limbs + 1)
            .map(|chunk| chunk.to_vec())
            .collect();

        Self { key, keys_decomp }
    }

    pub fn get_width(limb_bits: usize, decomp: usize, key_vec_len: usize) -> usize {
        // there are (limb_bits + decomp - 1) / decomp sublimbs per limb, we add 1 to
        // account for the sublimb itself, and another 1 to account for the shifted
        // last sublimb
        key_vec_len * ((limb_bits + decomp - 1) / decomp + 2)
    }

    pub fn cols_numbered(
        cols: &[usize],
        limb_bits: usize,
        decomp: usize,
        key_vec_len: usize,
    ) -> SortedLimbsCols<usize> {
        // num_limbs is the number of sublimbs per limb, not including the shifted last sublimb
        let num_limbs = (limb_bits + decomp - 1) / decomp;
        let key = cols[..key_vec_len].to_vec();
        // num_limbs + 1 to account for the shifted last sublimb
        let keys_decomp = cols[key_vec_len..]
            .chunks(num_limbs + 1)
            .map(|chunk| chunk.to_vec())
            .collect();

        SortedLimbsCols { key, keys_decomp }
    }
}
