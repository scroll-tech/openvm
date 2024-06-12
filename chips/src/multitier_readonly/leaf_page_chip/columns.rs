pub struct LeafPageCols<T> {
    pub cache_cols: LeafDataCols<T>,
    pub metadata: LeafPageMetadataCols<T>,
}

pub struct LeafDataCols<T> {
    pub is_leaf: T,
    pub is_alloc: T,
    pub idx: Vec<T>,
    pub data: Vec<T>,
}

pub struct LeafPageMetadataCols<T> {
    pub commitment: Vec<T>,
    pub mult: T,
    pub mult_alloc: T,
}

/// Columns are
/// [key] | [value] | [commitment] | [mult]
impl<T: Clone> LeafPageCols<T> {
    pub fn from_slice(cols: &[T], idx_len: usize, data_len: usize, commitment_len: usize) -> Self {
        Self {
            cache_cols: LeafDataCols::from_slice(
                &cols[0..2 + idx_len + data_len],
                idx_len,
                data_len,
            ),
            metadata: LeafPageMetadataCols::from_slice(
                &cols[2 + idx_len + data_len..4 + idx_len + data_len + commitment_len],
                commitment_len,
            ),
        }
    }
}

impl<T: Clone> LeafDataCols<T> {
    pub fn from_slice(cols: &[T], idx_len: usize, data_len: usize) -> Self {
        Self {
            is_leaf: cols[0].clone(),
            is_alloc: cols[1].clone(),
            idx: cols[2..2 + idx_len].to_vec(),
            data: cols[2 + idx_len..2 + idx_len + data_len].to_vec(),
        }
    }
}

impl<T: Clone> LeafPageMetadataCols<T> {
    pub fn from_slice(cols: &[T], commitment_len: usize) -> Self {
        Self {
            commitment: cols[0..commitment_len].to_vec(),
            mult: cols[commitment_len].clone(),
            mult_alloc: cols[commitment_len + 1].clone(),
        }
    }
}
