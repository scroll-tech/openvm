use afs_stark_backend::interaction::{AirBridge, Interaction};
use p3_air::VirtualPairCol;
use p3_field::PrimeField64;

use crate::multitier_readonly::page_requester::columns::make_col_map;

use super::PageRequester;

impl<F: PrimeField64> AirBridge<F> for PageRequester {
    fn receives(&self) -> Vec<Interaction<F>> {
        let mut data_cols = Vec::new();
        let cols = make_col_map(self.idx_len, self.data_len);
        for k in cols.idx {
            data_cols.push(VirtualPairCol::single_main(k));
        }
        for v in cols.data {
            data_cols.push(VirtualPairCol::single_main(v));
        }
        vec![Interaction {
            fields: data_cols,
            count: VirtualPairCol::constant(F::one()),
            argument_index: *self.data_bus_index(),
        }]
    }
}
