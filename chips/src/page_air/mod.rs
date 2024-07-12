pub mod air;
pub mod bridge;

#[derive(Clone)]
pub struct PageAir {
    page_bus: usize,
    is_send: bool,
    idx_len: usize,
    data_len: usize,
}

impl PageAir {
    pub fn new(page_bus: usize, is_send: bool, idx_len: usize, data_len: usize) -> Self {
        Self {
            page_bus,
            is_send,
            idx_len,
            data_len,
        }
    }

    pub fn idx_len(&self) -> usize {
        self.idx_len
    }

    pub fn air_width(&self) -> usize {
        1 + self.idx_len + self.data_len
    }
}
