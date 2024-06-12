use getset::Getters;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

#[derive(Clone, Getters)]
pub struct PageRequester {
    #[getset(get = "pub")]
    data_bus_index: usize,
    idx_len: usize,
    data_len: usize,
}

impl PageRequester {
    pub fn new(data_bus_index: usize, idx_len: usize, data_len: usize) -> Self {
        Self {
            data_bus_index,
            idx_len,
            data_len,
        }
    }
}
