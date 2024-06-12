pub mod air;
pub mod bridge;
pub mod columns;
pub mod trace;

use getset::Getters;

#[derive(Clone, Default, Getters)]
// a single row chip meant to start the flow from the root
pub struct RootSignalChip<const COMMITMENT_LEN: usize> {
    #[getset(get = "pub")]
    bus_index: usize,
}

impl<const COMMITMENT_LEN: usize> RootSignalChip<COMMITMENT_LEN> {
    pub fn new(bus_index: usize) -> Self {
        RootSignalChip { bus_index }
    }
    pub fn air_width(&self) -> usize {
        COMMITMENT_LEN + 1
    }
}
