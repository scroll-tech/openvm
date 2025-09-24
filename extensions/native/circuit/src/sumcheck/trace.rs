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
use crate::{sumcheck::{chip::NativeSumcheckChip, columns::{HeaderSpecificCols, LogupSpecificCols, NativeSumcheckCols, ProdSpecificCols}}, EXT_DEG};

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
            cols.last_timestamp = F::from_canonical_usize(record.from_state.timestamp as usize + record.final_timestamp_increment);
            cols.register_ptrs = record.register_ptrs;
            cols.ctx = record.ctx;
            cols.prod_nested_len = record.ctx[4] * record.ctx[3];
            cols.logup_nested_len = record.ctx[6] * record.ctx[5];
            cols.challenges = record.challenges;
            cols.alpha = record.alpha;
            cols.max_round = record.max_round;
            cols.within_round_limit = if record.within_round_limit { F::ONE } else { F::ZERO };
            cols.should_acc = if record.should_acc { F::ONE } else { F::ZERO };
            cols.eval_acc = record.eval_acc;

            if record.row_type == 0 {
                cols.header_row = F::ONE;
                cols.header_continuation = if record.continuation { F::ONE } else { F::ZERO };
                let header: &mut HeaderSpecificCols<F> =
                    cols.specific[..HeaderSpecificCols::<F>::width()].borrow_mut();

                header.pc = F::from_canonical_u32(record.from_state.pc);
                header.registers = record.registers;
            
                for i in 0..7usize {
                    let mem_record = memory.record_by_id(record.read_data_records[i]);
                    aux_cols_factory.generate_read_aux(mem_record, &mut header.read_records[i]);
                }

                // write the final result
                let mem_record = memory.record_by_id(record.write_data_records[0]);
                aux_cols_factory.generate_write_aux(mem_record, &mut header.write_records);
            } else if record.row_type == 1 {
                cols.prod_row = F::ONE;
                cols.prod_continuation = if record.continuation { F::ONE } else { F::ZERO };
                cols.prod_row_within_max_round = if record.within_round_limit { F::ONE } else { F::ZERO };
                cols.prod_in_round_evaluation = if record.within_round_limit { record.ctx[7] } else { F::ZERO };
                cols.prod_next_round_evaluation = if record.within_round_limit { F::ONE - record.ctx[7] } else { F::ZERO };
                cols.prod_acc = if record.should_acc { F::ONE } else { F::ZERO };
                let prod: &mut ProdSpecificCols<F> =
                    cols.specific[..ProdSpecificCols::<F>::width()].borrow_mut();

                cols.curr_prod_n = F::from_canonical_usize(record.prod_spec_n + 1);
                cols.challenges[0..EXT_DEG].copy_from_slice(&record.alpha1);
                prod.p[0..EXT_DEG].copy_from_slice(&record.p1);
                prod.p[EXT_DEG..(EXT_DEG * 2)].copy_from_slice(&record.p2);
                prod.data_ptr = record.data_ptr;
                prod.acc_eval = record.acc_eval;

                // Read max_round
                let mem_record = memory.record_by_id(record.read_data_records[0]);
                aux_cols_factory.generate_read_aux(mem_record, &mut prod.read_records[0]);

                if record.within_round_limit {
                    // Read p1, p2
                    let mem_record = memory.record_by_id(record.read_data_records[1]);
                    aux_cols_factory.generate_read_aux(mem_record, &mut prod.read_records[1]);

                    // Write p eval
                    prod.p_evals = record.p_evals;
                    let mem_record = memory.record_by_id(record.write_data_records[0]);
                    aux_cols_factory.generate_write_aux(mem_record, &mut prod.write_record);
                }
            } else if record.row_type == 2 {
                cols.logup_row = F::ONE;
                cols.logup_continuation = if record.continuation { F::ONE } else { F::ZERO };
                cols.logup_row_within_max_round = if record.within_round_limit { F::ONE } else { F::ZERO };
                cols.logup_in_round_evaluation = if record.within_round_limit { record.ctx[7] } else { F::ZERO };
                cols.logup_next_round_evaluation = if record.within_round_limit { F::ONE - record.ctx[7] } else { F::ZERO };
                cols.logup_acc = if record.should_acc { F::ONE } else { F::ZERO };
                let logup: &mut LogupSpecificCols<F> =
                    cols.specific[..LogupSpecificCols::<F>::width()].borrow_mut();

                cols.curr_logup_n = F::from_canonical_usize(record.logup_spec_n + 1);
                cols.challenges[0..EXT_DEG].copy_from_slice(&record.alpha1);
                cols.challenges[(EXT_DEG * 3)..(EXT_DEG * 4)].copy_from_slice(&record.alpha2);
                logup.pq[0..EXT_DEG].copy_from_slice(&record.p1);
                logup.pq[EXT_DEG..(EXT_DEG * 2)].copy_from_slice(&record.p2);
                logup.pq[(EXT_DEG * 2)..(EXT_DEG * 3)].copy_from_slice(&record.q1);
                logup.pq[(EXT_DEG * 3)..(EXT_DEG * 4)].copy_from_slice(&record.q2);
                logup.data_ptr = record.data_ptr;
                logup.acc_eval = record.acc_eval;

                // Read max_round
                let mem_record = memory.record_by_id(record.read_data_records[0]);
                aux_cols_factory.generate_read_aux(mem_record, &mut logup.read_records[0]);

                if record.within_round_limit {
                    // Read p1, p2, q1, q2
                    let mem_record = memory.record_by_id(record.read_data_records[1]);
                    aux_cols_factory.generate_read_aux(mem_record, &mut logup.read_records[1]);

                    // Write p and q eval
                    logup.p_evals = record.p_evals;
                    logup.q_evals = record.q_evals;
                    for i in 0..2usize {
                        let mem_record = memory.record_by_id(record.write_data_records[i]);
                        aux_cols_factory.generate_write_aux(mem_record, &mut logup.write_records[i]);
                    }
                }
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