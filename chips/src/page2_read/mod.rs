use crate::MAX_COMMITMENT_LEN;

use self::columns::Page2ReadCols;
use afs_stark_backend::interaction::Interaction;
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

pub struct Page2ReadChip {
    path_bus_index: usize,
    key_len: usize,
    limb_bits: usize,
    page_size: usize,
    page_data: Vec<Vec<u32>>,
    // forget about range checking stuff for now
    // pub range_chip: Mutex<RangeCheckerChip<MAX>>,
}

impl Page2ReadChip {
    /// assumes keys have already been split into limb_bits
    pub fn new(
        path_bus_index: usize,
        key_len: usize,
        limb_bits: usize,
        page: Vec<Vec<u32>>,
    ) -> Self {
        assert!(!page.is_empty());
        assert_eq!(page[0].len(), 2 + 2 * key_len + MAX_COMMITMENT_LEN);
        Self {
            path_bus_index,
            key_len,
            limb_bits,
            page_size: page.len(),
            page_data: page,
        }
    }

    pub fn path_bus_index(&self) -> usize {
        self.path_bus_index
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn get_width(&self) -> usize {
        6 + 2 * MAX_COMMITMENT_LEN + 2 * self.key_len
    }

    // receives: ([key] | [val] | [commitment]) mult times
    pub fn receives_custom<F: PrimeField64>(
        &self,
        cols: Page2ReadCols<usize>,
    ) -> Vec<Interaction<F>> {
        let mut virtual_cols = vec![];
        for page_col in cols.commitment {
            virtual_cols.push(VirtualPairCol::single_main(page_col));
        }
        vec![Interaction {
            fields: virtual_cols,
            count: VirtualPairCol::single_main(cols.mult),
            argument_index: self.path_bus_index(),
        }]
    }

    // sends: ([key] | [val] | [child_commitment]) mult times
    pub fn sends_custom<F: PrimeField64>(&self, cols: Page2ReadCols<usize>) -> Vec<Interaction<F>> {
        let mut virtual_cols = vec![];
        for page_col in cols.child_commitment {
            virtual_cols.push(VirtualPairCol::single_main(page_col));
        }
        vec![Interaction {
            fields: virtual_cols,
            count: VirtualPairCol::single_main(cols.mult),
            argument_index: self.path_bus_index(),
        }]
    }
}
