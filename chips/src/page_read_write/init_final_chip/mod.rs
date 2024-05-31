pub mod chip;
pub mod columns;

pub struct InitFinalChip {
    bus_index: usize,
    page_width: usize,

    is_send: bool,
}

impl InitFinalChip {
    pub fn new(bus_index: usize, page_width: usize, is_send: bool) -> Self {
        Self {
            bus_index,
            page_width,
            is_send,
        }
    }

    pub fn bus_index(&self) -> usize {
        self.bus_index
    }

    pub fn air_width(&self) -> usize {
        self.page_width
    }
}
