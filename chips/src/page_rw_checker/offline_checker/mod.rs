mod air;
mod chip;
mod columns;
mod trace;

pub struct OfflineChecker {
    bus_index: usize,

    idx_len: usize,
    data_len: usize,
}

impl OfflineChecker {
    pub fn new(bus_index: usize, idx_len: usize, data_len: usize) -> Self {
        Self {
            bus_index,
            idx_len,
            data_len,
        }
    }

    pub fn bus_index(&self) -> usize {
        self.bus_index
    }

    fn page_width(&self) -> usize {
        1 + self.idx_len + self.data_len
    }

    pub fn air_width(&self) -> usize {
        7 + self.page_width() + 2 * (self.idx_len + self.data_len)
    }
}
