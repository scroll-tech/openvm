use std::{borrow::BorrowMut, sync::Arc};

use openvm_circuit::system::memory::{MemoryAuxColsFactory, OfflineMemory};
use openvm_circuit_primitives::utils::next_power_of_two_or_zero;
use openvm_instructions::{instruction::Instruction, LocalOpcode};
use openvm_native_compiler::Poseidon2Opcode::COMP_POS2;
use openvm_stark_backend::{
    config::{StarkGenericConfig, Val},
    p3_air::BaseAir,
    p3_field::{Field, PrimeField32},
    p3_matrix::dense::RowMajorMatrix,
    p3_maybe_rayon::prelude::*,
    prover::types::AirProofInput,
    AirRef, Chip, ChipUsageGetter,
};
use crate::sumcheck::chip::NativeSumcheckChip;

impl<F: PrimeField32> NativeSumcheckChip<F> {
    fn generate_trace(self) -> RowMajorMatrix<F> {
        let width = self.trace_width();
        let height = next_power_of_two_or_zero(self.height);
        let mut flat_trace = F::zero_vec(width * height);
        let memory = self.offline_memory.lock().unwrap();
        let aux_cols_factory = memory.aux_cols_factory();
        let mut used_cells = 0;

        RowMajorMatrix::new(flat_trace, width)
    }
}

impl<F: Field> ChipUsageGetter
    for NativeSumcheckChip<F>
{
    fn air_name(&self) -> String {
        "SumcheckLayerEval".to_string()
    }

    fn current_trace_height(&self) -> usize {
        self.height
    }

    fn trace_width(&self) -> usize {
        // _debug
        0
    }
}

impl<SC: StarkGenericConfig> Chip<SC>
    for NativeSumcheckChip<Val<SC>>
where
    Val<SC>: PrimeField32,
{
    fn air(&self) -> AirRef<SC> {
        Arc::new(self.air.clone())
    }
    fn generate_air_proof_input(self) -> AirProofInput<SC> {
        AirProofInput::simple_no_pis(self.generate_trace())
    }
}