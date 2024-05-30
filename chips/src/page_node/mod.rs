use std::cmp::max;

use crate::MAX_COMMITMENT_LEN;

use afs_stark_backend::interaction::Interaction;
use columns::{PageNodeBranch, PageNodeTerminal};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

pub struct PageNodeReadChip {
    path_bus_index: usize,
    val_bus_index: usize,
    key_len: usize,
    val_len: usize,
    limb_bits: usize,
    page_size: usize,
    page_data: Vec<Vec<u32>>,
    // forget about range checking stuff for now
    // pub range_chip: Mutex<RangeCheckerChip<MAX>>,
}

impl PageNodeReadChip {
    /// assumes keys have already been split into limb_bits
    pub fn new(
        val_bus_index: usize,
        path_bus_index: usize,
        key_len: usize,
        val_len: usize,
        limb_bits: usize,
        page: Vec<Vec<u32>>,
    ) -> Self {
        assert!(!page.is_empty());
        let node = PageNodeReadChip {
            path_bus_index,
            val_bus_index,
            key_len,
            val_len,
            limb_bits,
            page_size: page.len(),
            page_data: page,
        };
        assert!(node.page_data[0].len() >= node.get_width() - MAX_COMMITMENT_LEN - 4);
        node
    }

    pub fn path_bus_index(&self) -> usize {
        self.path_bus_index
    }

    pub fn val_bus_index(&self) -> usize {
        self.val_bus_index
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn get_width(&self) -> usize {
        6 + max(
            MAX_COMMITMENT_LEN + 2 * self.key_len,
            self.key_len + self.val_len,
        ) + MAX_COMMITMENT_LEN
    }

    // receives: ([key] | [val] | [commitment]) mult times
    // every node receives mult_p times on path_bus
    // every node receives mult_t times on val_bus
    pub fn receives_branch<F: PrimeField64>(
        &self,
        cols: PageNodeBranch<usize>,
    ) -> Vec<Interaction<F>> {
        let mut path_cols = vec![];
        for page_col in cols.commitment {
            path_cols.push(VirtualPairCol::single_main(page_col));
        }
        vec![Interaction {
            fields: path_cols,
            count: VirtualPairCol::single_main(cols.mult_p),
            argument_index: self.path_bus_index(),
        }]
    }

    pub fn receives_terminal<F: PrimeField64>(
        &self,
        cols: PageNodeTerminal<usize>,
    ) -> Vec<Interaction<F>> {
        let mut val_cols = vec![];
        for page_col in cols.key {
            val_cols.push(VirtualPairCol::single_main(page_col));
        }
        for page_col in cols.val {
            val_cols.push(VirtualPairCol::single_main(page_col));
        }
        vec![Interaction {
            fields: val_cols,
            count: VirtualPairCol::single_main(cols.mult_t),
            argument_index: self.val_bus_index(),
        }]
    }

    // sends: ([key] | [val] | [child_commitment]) mult times
    pub fn sends_branch<F: PrimeField64>(
        &self,
        cols: PageNodeBranch<usize>,
    ) -> Vec<Interaction<F>> {
        let mut virtual_cols = vec![];
        for page_col in cols.child_commitment {
            virtual_cols.push(VirtualPairCol::single_main(page_col));
        }
        vec![Interaction {
            fields: virtual_cols,
            count: VirtualPairCol::single_main(cols.mult_b),
            argument_index: self.path_bus_index(),
        }]
    }
}
