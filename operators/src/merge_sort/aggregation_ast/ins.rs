use std::{
    cmp::{max, min},
    sync::Arc,
};

use afs_chips::common::page::{self, Page};
use afs_test_utils::config::baby_bear_poseidon2::BabyBearPoseidon2Config;
use itertools::Itertools;

use crate::merge_sort::{
    get_top_p::page_controller::PageController as GTPPageController,
    init_remaining::PageController as IRPageController,
    split_remaining::page_controller::PageController as SRPageController,
};

use super::DataFrameRow;

pub struct PageWithMetadata {
    page: Page,
    start: Vec<u32>,
    end: Vec<u32>,
    is_full: bool,
}

pub enum GetTopPCum {
    Empty(),
    GetTopP(),
    GetTopPCum2 { front: Arc<Self>, end: Arc<Self> },
}

pub enum InsTree {
    Main(MergeSort),
    FinishMerge(GetTopPCum),
    GetTopPCum(GetTopPCum),
    InitRemainingGTP(GetTopPCum),
    MergeSort(MergeSort),
    PreMerge(PreMerge),
}

pub enum MergeSort {
    Merge {
        pre: Arc<PreMerge>,
        finish: GetTopPCum,
    },
    Single(),
}

pub struct PreMerge {
    pub merge_sorts: Vec<Arc<MergeSort>>,
}

// please pad to K if 1 < len < K. Since K is usually 2, we don't care
pub fn gen_ins_tree_main(pages: &[usize], k: usize, page_height: usize) -> InsTree {
    assert!(k >= 2);
    let len = pages.len();
    assert!(len >= 1);
    InsTree::Main(gen_merge_sort(pages, k, page_height).0)
}

pub fn gen_merge_sort(pages: &[usize], k: usize, page_height: usize) -> (MergeSort, Vec<usize>) {
    let len = pages.len();
    if len == 1 {
        return (MergeSort::Single(), vec![pages[0]]);
    }
    let new_k = min(k, len);
    let splits = (0..new_k)
        .map(|i| &pages[i * len / new_k..(i + 1) * len / new_k])
        .collect_vec();
    let merges = splits
        .into_iter()
        .map(|i| gen_merge_sort(i, k, page_height))
        .collect_vec();
    let mut vecs = merges.iter().map(|(_, v)| v.clone()).collect_vec();
    vecs.resize(k, vec![0]);
    let lens = vecs.concat();

    let merges = merges.into_iter().map(|(m, _)| Arc::new(m)).collect_vec();
    let len = lens.len();
    let finish_merge = gen_get_top_p_cum_loop(len - k, k, page_height);
    let mut sum = 0;
    for l in &lens {
        sum += *l;
    }
    let mut new_lens = vec![page_height; sum / page_height];
    if new_lens.len() < lens.len() {
        new_lens.push(sum % page_height);
    }
    (
        MergeSort::Merge {
            pre: Arc::new(PreMerge {
                merge_sorts: merges,
            }),
            finish: finish_merge,
        },
        new_lens,
    )
}

pub fn gen_get_top_p_cum_loop(len: usize, k: usize, page_height: usize) -> GetTopPCum {
    if len == 0 {
        GetTopPCum::Empty()
    } else if len == 1 {
        GetTopPCum::GetTopP {}
    } else {
        GetTopPCum::GetTopPCum2 {
            front: Arc::new(gen_get_top_p_cum_loop(len / 2, k, page_height)),
            end: Arc::new(gen_get_top_p_cum_loop(len - len / 2, k, page_height)),
        }
    }
}
