use openvm_circuit::arch::instructions::program::Program;
use openvm_native_compiler::{
    conversion::CompilerOptions,
    ir::{Builder, Felt, RVar, Usize, DIGEST_SIZE},
};
use openvm_native_recursion::{
    challenger::duplex::DuplexChallengerVariable, fri::TwoAdicFriPcsVariable, hints::Hintable,
    stark::StarkVerifier, types::new_from_inner_multi_vk, utils::const_fri_config,
};
use openvm_stark_backend::{keygen::types::MultiStarkVerifyingKey, p3_field::FieldAlgebra};
use openvm_stark_sdk::config::FriParameters;

use super::{
    common::{
        assert_or_assign_connector_pvs, assert_or_assign_memory_pvs,
        assert_required_air_for_app_vm_present, get_connector_pvs, get_memory_pvs,
        get_program_commit, types::VmVerifierPvs,
    },
    root::{
        compute_exe_commit,
        types::{RootVmVerifierInput, RootVmVerifierPvs},
        vars::RootVmVerifierInputVariable,
    },
    utils::VariableP2Hasher,
};
use crate::{C, F, SC};

pub mod types;
mod vars;

pub struct MinimalVmVerifierConfig {
    pub leaf_fri_params: FriParameters,
    // pub app_system_config: SystemConfig,
    pub num_public_values: usize,
    pub compiler_options: CompilerOptions,
}

impl MinimalVmVerifierConfig {
    pub fn build_program(&self, app_vm_vk: &MultiStarkVerifyingKey<SC>) -> Program<F> {
        let m_advice = new_from_inner_multi_vk(app_vm_vk);
        let mut builder = Builder::<C>::default();

        {
            builder.cycle_tracker_start("InitializePcsConst");
            let pcs = TwoAdicFriPcsVariable {
                config: const_fri_config(&mut builder, &self.leaf_fri_params),
            };
            // builder.print_debug();
            builder.cycle_tracker_end("InitializePcsConst");
            builder.cycle_tracker_start("ReadProofsFromInput");
            // let proofs: Array<C, StarkProofVariable<_>> =
            //     <Vec<Proof<BabyBearPoseidon2Config>> as Hintable<C>>::read(&mut builder);
            let RootVmVerifierInputVariable {
                proofs,
                public_values, // This is an Array<C, Felt<C::F>>
            } = RootVmVerifierInput::<SC>::read(&mut builder);
            // At least 1 proof should be provided.
            builder.assert_ne::<Usize<_>>(proofs.len(), RVar::zero());
            builder.cycle_tracker_end("ReadProofsFromInput");

            builder.cycle_tracker_start("VerifyProofs");
            let pvs = VmVerifierPvs::<Felt<F>>::uninit(&mut builder);
            builder.range(0, proofs.len()).for_each(|i, builder| {
                let proof = builder.get(&proofs, i);
                assert_required_air_for_app_vm_present(builder, &proof);
                StarkVerifier::verify::<DuplexChallengerVariable<C>>(
                    builder, &pcs, &m_advice, &proof,
                );
                {
                    let commit = get_program_commit(builder, &proof);
                    builder.if_eq(i, RVar::zero()).then_or_else(
                        |builder| {
                            builder.assign(&pvs.app_commit, commit);
                        },
                        |builder| builder.assert_eq::<[_; DIGEST_SIZE]>(pvs.app_commit, commit),
                    );
                }

                let proof_connector_pvs = get_connector_pvs(builder, &proof);
                assert_or_assign_connector_pvs(builder, &pvs.connector, i, &proof_connector_pvs);

                let proof_memory_pvs = get_memory_pvs(builder, &proof);
                assert_or_assign_memory_pvs(builder, &pvs.memory, i, &proof_memory_pvs);
            });
            builder.cycle_tracker_end("VerifyProofs");

            // App Program should terminate
            builder.assert_felt_eq(pvs.connector.is_terminate, F::ONE);
            // App Program should exit successfully
            builder.assert_felt_eq(pvs.connector.exit_code, F::ZERO);

            builder.cycle_tracker_start("ExtractPublicValues");
            let public_values_vec: Vec<Felt<F>> = (0..self.num_public_values)
                .map(|i| builder.get(&public_values, i))
                .collect();
            let hasher = VariableP2Hasher::new(&mut builder);
            let pv_commit = hasher.merkle_root(&mut builder, &public_values_vec);
            builder.cycle_tracker_end("ExtractPublicValues");

            let root_pvs = RootVmVerifierPvs {
                exe_commit: compute_exe_commit(
                    &mut builder,
                    &hasher,
                    pvs.app_commit,
                    pvs.memory.initial_root,
                    pvs.connector.initial_pc,
                ),
                leaf_verifier_commit: pvs.app_commit,
                public_values: public_values_vec,
            };
            root_pvs
                .flatten()
                .into_iter()
                .for_each(|v| builder.commit_public_value(v));

            builder.halt();
        }

        builder.compile_isa_with_options(self.compiler_options)
    }
}
