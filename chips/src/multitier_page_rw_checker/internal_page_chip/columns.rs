use crate::{
    is_equal::columns::IsEqualAuxCols,
    is_less_than_tuple::{
        self,
        columns::{IsLessThanTupleAuxCols, IsLessThanTupleCols},
    },
    multitier_page_rw_checker::page_controller::LessThanTupleParams,
};

#[derive(Clone)]
pub struct InternalPageCols<T> {
    pub cache_cols: PtrPageCols<T>,
    pub metadata: InternalPageMetadataCols<T>,
}

#[derive(Clone)]
pub struct PtrPageCols<T> {
    pub is_alloc: T,
    pub start: Vec<T>,
    pub end: Vec<T>,
    pub commitment: Vec<T>,
}

#[derive(Clone)]
pub struct InternalPageSubAirCols<T> {
    pub key1_start: IsLessThanTupleAuxCols<T>,
    pub end_key1: IsLessThanTupleAuxCols<T>,
    pub key2_start: IsLessThanTupleAuxCols<T>,
    pub end_key2: IsLessThanTupleAuxCols<T>,
    pub end_start: IsLessThanTupleAuxCols<T>,
    pub end_next: IsLessThanTupleAuxCols<T>,
    pub mult_inv: T,
}

// pub struct LeafPageCacheCols<T> {
//     pub is_alloc: T, // indicates if row is allocated
//     pub idx: Vec<T>,
//     pub data: Vec<T>,
// }
#[derive(Clone)]
pub struct TwoRangeInclusionCols<T> {
    pub start: Vec<T>,
    pub end: Vec<T>,
    pub less_than_start: (T, T),
    pub greater_than_end: (T, T),
}

#[derive(Clone)]
pub struct ProveSortCols<T> {
    pub next_key: Vec<T>,
    // we want this to be true
    pub end_less_than_next: T,
    // we want this to be false
    pub end_less_than_start: T,
}

#[derive(Clone)]
pub struct InternalPageMetadataCols<T> {
    pub own_commitment: Vec<T>,
    pub mult: T,
    pub mult_alloc: T,
    pub mult_alloc_minus_one: T,
    pub mult_alloc_is_1: T,
    pub prove_sort_cols: Option<ProveSortCols<T>>,
    pub range_inclusion_cols: Option<TwoRangeInclusionCols<T>>,
    pub subchip_aux_cols: Option<InternalPageSubAirCols<T>>,
}

impl<T> InternalPageCols<T> {
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
        InternalPageCols {
            cache_cols: PtrPageCols::from_slice(
                &cols[0..1 + 2 * idx_len + commitment_len],
                idx_len,
                commitment_len,
            ),
            metadata: InternalPageMetadataCols::from_slice(
                &cols[1 + 2 * idx_len + commitment_len..],
                idx_len,
                commitment_len,
                is_init,
                is_less_than_tuple_params,
            ),
        }
    }
}

impl<T> PtrPageCols<T> {
    pub fn from_slice(cols: &[T], idx_len: usize, commitment_len: usize) -> Self
    where
        T: Clone,
    {
        PtrPageCols {
            is_alloc: cols[0].clone(),
            start: cols[1..1 + idx_len].to_vec(),
            end: cols[1 + idx_len..1 + 2 * idx_len].to_vec(),
            commitment: cols[1 + 2 * idx_len..1 + 2 * idx_len + commitment_len].to_vec(),
        }
    }
}

impl<T> InternalPageMetadataCols<T> {
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
            InternalPageMetadataCols {
                own_commitment: cols[0..commitment_len].to_vec(),
                mult: cols[commitment_len].clone(),
                mult_alloc: cols[commitment_len + 1].clone(),
                mult_alloc_is_1: cols[commitment_len + 2].clone(),
                mult_alloc_minus_one: cols[commitment_len + 3].clone(),
                prove_sort_cols: None,
                range_inclusion_cols: None,
                subchip_aux_cols: None,
            }
        } else {
            let mut new_start = commitment_len + 4;
            let prove_sort_cols = ProveSortCols {
                next_key: cols[new_start..new_start + idx_len].to_vec(),
                end_less_than_next: cols[new_start + idx_len].clone(),
                end_less_than_start: cols[new_start + idx_len + 1].clone(),
            };
            new_start += idx_len + 2;
            let range_inclusion_cols = TwoRangeInclusionCols {
                start: cols[new_start..new_start + idx_len].to_vec(),
                end: cols[new_start + idx_len..new_start + 2 * idx_len].to_vec(),
                less_than_start: (
                    cols[new_start + 2 * idx_len].clone(),
                    cols[new_start + 2 * idx_len + 2].clone(),
                ),
                greater_than_end: (
                    cols[new_start + 2 * idx_len + 1].clone(),
                    cols[new_start + 2 * idx_len + 3].clone(),
                ),
            };
            new_start += 2 * idx_len + 4;
            let mut aux_allocs = vec![];
            let aux_size = IsLessThanTupleAuxCols::<T>::get_width(
                is_less_than_tuple_params.limb_bits.clone(),
                is_less_than_tuple_params.decomp,
                idx_len,
            );
            for i in 0..6 {
                aux_allocs.push(IsLessThanTupleAuxCols::from_slice(
                    &cols[new_start + i * aux_size..new_start + (i + 1) * aux_size],
                    is_less_than_tuple_params.limb_bits.clone(),
                    is_less_than_tuple_params.decomp,
                    idx_len,
                ))
            }
            let subair_cols = InternalPageSubAirCols {
                key1_start: aux_allocs[0].clone(),
                end_key1: aux_allocs[1].clone(),
                key2_start: aux_allocs[2].clone(),
                end_key2: aux_allocs[3].clone(),
                end_start: aux_allocs[4].clone(),
                end_next: aux_allocs[5].clone(),
                mult_inv: cols[new_start + 6 * aux_size].clone(),
            };
            InternalPageMetadataCols {
                own_commitment: cols[0..commitment_len].to_vec(),
                mult: cols[commitment_len].clone(),
                mult_alloc: cols[commitment_len + 1].clone(),
                mult_alloc_is_1: cols[commitment_len + 2].clone(),
                mult_alloc_minus_one: cols[commitment_len + 3].clone(),
                prove_sort_cols: Some(prove_sort_cols),
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
