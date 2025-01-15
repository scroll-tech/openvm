use crate::ir::{Array, Builder, Config, DslIr, Felt, Usize, Var};

impl<C: Config> Builder<C> {
    pub fn verify_batch(
        &mut self,
        dimensions: &Array<C, Usize<C::F>>,
        opened_values: &Array<C, Array<C, Felt<C::F>>>,
        proof: &Array<C, Array<C, Felt<C::F>>>,
        index_bits: &Array<C, Var<C::N>>,
        commit: &Array<C, Felt<C::F>>,
    ) {
        self.push(DslIr::VerifyBatch(
            dimensions.clone(),
            opened_values.clone(),
            proof.clone(),
            index_bits.clone(),
            commit.clone(),
        ));
    }
}
