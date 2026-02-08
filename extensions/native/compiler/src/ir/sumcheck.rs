use super::{Array, Builder, Config, DslIr, Ext, Usize, Var};

impl<C: Config> Builder<C> {
    /// Extends native VM ability to calculate the evaluation for a sumcheck layer
    /// This opcode supports two modes (indicated by a context variable):
    /// 1. calculate the expected evaluation of two types of sumchecks (prod, logup)
    /// 2. calculate the expected value of next layer p[r] = eq(0,r)*p[0] + eq(1,r)*p[1]
    ///
    /// Context variables
    ///
    /// 0: round,
    /// 1: number of product
    /// 2. number of logup
    /// 3. (3D array description) prod_specs_eval inner length
    /// 4. (3D array description) prod_specs_eval inner_inner length
    /// 5. (3D array description) logup_spec_eval inner length
    /// 6. (3D array description) logup_spec_eval inner length
    /// 7. Operational mode indicator
    /// 8+ Additional usize-type variables indicating maximum rounds
    ///
    /// Output
    ///
    /// 1. for computing expected evaluation, output = [ \sum_i alpha^i * prod[i][0] * prod[i][1] +
    ///    \sum_j alpha^(2j) * (logup_q[i][0] * logup_q[i][1] + alpha* logup_p[i][0] * logup_q[i][1]
    ///    + alpha * logup_p[i][1] * logup_q[i][0] ];
    ///
    /// 2. for computing expected eval of next layer, output[1+i] = eq(0,r)*p[i][0] + eq(1,r) *
    ///    p[i][1].
    pub fn sumcheck_layer_eval(
        &mut self,
        input_ctx: &Array<C, Usize<C::N>>, // Context variables
        challenges: &Array<C, Ext<C::F, C::EF>>, // Challenges
        prod_specs_eval: &Array<C, Ext<C::F, C::EF>>, /* GKR product IOP evaluations. Flattened
                                            * from 3D array. */
        logup_specs_eval: &Array<C, Ext<C::F, C::EF>>, /* GKR logup IOP evaluations. Flattened
                                                        * from 3D array. */
        prod_specs_eval_id: Var<C::N>, /* ID for GKR product IOP evaluations hint. */
        logup_specs_eval_id: Var<C::N>, /* ID for GKR logup IOP evaluations hint. */
        r_evals: &Array<C, Ext<C::F, C::EF>>, /* Next layer's evaluations (pointer used for
                                        * storing opcode output) */
    ) {
        self.operations.push(DslIr::SumcheckLayerEval(
            input_ctx.ptr(),
            challenges.ptr(),
            prod_specs_eval.ptr(),
            logup_specs_eval.ptr(),
            prod_specs_eval_id,
            logup_specs_eval_id,
            r_evals.ptr(),
        ));
    }
}
