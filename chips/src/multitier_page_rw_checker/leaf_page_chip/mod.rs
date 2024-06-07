use getset::Getters;

use crate::{
    is_less_than_tuple::{columns::IsLessThanTupleCols, IsLessThanTupleAir},
    page_rw_checker::page_chip::PageChip,
};

use super::page_controller::LessThanTupleParams;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

#[derive(Clone, Getters)]
pub struct LeafPageChip<const COMMITMENT_LEN: usize> {
    // bus to establish connectivity/internode consistency
    #[getset(get = "pub")]
    path_bus_index: usize,
    // bus to send data to other chips
    #[getset(get = "pub")]
    data_bus_index: usize,

    #[getset(get = "pub")]
    page_chip: PageChip,
    // parameter telling if this is a leaf chip on the init side or the final side.
    is_less_than_tuple_air: Option<LeafPageSubAirs>,
    is_less_than_tuple_param: LessThanTupleParams,
    is_init: bool,
    idx_len: usize,
    data_len: usize,
}

#[derive(Clone)]
pub struct LeafPageSubAirs {
    pub key_start: IsLessThanTupleAir,
    pub end_key: IsLessThanTupleAir,
}

#[derive(Clone)]
pub struct LeafPageSubAirBuses {
    pub key_start: usize,
    pub end_key: usize,
}

impl<const COMMITMENT_LEN: usize> LeafPageChip<COMMITMENT_LEN> {
    pub fn new(
        path_bus_index: usize,
        data_bus_index: usize,
        is_less_than_tuple_param: LessThanTupleParams,
        lt_bus_index: usize,
        idx_len: usize,
        data_len: usize,
        is_init: bool,
    ) -> Self {
        if is_init {
            Self {
                path_bus_index,
                data_bus_index,
                page_chip: PageChip::new(data_bus_index, idx_len, data_len, is_init),
                idx_len,
                data_len,
                is_init,
                is_less_than_tuple_air: None,
                is_less_than_tuple_param,
            }
        } else {
            Self {
                path_bus_index,
                data_bus_index,
                page_chip: PageChip::new(data_bus_index, idx_len, data_len, is_init),
                idx_len,
                data_len,
                is_init,
                is_less_than_tuple_air: Some(LeafPageSubAirs {
                    key_start: IsLessThanTupleAir::new(
                        lt_bus_index,
                        is_less_than_tuple_param.range_max, // unsure about this
                        is_less_than_tuple_param.limb_bits.clone(),
                        is_less_than_tuple_param.decomp,
                    ),
                    end_key: IsLessThanTupleAir::new(
                        lt_bus_index,
                        is_less_than_tuple_param.range_max, // unsure about this
                        is_less_than_tuple_param.limb_bits.clone(),
                        is_less_than_tuple_param.decomp,
                    ),
                }),
                is_less_than_tuple_param,
            }
        }
    }

    // if self.is_final, we need to include range data to establish sortedness
    // in particular, for each idx, prove the idx lies in the start and end.
    // we then need extra columns that contain results of is_less_than comparisons
    // in particular, we need to constrain that is_alloc * ((1 - (idx < start)) * (1 - (end < idx)) - 1) = 0
    pub fn air_width(&self) -> usize {
        self.page_chip().air_width()
            + COMMITMENT_LEN
            + (1 - self.is_init as usize)
                * 2
                * (self.idx_len
                    + 1
                    + 2 * IsLessThanTupleCols::<usize>::get_width(
                        self.is_less_than_tuple_param.limb_bits.clone(),
                        self.is_less_than_tuple_param.decomp,
                        self.idx_len,
                    )
                    - 4 * self.idx_len
                    - 2)
    }
}
