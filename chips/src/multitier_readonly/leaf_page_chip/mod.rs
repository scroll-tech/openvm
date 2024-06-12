use getset::Getters;

pub mod air;
pub mod bridge;
pub mod columns;
pub mod trace;

#[derive(Clone, Getters)]
pub struct LeafPageChip<const COMMITMENT_LEN: usize> {
    #[getset(get = "pub")]
    path_bus_index: usize,
    #[getset(get = "pub")]
    data_bus_index: usize,
    idx_len: usize,
    data_len: usize,
    // forget about range checking stuff for now
    // pub range_chip: Mutex<RangeCheckerChip<MAX>>,
}

impl<const COMMITMENT_LEN: usize> LeafPageChip<COMMITMENT_LEN> {
    /// assumes keys have already been split into limb_bits
    pub fn new(
        data_bus_index: usize,
        path_bus_index: usize,
        idx_len: usize,
        data_len: usize,
    ) -> Self {
        assert!(data_bus_index != path_bus_index);
        Self {
            data_bus_index,
            path_bus_index,
            idx_len,
            data_len,
        }
    }

    pub fn air_width(&self) -> usize {
        2 + self.idx_len + self.data_len + COMMITMENT_LEN + 2
    }
}
