pub struct InternalPageCols<T> {
    pub cache_cols: InternalDataCols<T>,
    pub metadata: InternalPageMetadataCols<T>,
}

pub struct InternalDataCols<T> {
    pub is_leaf: T,
    pub is_alloc: T,
    pub start: Vec<T>,
    pub end: Vec<T>,
    pub child_commit: Vec<T>,
}

pub struct InternalPageMetadataCols<T> {
    pub commitment: Vec<T>,
    pub mult: T,
    pub mult_alloc: T,
}

/// Columns are
/// [key] | [value] | [commitment] | [mult]
impl<T: Clone> InternalPageCols<T> {
    pub fn from_slice(cols: &[T], idx_len: usize, commitment_len: usize) -> Self {
        Self {
            cache_cols: InternalDataCols::from_slice(
                &cols[0..2 + 2 * idx_len + commitment_len].to_vec(),
                idx_len,
                commitment_len,
            ),
            metadata: InternalPageMetadataCols::from_slice(
                &cols[2 + 2 * idx_len + commitment_len..4 + 2 * idx_len + 2 * commitment_len]
                    .to_vec(),
                commitment_len,
            ),
        }
    }
}

impl<T: Clone> InternalDataCols<T> {
    pub fn from_slice(cols: &[T], idx_len: usize, commitment_len: usize) -> Self {
        Self {
            is_leaf: cols[0].clone(),
            is_alloc: cols[1].clone(),
            start: cols[2..2 + idx_len].to_vec(),
            end: cols[2 + idx_len..2 + 2 * idx_len].to_vec(),
            child_commit: cols[2 + 2 * idx_len..2 + 2 * idx_len + commitment_len].to_vec(),
        }
    }
}

impl<T: Clone> InternalPageMetadataCols<T> {
    pub fn from_slice(cols: &[T], commitment_len: usize) -> Self {
        Self {
            commitment: cols[0..commitment_len].to_vec(),
            mult: cols[commitment_len].clone(),
            mult_alloc: cols[commitment_len + 1].clone(),
        }
    }
}
