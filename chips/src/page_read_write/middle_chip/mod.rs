use super::page_controller::Operation;

mod air;
mod chip;
mod columns;
mod trace;

pub struct MiddleChip {
    bus_index: usize,

    key_len: usize,
    val_len: usize,
}

impl MiddleChip {
    pub fn new(bus_index: usize, key_len: usize, val_len: usize) -> Self {
        Self {
            bus_index,
            key_len,
            val_len,
        }
    }

    pub fn bus_index(&self) -> usize {
        self.bus_index
    }

    fn page_width(&self) -> usize {
        // TODO: do we need to handle this case?
        if self.key_len == 0 && self.val_len == 0 {
            // Special case when MiddleChip is empty
            return 0;
        }

        1 + self.key_len + self.val_len
    }

    pub fn air_width(&self) -> usize {
        7 + self.page_width() + 2 * (self.key_len + self.val_len)
    }
}
