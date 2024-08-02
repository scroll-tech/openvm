use p3_baby_bear::BabyBear;
use p3_field::{AbstractField, TwoAdicField};

use afs_compiler::ir::{Array, Builder, Config, Felt};
use afs_test_utils::config::{baby_bear_poseidon2::BabyBearPoseidon2Config, FriParameters};
use stark_vm::cpu::trace::Instruction;

use super::stark::DynRapForRecursion;
use crate::fri::TwoAdicFriPcsVariable;
use crate::hints::Hintable;
use crate::stark::StarkVerifier;
use crate::types::{
    AggregationVerifierInput, AggregationVerifierInputVariable, InnerConfig,
    MultiStarkVerificationAdvice,
};

use crate::utils::const_fri_config;

#[derive(Debug, Clone, Copy)]
pub struct AggregationVerifierProgram<InnerConfig> {
    _phantom: std::marker::PhantomData<InnerConfig>,
}

impl AggregationVerifierProgram<InnerConfig> {
    pub fn build(
        raps: [Vec<&dyn DynRapForRecursion<InnerConfig>>; 2],
        constants: [MultiStarkVerificationAdvice<InnerConfig>; 2],
        fri_params: &FriParameters,
        left_agg: bool,
        right_agg: bool,
    ) -> Vec<Instruction<BabyBear>> {
        let mut builder = Builder::<InnerConfig>::default();

        let input: AggregationVerifierInputVariable<_> = builder.uninit();
        AggregationVerifierInput::<BabyBearPoseidon2Config>::witness(&input, &mut builder);

        let pcs = TwoAdicFriPcsVariable {
            config: const_fri_config(&mut builder, fri_params),
        };

        MultiStarkVerifier::verify(&mut builder, &pcs, raps, constants, &input);

        let mut init_pcs: Array<_, Felt<_>> = builder.dyn_array(2);
        let mut final_pcs: Array<_, Felt<_>> = builder.dyn_array(2);

        let num_traces = input
            .verifier_input_1
            .public_values
            .len()
            .materialize(&mut builder);

        builder.range(0, num_traces).for_each(|i, builder| {
            let pv_1 = builder.get(&input.verifier_input_1.public_values, i);
            let pv_2 = builder.get(&input.verifier_input_2.public_values, i);

            builder
                .if_ne(pv_1.len(), <InnerConfig as Config>::F::zero())
                .then(|builder| {
                    let init_pc = builder.get(&pv_1, 2 * (left_agg as usize));
                    let final_pc = builder.get(&pv_1, 1 + 2 * (left_agg as usize));
                    builder.set(&mut init_pcs, 0, init_pc);
                    builder.set(&mut final_pcs, 0, final_pc);
                });

            builder
                .if_ne(pv_2.len(), <InnerConfig as Config>::F::zero())
                .then(|builder| {
                    let init_pc = builder.get(&pv_2, 2 * (right_agg as usize));
                    let final_pc = builder.get(&pv_2, 1 + 2 * (right_agg as usize));
                    builder.set(&mut init_pcs, 1, init_pc);
                    builder.set(&mut final_pcs, 1, final_pc);
                })
        });

        let first_init_pc = builder.get(&init_pcs, 0);
        let first_final_pc = builder.get(&final_pcs, 0);
        let second_init_pc = builder.get(&init_pcs, 1);
        let second_final_pc = builder.get(&final_pcs, 1);

        builder.assert_felt_eq(first_final_pc, second_init_pc);
        // TODO: expose first_init, second_final as pvs

        builder.halt();

        const WORD_SIZE: usize = 1;
        builder.compile_isa::<WORD_SIZE>()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MultiStarkVerifier<C: Config> {
    _phantom: std::marker::PhantomData<C>,
}

impl<C: Config> MultiStarkVerifier<C>
where
    C::F: TwoAdicField,
{
    /// Reference: [afs_stark_backend::verifier::MultiTraceStarkVerifier::verify].
    pub fn verify(
        builder: &mut Builder<C>,
        pcs: &TwoAdicFriPcsVariable<C>,
        raps: [Vec<&dyn DynRapForRecursion<C>>; 2],
        constants: [MultiStarkVerificationAdvice<C>; 2],
        AggregationVerifierInputVariable {
            verifier_input_1,
            verifier_input_2,
        }: &AggregationVerifierInputVariable<C>,
    ) {
        let [raps_0, raps_1] = raps;
        let [constants_0, constants_1] = constants;
        StarkVerifier::verify(builder, pcs, raps_0, constants_0, verifier_input_1);
        StarkVerifier::verify(builder, pcs, raps_1, constants_1, verifier_input_2);
    }
}
