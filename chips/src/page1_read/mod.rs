use crate::MAX_COMMITMENT_LEN;

use self::columns::Page1ReadCols;
use afs_stark_backend::interaction::Interaction;
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

pub mod air;
pub mod chip;
pub mod columns;
pub mod trace;

pub struct Page1ReadChip {
    val_bus_index: usize,
    path_bus_index: usize,
    key_len: usize,
    limb_bits: usize,
    val_len: usize,
    page_size: usize,
    page_data: Vec<Vec<u32>>,
    // forget about range checking stuff for now
    // pub range_chip: Mutex<RangeCheckerChip<MAX>>,
}

impl Page1ReadChip {
    /// assumes keys have already been split into limb_bits
    pub fn new(
        val_bus_index: usize,
        path_bus_index: usize,
        key_len: usize,
        limb_bits: usize,
        val_len: usize,
        page: Vec<Vec<u32>>,
    ) -> Self {
        assert!(!page.is_empty());
        assert!(page[0].len() == 2 + key_len + val_len);
        assert!(val_bus_index != path_bus_index);
        Self {
            val_bus_index,
            path_bus_index,
            key_len,
            limb_bits,
            val_len,
            page_size: page.len(),
            page_data: page,
        }
    }

    pub fn val_bus_index(&self) -> usize {
        self.val_bus_index
    }

    pub fn path_bus_index(&self) -> usize {
        self.path_bus_index
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn get_width(&self) -> usize {
        6 + self.val_len + self.key_len + MAX_COMMITMENT_LEN
    }

    // receives: ([key] | [page] ) mult times
    pub fn receives_custom<F: PrimeField64>(
        &self,
        cols: Page1ReadCols<usize>,
    ) -> Vec<Interaction<F>> {
        let mut val_virtual_cols = vec![];
        for page_col in cols.key {
            val_virtual_cols.push(VirtualPairCol::single_main(page_col));
        }
        for page_col in cols.page_row {
            val_virtual_cols.push(VirtualPairCol::single_main(page_col));
        }

        let mut path_virtual_cols = vec![];
        for page_col in cols.commitment {
            path_virtual_cols.push(VirtualPairCol::single_main(page_col));
        }

        vec![
            Interaction {
                fields: val_virtual_cols,
                count: VirtualPairCol::single_main(cols.mult_t),
                argument_index: self.val_bus_index(),
            },
            Interaction {
                fields: path_virtual_cols,
                count: VirtualPairCol::single_main(cols.mult_t),
                argument_index: self.path_bus_index(),
            },
        ]
    }
}
