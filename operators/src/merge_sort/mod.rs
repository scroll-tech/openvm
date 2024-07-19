use afs_chips::{common::page::Page, page_btree::cmp};
use tracing::info_span;

pub mod aggregation_ast;
pub mod get_top_p;
pub mod init_remaining;
pub mod split_remaining;

pub fn deterministic_page_sort(pages: &[Page]) -> Page {
    let k = pages.len();
    let mut buf = vec![];
    let mut cur_idx = vec![0; k];
    let sort_span = info_span!("Sort k pages and merge").entered();
    loop {
        let mut best_idx = -1;
        let mut best_idx_is_alloc = 0;
        let mut smallest_start = &vec![];
        for (i, (page, idx)) in pages.iter().zip(cur_idx.iter()).enumerate() {
            if cur_idx[i] < page.rows.len()
                && (best_idx == -1
                    || cmp(&page[*idx].idx, smallest_start) < 0
                    || page[*idx].is_alloc == 1 && best_idx_is_alloc == 0)
                && !(page[*idx].is_alloc == 0 && best_idx_is_alloc == 1)
            {
                best_idx = i as i32;
                best_idx_is_alloc = page[*idx].is_alloc;
                smallest_start = &page[*idx].idx;
            }
        }
        if best_idx == -1 {
            break;
        } else {
            buf.push(pages[best_idx as usize][cur_idx[best_idx as usize]].clone());
            cur_idx[best_idx as usize] += 1;
        }
    }
    sort_span.exit();
    Page { rows: buf }
}
