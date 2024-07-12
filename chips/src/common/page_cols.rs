use std::iter;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, derive_new::new)]
pub struct PageCols<T> {
    pub is_alloc: T, // indicates if row is allocated
    pub idx: Vec<T>,
    pub data: Vec<T>,
}

impl<T: Clone> PageCols<T> {
    pub fn from_slice(cols: &[T], idx_len: usize, data_len: usize) -> PageCols<T> {
        PageCols {
            is_alloc: cols[0].clone(),
            idx: cols[1..idx_len + 1].to_vec(),
            data: cols[idx_len + 1..idx_len + data_len + 1].to_vec(),
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        iter::once(self.is_alloc.clone())
            .chain(self.idx.clone())
            .chain(self.data.clone())
            .collect()
    }
}

impl PageCols<u32> {
    pub fn blank(idx_len: usize, data_len: usize) -> Self {
        PageCols {
            is_alloc: 0,
            idx: vec![0; idx_len],
            data: vec![0; data_len],
        }
    }
}

impl Ord for PageCols<u32> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is_alloc > other.is_alloc {
            return std::cmp::Ordering::Less;
        }
        if self.is_alloc < other.is_alloc {
            return std::cmp::Ordering::Greater;
        }
        let mut i = 0;
        while i < self.idx.len() && self.idx[i] == other.idx[i] {
            i += 1;
        }
        if i == self.idx.len() {
            std::cmp::Ordering::Equal
        } else if self.idx[i] > other.idx[i] {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

impl PartialOrd for PageCols<u32> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.is_alloc > other.is_alloc {
            return Some(std::cmp::Ordering::Less);
        }
        if self.is_alloc < other.is_alloc {
            return Some(std::cmp::Ordering::Greater);
        }
        let mut i = 0;
        while i < self.idx.len() && self.idx[i] == other.idx[i] {
            i += 1;
        }
        if i == self.idx.len() {
            Some(std::cmp::Ordering::Equal)
        } else if self.idx[i] > other.idx[i] {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}
