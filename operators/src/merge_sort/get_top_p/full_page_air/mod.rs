use afs_chips::{
    indexed_output_page_air::IndexedOutputPageAir,
    is_less_than_tuple::{columns::IsLessThanTupleAuxCols, IsLessThanTupleAir},
    multitier_page_rw_checker::page_controller::MyLessThanTupleParams,
};
use getset::Getters;

pub mod air;
pub mod bridge;
pub mod columns;
pub mod trace;

#[derive(Clone, Getters)]
pub struct FullIndexedOutputPageAir {
    // bus to send data to other chips
    #[getset(get = "pub")]
    page_bus_index: usize,

    #[getset(get = "pub")]
    page_chip: IndexedOutputPageAir,
    // parameter telling if this is a leaf chip on the init side or the final side.
    is_less_than_tuple_air: FullIndexedOutputPageSubAirs,
    is_less_than_tuple_param: MyLessThanTupleParams,
    idx_len: usize,
    data_len: usize,
}

#[derive(Clone)]
pub struct FullIndexedOutputPageSubAirs {
    pub idx_start: IsLessThanTupleAir,
    pub end_idx: IsLessThanTupleAir,
}

#[allow(clippy::too_many_arguments)]
impl FullIndexedOutputPageAir {
    pub fn new(
        page_bus_index: usize,
        is_less_than_tuple_param: MyLessThanTupleParams,
        lt_bus_index: usize,
        idx_len: usize,
        data_len: usize,
    ) -> Self {
        Self {
            page_bus_index,
            page_chip: IndexedOutputPageAir::new(
                lt_bus_index,
                idx_len,
                data_len,
                is_less_than_tuple_param.limb_bits,
                is_less_than_tuple_param.decomp,
            ),
            idx_len,
            data_len,
            is_less_than_tuple_air: FullIndexedOutputPageSubAirs {
                idx_start: IsLessThanTupleAir::new(
                    lt_bus_index,
                    vec![is_less_than_tuple_param.limb_bits; idx_len],
                    is_less_than_tuple_param.decomp,
                ),
                end_idx: IsLessThanTupleAir::new(
                    lt_bus_index,
                    vec![is_less_than_tuple_param.limb_bits; idx_len],
                    is_less_than_tuple_param.decomp,
                ),
            },
            is_less_than_tuple_param,
        }
    }

    // if self.is_final, we need to include range data to establish sortedness
    // in particular, for each idx, prove the idx lies in the start and end.
    // we then need extra columns that contain results of is_less_than comparisons
    // in particular, we need to constrain that is_alloc * ((1 - (idx < start)) * (1 - (end < idx)) - 1) = 0
    pub fn air_width(&self) -> usize {
        self.page_chip().air_width()
            + (2 * self.idx_len
                + 2
                + 2 * IsLessThanTupleAuxCols::<usize>::get_width(
                    vec![self.is_less_than_tuple_param.limb_bits; self.idx_len],
                    self.is_less_than_tuple_param.decomp,
                    self.idx_len,
                ))
    }
}
