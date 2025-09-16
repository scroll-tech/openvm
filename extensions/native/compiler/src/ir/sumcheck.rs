use openvm_native_compiler_derive::iter_zip;
use openvm_stark_backend::p3_field::FieldAlgebra;
use crate::ir::Variable;
use super::{Array, ArrayLike, Builder, Config, DslIr, Ext, Felt, MemIndex, Ptr, Usize, Var};

impl<C: Config> Builder<C> {
    /// Extends native VM ability to calculate the evaluation for a sumcheck layer
    pub fn sumcheck_layer_eval (
        &mut self,
        input_ctx: Array<C, Usize<C::N>>,
        challenges: Array<C, Ext<C::F, C::EF>>,
        prod_specs_eval: &Array<C, Ext<C::F, C::EF>>,
        logup_specs_eval: &Array<C, Ext<C::F, C::EF>>,
        // r_evals: &Array<C, Ext<C::F, C::EF>>,
    ) -> Usize<C::N> {
        self.operations.push(DslIr::SumcheckLayerEval(
            input_ctx,
            challenges,
            prod_specs_eval.ptr(), 
            logup_specs_eval.ptr(),
            // r_evals.ptr(),
        ));

        Usize::from(0)
    }
}