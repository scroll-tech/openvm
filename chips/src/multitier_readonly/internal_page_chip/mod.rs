use getset::Getters;

pub mod air;
pub mod bridge;
pub mod columns;
pub mod trace;

#[derive(Clone, Getters)]
pub struct InternalPageChip<const COMMITMENT_LEN: usize> {
    #[getset(get = "pub")]
    path_bus_index: usize,
    idx_len: usize,
    // forget about range checking stuff for now
    // pub range_chip: Mutex<RangeCheckerChip<MAX>>,
}

impl<const COMMITMENT_LEN: usize> InternalPageChip<COMMITMENT_LEN> {
    /// assumes keys have already been split into limb_bits
    pub fn new(path_bus_index: usize, idx_len: usize) -> Self {
        Self {
            path_bus_index,
            idx_len,
        }
    }

    pub fn air_width(&self) -> usize {
        2 + 2 * self.idx_len + 2 * COMMITMENT_LEN + 2
    }
}
