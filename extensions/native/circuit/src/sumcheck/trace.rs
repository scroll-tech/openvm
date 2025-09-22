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
use crate::sumcheck::{chip::NativeSumcheckChip, columns::{HeaderSpecificCols, LogupSpecificCols, NativeSumcheckCols, ProdSpecificCols}};

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
        NativeSumcheckCols::<F>::width()
    }
}

impl<F: PrimeField32> NativeSumcheckChip<F> {
    fn generate_trace(self) -> RowMajorMatrix<F> {
        let width = self.trace_width();
        let height = next_power_of_two_or_zero(self.height);
        let mut flat_trace: Vec<F> = F::zero_vec(width * height);

        let memory = self.offline_memory.lock().unwrap();
        let aux_cols_factory = memory.aux_cols_factory();

        let mut used_cells = 0;
        for record in self.record_set {
            let slice = &mut flat_trace[used_cells..used_cells + width];
            let cols: &mut NativeSumcheckCols<F> = slice.borrow_mut();
            cols.first_timestamp = F::from_canonical_u32(record.from_state.timestamp);
            cols.start_timestamp = F::from_canonical_usize(record.from_state.timestamp as usize + record.curr_timestamp_increment);
            cols.last_timestamp = F::from_canonical_usize(record.final_timestamp_increment);

            if record.row_type == 0 {
                cols.header_row = F::ONE;
                let header: &mut HeaderSpecificCols<F> =
                    cols.specific[..HeaderSpecificCols::<F>::width()].borrow_mut();
            } else if record.row_type == 1 {
                cols.prod_row = F::ONE;
                let prod: &mut ProdSpecificCols<F> =
                    cols.specific[..ProdSpecificCols::<F>::width()].borrow_mut();
            } else if record.row_type == 2 {
                cols.logup_row = F::ONE;
                let logup: &mut LogupSpecificCols<F> =
                    cols.specific[..LogupSpecificCols::<F>::width()].borrow_mut();
            } else {
                unreachable!()
            }
            
            used_cells += width;
        }

        RowMajorMatrix::new(flat_trace, width)
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