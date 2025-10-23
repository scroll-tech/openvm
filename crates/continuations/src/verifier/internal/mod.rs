use openvm_circuit::arch::{instructions::program::Program, SystemConfig};
use openvm_native_compiler::{conversion::CompilerOptions, prelude::*};
use openvm_native_recursion::{
    fri::TwoAdicFriPcsVariable, hints::Hintable, types::new_from_inner_multi_vk,
    utils::const_fri_config,
};
use openvm_stark_sdk::{
    config::{baby_bear_poseidon2::BabyBearPoseidon2Config, FriParameters},
    openvm_stark_backend::keygen::types::MultiStarkVerifyingKey,
};

use crate::{
    verifier::{
        common::non_leaf::NonLeafVerifierVariables,
        internal::{
            types::{InternalVmVerifierExtraPvs, InternalVmVerifierInput, InternalVmVerifierPvs},
            vars::InternalVmVerifierInputVariable,
        },
    },
    C, F,
};

use openvm_circuit::{
    system::memory::tree::public_values::PUBLIC_VALUES_ADDRESS_SPACE_OFFSET,
};
use openvm_stark_sdk::{
    openvm_stark_backend::p3_util::log2_strict_usize,
};
use crate::{
    verifier::{
        leaf::types::UserPublicValuesRootProof,
        utils::VariableP2Compressor,
    },
};

pub mod types;
pub mod vars;

/// Config to generate internal VM verifier program.
pub struct InternalVmVerifierConfig {
    pub app_fri_params: FriParameters,
    pub app_system_config: SystemConfig,
    pub internal_fri_params: FriParameters,
    pub compiler_options: CompilerOptions,
}

impl InternalVmVerifierConfig {
    pub fn build_program(
        &self,
        app_vm_vk: &MultiStarkVerifyingKey<BabyBearPoseidon2Config>,
        internal_vm_vk: &MultiStarkVerifyingKey<BabyBearPoseidon2Config>,
    ) -> Program<F> {
        let app_advice = new_from_inner_multi_vk(app_vm_vk);
        let internal_advice = new_from_inner_multi_vk(internal_vm_vk);
        let mut builder = Builder::<C>::default();
        {
            builder.cycle_tracker_start("ReadProofsFromInput");
            let InternalVmVerifierInputVariable {
                self_program_commit,
                proofs,
            } = InternalVmVerifierInput::<BabyBearPoseidon2Config>::read(&mut builder);
            builder.cycle_tracker_end("ReadProofsFromInput");
            builder.cycle_tracker_start("InitializePcsConst");
            let app_pcs = TwoAdicFriPcsVariable {
                config: const_fri_config(&mut builder, &self.app_fri_params),
            };
            let internal_pcs = TwoAdicFriPcsVariable {
                config: const_fri_config(&mut builder, &self.internal_fri_params),
            };
            builder.cycle_tracker_end("InitializePcsConst");
            let non_leaf_verifier = NonLeafVerifierVariables {
                internal_program_commit: self_program_commit,
                app_pcs,
                app_advice,
                internal_pcs,
                internal_advice,
            };
            builder.cycle_tracker_start("VerifyProofs");
            let (vm_verifier_pvs, app_verifier_commit) =
                non_leaf_verifier.verify_internal_or_app_proofs(&mut builder, &proofs);

            let is_terminate = builder.cast_felt_to_var(vm_verifier_pvs.connector.is_terminate);
            builder.if_eq(is_terminate, RVar::one()).then(|builder| {
                let (pv_commit, expected_memory_root) =
                    self.verify_user_public_values_root(builder);
                builder.assert_eq::<[_; DIGEST_SIZE]>(vm_verifier_pvs.memory.final_root, expected_memory_root);
                builder.assign(&vm_verifier_pvs.public_values_commit, pv_commit);
            });

            builder.cycle_tracker_end("VerifyProofs");
            let pvs = InternalVmVerifierPvs {
                vm_verifier_pvs,
                extra_pvs: InternalVmVerifierExtraPvs {
                    internal_program_commit: self_program_commit,
                    app_verifier_commit,
                },
            };
            for pv in pvs.flatten() {
                builder.commit_public_value(pv);
            }

            builder.halt();
        }

        builder.compile_isa_with_options(self.compiler_options)
    }

    /// Read the public values root proof from the input stream and verify it.
    /// This verification must be consistent `openvm_circuit::system::memory::tree::public_values`.
    /// Returns the public values commit and the corresponding memory state root.
    fn verify_user_public_values_root(
        &self,
        builder: &mut Builder<C>,
    ) -> ([Felt<F>; DIGEST_SIZE], [Felt<F>; DIGEST_SIZE]) {
        let memory_dimensions = self.app_system_config.memory_config.memory_dimensions();
        let pv_as = PUBLIC_VALUES_ADDRESS_SPACE_OFFSET + memory_dimensions.as_offset;
        let pv_start_idx = memory_dimensions.label_to_index((pv_as, 0));
        let pv_height = log2_strict_usize(self.app_system_config.num_public_values / DIGEST_SIZE);
        let proof_len = memory_dimensions.overall_height() - pv_height;
        let idx_prefix = pv_start_idx >> pv_height;

        // Read the public values root proof from the input stream.
        let root_proof = UserPublicValuesRootProof::<F>::read(builder);
        builder.assert_eq::<Usize<_>>(root_proof.sibling_hashes.len(), Usize::from(proof_len));
        let mut curr_commit = root_proof.public_values_commit;
        // Share the same state array to avoid unnecessary allocations.
        let compressor = VariableP2Compressor::new(builder);
        for i in 0..proof_len {
            let sibling_hash = builder.get(&root_proof.sibling_hashes, i);
            let (l_hash, r_hash) = if idx_prefix & (1 << i) != 0 {
                (sibling_hash, curr_commit)
            } else {
                (curr_commit, sibling_hash)
            };
            curr_commit = compressor.compress(builder, &l_hash, &r_hash);
        }
        (root_proof.public_values_commit, curr_commit)
    }
}
