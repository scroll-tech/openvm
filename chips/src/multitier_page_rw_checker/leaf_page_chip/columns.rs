use crate::{
    common::page_cols::PageCols, is_less_than_tuple::columns::IsLessThanTupleAuxCols,
    multitier_page_rw_checker::page_controller::MyLessThanTupleParams,
    page_rw_checker::my_final_page::columns::MyFinalPageAuxCols,
};

#[derive(Clone)]
pub struct LeafPageCols<T> {
    pub cache_cols: LeafDataCols<T>,
    pub metadata: LeafPageMetadataCols<T>,
}

#[derive(Clone)]
pub struct LeafDataCols<T> {
    pub is_leaf: T,
    pub page_cols: PageCols<T>,
}

#[derive(Clone)]
pub struct LeafPageSubAirCols<T> {
    pub idx_start: IsLessThanTupleAuxCols<T>,
    pub end_idx: IsLessThanTupleAuxCols<T>,
    pub final_page_aux: MyFinalPageAuxCols<T>,
}

#[derive(Clone)]
pub struct RangeInclusionCols<T> {
    pub start: Vec<T>,
    pub end: Vec<T>,
    pub less_than_start: T,
    pub greater_than_end: T,
}

#[derive(Clone)]
pub struct LeafPageMetadataCols<T> {
    pub own_commitment: Vec<T>,
    pub range_inclusion_cols: Option<RangeInclusionCols<T>>,
    pub subchip_aux_cols: Option<LeafPageSubAirCols<T>>,
}

impl<T> LeafPageCols<T> {
    pub fn from_slice(
        cols: &[T],
        idx_len: usize,
        data_len: usize,
        commitment_len: usize,
        is_init: bool,
        is_less_than_tuple_params: MyLessThanTupleParams,
    ) -> Self
    where
        T: Clone,
    {
        LeafPageCols {
            cache_cols: LeafDataCols::from_slice(
                &cols[0..2 + idx_len + data_len],
                idx_len,
                data_len,
            ),
            metadata: LeafPageMetadataCols::from_slice(
                &cols[2 + idx_len + data_len..],
                idx_len,
                commitment_len,
                is_init,
                is_less_than_tuple_params,
            ),
        }
    }
}

impl<T> LeafPageMetadataCols<T> {
    pub fn from_slice(
        cols: &[T],
        idx_len: usize,
        commitment_len: usize,
        is_init: bool,
        is_less_than_tuple_params: MyLessThanTupleParams,
    ) -> Self
    where
        T: Clone,
    {
        if is_init {
            LeafPageMetadataCols {
                own_commitment: cols[0..commitment_len].to_vec(),
                range_inclusion_cols: None,
                subchip_aux_cols: None,
            }
        } else {
            let mut new_start = commitment_len;
            let range_inclusion_cols = RangeInclusionCols {
                start: cols[new_start..new_start + idx_len].to_vec(),
                end: cols[new_start + idx_len..new_start + 2 * idx_len].to_vec(),
                less_than_start: cols[new_start + 2 * idx_len].clone(),
                greater_than_end: cols[new_start + 2 * idx_len + 1].clone(),
            };
            new_start += 2 * idx_len + 2;
            let mut aux_allocs = vec![];
            let aux_size = IsLessThanTupleAuxCols::<T>::get_width(
                vec![is_less_than_tuple_params.limb_bits; idx_len],
                is_less_than_tuple_params.decomp,
                idx_len,
            );
            for i in 0..2 {
                aux_allocs.push(IsLessThanTupleAuxCols::from_slice(
                    &cols[new_start + i * aux_size..new_start + (i + 1) * aux_size],
                    vec![is_less_than_tuple_params.limb_bits; idx_len],
                    is_less_than_tuple_params.decomp,
                    idx_len,
                ))
            }
            let subair_cols = LeafPageSubAirCols {
                idx_start: aux_allocs[0].clone(),
                end_idx: aux_allocs[1].clone(),
                final_page_aux: MyFinalPageAuxCols::from_slice(
                    &cols[new_start + 2 * aux_size..],
                    is_less_than_tuple_params.limb_bits,
                    is_less_than_tuple_params.decomp,
                    idx_len,
                ),
            };
            LeafPageMetadataCols {
                own_commitment: cols[0..commitment_len].to_vec(),
                range_inclusion_cols: Some(range_inclusion_cols),
                subchip_aux_cols: Some(subair_cols),
            }
        }
    }
}

impl<T> LeafDataCols<T> {
    pub fn from_slice(cols: &[T], idx_len: usize, data_len: usize) -> Self
    where
        T: Clone,
    {
        LeafDataCols {
            is_leaf: cols[0].clone(),
            page_cols: PageCols::from_slice(&cols[1..2 + idx_len + data_len], idx_len, data_len),
        }
    }
}
