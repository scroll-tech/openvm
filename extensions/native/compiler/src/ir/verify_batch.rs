use crate::ir::{Array, Builder, Config, DslIr, Ext, Felt, Usize, Var};

impl<C: Config> Builder<C> {
    pub fn verify_batch_felt(
        &mut self,
        dimensions: &Array<C, Usize<C::F>>,
        opened_values: &Array<C, Array<C, Felt<C::F>>>,
        proof: &Array<C, Array<C, Felt<C::F>>>,
        index_bits: &Array<C, Var<C::N>>,
        commit: &Array<C, Felt<C::F>>,
    ) {
        self.push(DslIr::VerifyBatchFelt(
            dimensions.clone(),
            opened_values.clone(),
            proof.clone(),
            index_bits.clone(),
            commit.clone(),
        ));
    }
    pub fn verify_batch_ext(
        &mut self,
        dimensions: &Array<C, Usize<C::F>>,
        opened_values: &Array<C, Array<C, Ext<C::F, C::EF>>>,
        proof: &Array<C, Array<C, Felt<C::F>>>,
        index_bits: &Array<C, Var<C::N>>,
        commit: &Array<C, Felt<C::F>>,
    ) {
        self.push(DslIr::VerifyBatchExt(
            dimensions.clone(),
            opened_values.clone(),
            proof.clone(),
            index_bits.clone(),
            commit.clone(),
        ));
    }
}
