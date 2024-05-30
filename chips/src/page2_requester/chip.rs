use afs_stark_backend::interaction::{Chip, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use crate::page2_requester::columns::make_col_map;

use super::Page2Requester;

impl<F: PrimeField64> Chip<F> for Page2Requester {
    fn sends(&self) -> Vec<Interaction<F>> {
        let mut val_cols = Vec::new();
        let mut path_cols = Vec::new();
        let cols = make_col_map(self.key_len, self.val_len);
        for k in cols.k {
            val_cols.push(VirtualPairCol::single_main(k));
        }
        for v in cols.v {
            val_cols.push(VirtualPairCol::single_main(v));
        }
        for c in cols.c {
            path_cols.push(VirtualPairCol::single_main(c));
        }
        vec![
            Interaction {
                fields: val_cols,
                count: VirtualPairCol::constant(F::one()),
                argument_index: self.val_bus_index(),
            },
            Interaction {
                fields: path_cols,
                count: VirtualPairCol::constant(F::one()),
                argument_index: self.path_bus_index(),
            },
        ]
    }
}
