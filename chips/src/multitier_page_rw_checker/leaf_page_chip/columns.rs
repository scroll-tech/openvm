use crate::{
    is_less_than_tuple::columns::{IsLessThanTupleAuxCols, IsLessThanTupleCols},
    multitier_page_rw_checker::page_controller::LessThanTupleParams,
    page_rw_checker::page_chip::columns::PageCols,
};

#[derive(Clone)]
pub struct LeafPageCols<T> {
    pub cache_cols: PageCols<T>,
    pub metadata: LeafPageMetadataCols<T>,
}

#[derive(Clone)]
pub struct LeafPageSubAirCols<T> {
    pub key_start: IsLessThanTupleAuxCols<T>,
    pub end_key: IsLessThanTupleAuxCols<T>,
}

// pub struct LeafPageCacheCols<T> {
//     pub is_alloc: T, // indicates if row is allocated
//     pub idx: Vec<T>,
//     pub data: Vec<T>,
// }
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
        is_less_than_tuple_params: LessThanTupleParams,
    ) -> Self
    where
        T: Clone,
    {
        LeafPageCols {
            cache_cols: PageCols::from_slice(&cols[0..1 + idx_len + data_len], idx_len, data_len),
            metadata: LeafPageMetadataCols::from_slice(
                &cols[1 + idx_len + data_len..],
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
        is_less_than_tuple_params: LessThanTupleParams,
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
                is_less_than_tuple_params.limb_bits.clone(),
                is_less_than_tuple_params.decomp,
                idx_len,
            );
            for i in 0..2 {
                aux_allocs.push(IsLessThanTupleAuxCols::from_slice(
                    &cols[new_start + i * aux_size..new_start + (i + 1) * aux_size],
                    is_less_than_tuple_params.limb_bits.clone(),
                    is_less_than_tuple_params.decomp,
                    idx_len,
                ))
            }
            let subair_cols = LeafPageSubAirCols {
                key_start: aux_allocs[0].clone(),
                end_key: aux_allocs[1].clone(),
            };
            LeafPageMetadataCols {
                own_commitment: cols[0..commitment_len].to_vec(),
                range_inclusion_cols: Some(range_inclusion_cols),
                subchip_aux_cols: Some(subair_cols),
            }
        }
    }
}

// impl<T> LeafPageCacheCols<T> {
//     pub fn from_slice(cols: &[T], idx_len: usize, data_len: usize) -> Self
//     where
//         T: Clone,
//     {
//         LeafPageCacheCols {
//             is_alloc: cols[0].clone(),
//             idx: cols[1..idx_len + 1].to_vec(),
//             data: cols[idx_len + 1..idx_len + data_len + 1].to_vec(),
//         }
//     }
// }
