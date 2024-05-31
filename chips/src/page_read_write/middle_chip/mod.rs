use super::checker_controller::Operation;

mod air;
mod columns;
mod trace;

pub struct MiddleChip {
    bus_index: usize,

    key_len: usize,
    val_len: usize,

    ops: Vec<Operation>,
}

impl MiddleChip {
    pub fn new(bus_index: usize, key_len: usize, val_len: usize) -> Self {
        Self {
            bus_index,
            key_len,
            val_len,
            ops: vec![],
        }
    }

    pub fn bus_index(&self) -> usize {
        self.bus_index
    }

    pub fn air_width(&self) -> usize {
        4 + self.key_len + self.val_len
    }
}
