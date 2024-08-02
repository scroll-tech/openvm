use p3_baby_bear::BabyBear;
use p3_field::{AbstractField, TwoAdicField};

use afs_compiler::ir::{Builder, Config, Felt};
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
/// [AggregationVerifierProgram::build] builds the program for the aggregation verifier.
/// The two segments of interest must be adjacnt.
pub struct AggregationVerifierProgram<InnerConfig> {
    _phantom: std::marker::PhantomData<InnerConfig>,
}

impl AggregationVerifierProgram<InnerConfig> {
    /// Builds the program for the aggregation verifier.
    /// Calls verification on two segments in one call using [MultiStarkVerifier::verify].
    /// Then fetches the initial and final PCs for each segment and asserts that left_final_pc == right_init_pc.
    /// Finally, commits the initial PC for the left segment and the final PC for the right segment.
    pub fn build(
        raps: [Vec<&dyn DynRapForRecursion<InnerConfig>>; 2],
        constants: [MultiStarkVerificationAdvice<InnerConfig>; 2],
        fri_params: &FriParameters,
        // Whether the left segment is the native program (false) or has already been aggregated (true).
        left_agg: bool,
        // Whether the right segment is the native program (false) or has already been aggregated (true).
        right_agg: bool,
    ) -> Vec<Instruction<BabyBear>> {
        let mut builder = Builder::<InnerConfig>::default();

        let input: AggregationVerifierInputVariable<_> = builder.uninit();
        AggregationVerifierInput::<BabyBearPoseidon2Config>::witness(&input, &mut builder);

        let pcs = TwoAdicFriPcsVariable {
            config: const_fri_config(&mut builder, fri_params),
        };

        // Verify the two segments.
        MultiStarkVerifier::verify(&mut builder, &pcs, raps, constants, &input);

        let num_traces = input
            .verifier_input_1
            .public_values
            .len()
            .materialize(&mut builder);

        let first_init_pc: Felt<BabyBear> = builder.uninit();
        let first_final_pc: Felt<BabyBear> = builder.uninit();
        let second_init_pc: Felt<BabyBear> = builder.uninit();
        let second_final_pc: Felt<BabyBear> = builder.uninit();

        builder.range(0, num_traces).for_each(|i, builder| {
            let pv_1 = builder.get(&input.verifier_input_1.public_values, i);
            let pv_2 = builder.get(&input.verifier_input_2.public_values, i);

            builder
                .if_ne(pv_1.len(), <InnerConfig as Config>::F::zero())
                .then(|builder| {
                    let temp_first_init_pc = builder.get(&pv_1, 2 * (left_agg as usize));
                    builder.assign(first_init_pc, temp_first_init_pc);
                    let temp_first_final_pc = builder.get(&pv_1, 1 + 2 * (left_agg as usize));
                    builder.assign(first_final_pc, temp_first_final_pc);
                });

            builder
                .if_ne(pv_2.len(), <InnerConfig as Config>::F::zero())
                .then(|builder| {
                    let temp_second_init_pc = builder.get(&pv_2, 2 * (right_agg as usize));
                    builder.assign(second_init_pc, temp_second_init_pc);
                    let temp_second_final_pc = builder.get(&pv_2, 1 + 2 * (right_agg as usize));
                    builder.assign(second_final_pc, temp_second_final_pc);
                })
        });

        // Assert that the left segment's final PC is equal to the right segment's initial PC.
        builder.assert_felt_eq(first_final_pc, second_init_pc);

        // Commit the left segment's initial PC and the right segment's final PC, in indices 2 and 3.
        builder.commit_public_value(first_init_pc);
        builder.commit_public_value(second_final_pc);

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
    /// Unpackage the pairs and verify each segment according to StarkVerifier.
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
