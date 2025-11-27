use std::sync::{Arc, Mutex};

use openvm_circuit::{
    arch::{
        CustomBorrow, ExecutionBridge, ExecutionError, ExecutionState, MultiRowLayout,
        MultiRowMetadata, PreflightExecutor, RecordArena, Streams, TraceFiller, VmChipWrapper,
        VmStateMut,
    },
    system::memory::{online::TracingMemory, MemoryAuxColsFactory, MemoryController},
};
use openvm_instructions::{instruction::Instruction, program::DEFAULT_PC_STEP, LocalOpcode};
use openvm_native_compiler::{conversion::AS, SumcheckOpcode::SUMCHECK_LAYER_EVAL};
use openvm_stark_backend::{
    p3_field::{Field, PrimeField, PrimeField32},
    p3_maybe_rayon::prelude::{ParallelIterator, ParallelSlice},
};
use serde::{Deserialize, Serialize};

use crate::{
    field_extension::{FieldExtension, EXT_DEG},
    fri::elem_to_ext,
    sumcheck::{
        air::NativeSumcheckAir,
        columns::{HeaderSpecificCols, LogupSpecificCols, NativeSumcheckCols, ProdSpecificCols},
    },
    utils::const_max,
};
const CONTEXT_ARR_BASE_LEN: usize = EXT_DEG * 2;

pub(crate) fn calculate_3d_ext_idx(
    inner_inner_len: u32,
    inner_len: u32,
    outer_idx: u32,
    inner_idx: u32,
    inner_inner_idx: u32,
) -> u32 {
    (inner_inner_len * inner_len * outer_idx + inner_inner_len * inner_idx + inner_inner_idx)
        * EXT_DEG as u32
}

#[derive(Debug, Clone, Default)]
pub struct NativeSumcheckMetadata {
    num_rows: usize,
}

impl MultiRowMetadata for NativeSumcheckMetadata {
    #[inline(always)]
    fn get_num_rows(&self) -> usize {
        self.num_rows
    }
}

type NativeSumcheckRecordLayout = MultiRowLayout<NativeSumcheckMetadata>;

pub struct NativeSumcheckRecordMut<'a, F>(&'a mut [NativeSumcheckCols<F>]);

impl<'a, F: PrimeField32>
    CustomBorrow<'a, NativeSumcheckRecordMut<'a, F>, NativeSumcheckRecordLayout> for [u8]
{
    fn custom_borrow(
        &'a mut self,
        layout: NativeSumcheckRecordLayout,
    ) -> NativeSumcheckRecordMut<'a, F> {
        // SAFETY:
        // - align_to_mut() ensures proper alignment for NativeSumcheckCols<F>
        // - Layout guarantees sufficient length for num_rows records
        // - Slice bounds validated by taking only num_rows elements
        let arr = unsafe { self.align_to_mut::<NativeSumcheckCols<F>>().1 };
        NativeSumcheckRecordMut(&mut arr[..layout.metadata.num_rows])
    }

    unsafe fn extract_layout(&self) -> NativeSumcheckRecordLayout {
        // Each instruction record consists solely of some number of contiguously
        // stored NativeSumcheckCols<...> structs, each of which corresponds to a
        // single trace row. Trace fillers don't actually need to know how many rows
        // each instruction uses, and can thus treat each NativePoseidon2Cols<...>
        // as a single record.
        NativeSumcheckRecordLayout {
            metadata: NativeSumcheckMetadata { num_rows: 1 },
        }
    }
}

#[derive(derive_new::new, Copy, Clone)]
pub struct NativeSumcheckExecutor;

#[derive(derive_new::new)]
pub struct NativeSumcheckFiller;

pub type NativeSumcheckChip<F> = VmChipWrapper<F, NativeSumcheckFiller>;

impl Default for NativeSumcheckExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl<F, RA> PreflightExecutor<F, RA> for NativeSumcheckExecutor
where
    F: PrimeField32,
    for<'buf> RA: RecordArena<'buf, NativeSumcheckRecordLayout, NativeSumcheckRecordMut<'buf, F>>,
{
    fn execute(
        &self,
        state: VmStateMut<F, TracingMemory, RA>,
        instruction: &Instruction<F>,
    ) -> Result<(), ExecutionError> {
        let &Instruction {
            opcode: op,
            a: output_register,
            b: input_register_1,
            c: input_register_2,
            d: data_address_space,
            e: register_address_space,
            f: input_register_3,
            g: input_register_4,
        } = instruction;

        if op == SUMCHECK_LAYER_EVAL.global_opcode() {
            /*
            let mut observation_records: Vec<SumcheckEvalRecord<F>> = vec![];
            let mut curr_timestamp: usize = 0;

            let (read_ctx_pointer, ctx_pointer) =
                memory.read_cell(register_address_space, input_register_1);
            let (read_cs_pointer, cs_pointer) =
                memory.read_cell(register_address_space, input_register_2);
            let (read_prod_pointer, prod_ptr) =
                memory.read_cell(register_address_space, input_register_3);
            let (read_logup_pointer, logup_ptr) =
                memory.read_cell(register_address_space, input_register_4);
            let (read_result_pointer, r_ptr) =
                memory.read_cell(register_address_space, output_register);
            let register_ptrs: [F; 5] = [ctx_pointer, cs_pointer, prod_ptr, logup_ptr, r_ptr];

            let (ctx_read, ctx): (RecordId, [F; EXT_DEG * 2]) =
                memory.read::<{ EXT_DEG * 2 }>(data_address_space, ctx_pointer);
            let [
                round,
                num_prod_spec,
                num_logup_spec,
                prod_specs_inner_len,
                prod_specs_inner_inner_len,
                logup_specs_inner_len,
                logup_specs_inner_inner_len,
                is_op_for_cur_sumcheck_round,    // This opcode supports two modes of operation:
                                                    // 1. calculate the expected evaluation of two types of sumchecks for the current round
                                                    //      a. product sumcheck: v' = v[0] * v[1]
                                                    //      b. logup sumcheck: p'= p[0] * q[1] + p[1] * q[0] and q'= q[0] * q[1].
                                                    // 2. calculate the expected value of next layer:
                                                    //      a. product sumcheck: v[r] = eq(0,r) * v[0] + eq(1,r) * v[1]
                                                    //      b. logup sumcheck: p[r] = eq(0,r) * p[0] + eq(1,r) * p[1] and q[r] = eq(0,r) * q[0] + eq(1,r) * q[1]
            ] = ctx;

            let (challenges_read, challenges): (RecordId, [F; EXT_DEG * 4]) =
                memory.read::<{ EXT_DEG * 4 }>(data_address_space, cs_pointer);
            let alpha: [F; 4] = challenges[0..EXT_DEG].try_into().expect("");

            let mut header_row = SumcheckEvalRecord {
                from_state,
                instruction: instruction.clone(),
                row_type: 0,
                continuation: true,
                curr_timestamp_increment: curr_timestamp,
                register_ptrs,
                alpha,
                registers: [
                    input_register_1,
                    input_register_2,
                    input_register_3,
                    input_register_4,
                    output_register,
                ],
                ctx,
                challenges,
                read_data_records: [
                    read_ctx_pointer,
                    read_cs_pointer,
                    read_prod_pointer,
                    read_logup_pointer,
                    read_result_pointer,
                    ctx_read,
                    challenges_read,
                ],
                ..Default::default()
            };

            observation_records.push(header_row);
            self.height += 1;
            curr_timestamp += 7;

            let mut eval_acc = elem_to_ext(F::from_canonical_u32(0));
            let mut alpha_acc = elem_to_ext(F::from_canonical_u32(1));
            let c1: [F; 4] = challenges[EXT_DEG..(EXT_DEG * 2)].try_into().expect("");
            let c2: [F; 4] = challenges[(EXT_DEG * 2)..(EXT_DEG * 3)]
                .try_into()
                .expect("");

            let mut i = F::ZERO;
            let mut i_usize = 0usize;
            while i < num_prod_spec {
                let mut prod_row: SumcheckEvalRecord<F> = SumcheckEvalRecord {
                    from_state,
                    instruction: instruction.clone(),
                    row_type: 1,
                    continuation: true,
                    curr_timestamp_increment: curr_timestamp,
                    register_ptrs,
                    ctx,
                    challenges,
                    alpha,
                    prod_spec_n: i_usize,
                    ..Default::default()
                };
                prod_row.alpha1 = alpha_acc;

                let (read_max_round, max_round) = memory.read_cell(
                    data_address_space,
                    ctx_pointer + F::from_canonical_usize(CONTEXT_ARR_BASE_LEN) + i,
                );
                prod_row.max_round = max_round;
                prod_row.read_data_records[0] = read_max_round;
                curr_timestamp += 1;

                if round < (max_round - F::from_canonical_usize(1)) {
                    prod_row.within_round_limit = true;
                    let start = calculate_3d_ext_idx(
                        prod_specs_inner_inner_len,
                        prod_specs_inner_len,
                        i,
                        round,
                        F::from_canonical_usize(0),
                    );
                    prod_row.data_ptr = start;

                    let (read_p, ps) =
                        memory.read::<{ EXT_DEG * 2 }>(data_address_space, prod_ptr + start);
                    let p1: [F; 4] = ps[0..EXT_DEG].try_into().expect("");
                    let p2: [F; 4] = ps[EXT_DEG..(EXT_DEG * 2)].try_into().expect("");

                    prod_row.read_data_records[1] = read_p;
                    prod_row.p1 = p1;
                    prod_row.p2 = p2;

                    let evals = if is_op_for_cur_sumcheck_round > F::ZERO {
                        FieldExtension::multiply(p1, p2)
                    } else {
                        FieldExtension::add(
                            FieldExtension::multiply(p1, c1),
                            FieldExtension::multiply(p2, c2),
                        )
                    };
                    prod_row.p_evals = evals;

                    let (write_slice_eval_1, _) = memory.write::<EXT_DEG>(
                        data_address_space,
                        r_ptr + (F::ONE + i) * F::from_canonical_usize(EXT_DEG),
                        evals,
                    );
                    prod_row.write_data_records[0] = write_slice_eval_1;

                    let is_op_for_next_sumcheck_round = F::ONE - is_op_for_cur_sumcheck_round;
                    let acc_eval = FieldExtension::multiply(alpha_acc, evals);
                    prod_row.acc_eval = acc_eval;

                    if (round + is_op_for_next_sumcheck_round)
                        < (max_round - F::from_canonical_usize(1))
                    {
                        eval_acc = FieldExtension::add(eval_acc, acc_eval);
                        prod_row.should_acc = true;
                        prod_row.eval_acc = eval_acc.clone();
                    }

                    curr_timestamp += 2;
                }

                alpha_acc = FieldExtension::multiply(alpha_acc, alpha);

                i = i + F::ONE;
                i_usize += 1;
                observation_records.push(prod_row);
                self.height += 1;
            }

            let mut i = F::ZERO;
            let mut i_usize = 0usize;
            while i < num_logup_spec {
                let mut logup_row: SumcheckEvalRecord<F> = SumcheckEvalRecord {
                    from_state,
                    instruction: instruction.clone(),
                    row_type: 2,
                    continuation: true,
                    curr_timestamp_increment: curr_timestamp,
                    register_ptrs,
                    ctx,
                    challenges,
                    alpha,
                    logup_spec_n: i_usize,
                    ..Default::default()
                };
                logup_row.alpha1 = alpha_acc;

                let (read_max_round, max_round) = memory.read_cell(
                    data_address_space,
                    ctx_pointer + F::from_canonical_usize(CONTEXT_ARR_BASE_LEN) + num_prod_spec + i,
                );
                logup_row.max_round = max_round;
                logup_row.read_data_records[0] = read_max_round;
                curr_timestamp += 1;

                if round < (max_round - F::from_canonical_usize(1)) {
                    logup_row.within_round_limit = true;
                    let start = calculate_3d_ext_idx(
                        logup_specs_inner_inner_len,
                        logup_specs_inner_len,
                        i,
                        round,
                        F::from_canonical_usize(0),
                    );
                    logup_row.data_ptr = start;

                    let (read_pqs, pqs) =
                        memory.read::<{ EXT_DEG * 4 }>(data_address_space, logup_ptr + start);
                    let p1: [F; 4] = pqs[0..EXT_DEG].try_into().expect("");
                    let p2: [F; 4] = pqs[EXT_DEG..(EXT_DEG * 2)].try_into().expect("");
                    let q1: [F; 4] = pqs[(EXT_DEG * 2)..(EXT_DEG * 3)].try_into().expect("");
                    let q2: [F; 4] = pqs[(EXT_DEG * 3)..(EXT_DEG * 4)].try_into().expect("");

                    logup_row.read_data_records[1] = read_pqs;
                    logup_row.p1 = p1;
                    logup_row.p2 = p2;
                    logup_row.q1 = q1;
                    logup_row.q2 = q2;

                    let p_evals = if is_op_for_cur_sumcheck_round > F::ZERO {
                        FieldExtension::add(
                            FieldExtension::multiply(p1, q2),
                            FieldExtension::multiply(p2, q1),
                        )
                    } else {
                        FieldExtension::add(
                            FieldExtension::multiply(p1, c1),
                            FieldExtension::multiply(p2, c2),
                        )
                    };

                    let q_evals = if is_op_for_cur_sumcheck_round > F::ZERO {
                        FieldExtension::multiply(q1, q2)
                    } else {
                        FieldExtension::add(
                            FieldExtension::multiply(q1, c1),
                            FieldExtension::multiply(q2, c2),
                        )
                    };

                    logup_row.p_evals = p_evals;
                    logup_row.q_evals = q_evals;

                    let (write_slice_eval_1, _) = memory.write::<EXT_DEG>(
                        data_address_space,
                        r_ptr + (F::ONE + num_prod_spec + i) * F::from_canonical_usize(EXT_DEG),
                        p_evals,
                    );
                    let (write_slice_eval_2, _) = memory.write::<EXT_DEG>(
                        data_address_space,
                        r_ptr
                            + (F::ONE + num_prod_spec + num_logup_spec + i)
                                * F::from_canonical_usize(EXT_DEG),
                        q_evals,
                    );

                    logup_row.write_data_records[0] = write_slice_eval_1;
                    logup_row.write_data_records[1] = write_slice_eval_2;

                    let is_op_for_next_sumcheck_round = F::ONE - is_op_for_cur_sumcheck_round;
                    let alpha_denominator = FieldExtension::multiply(alpha_acc, alpha);
                    logup_row.alpha2 = alpha_denominator;

                    if (round + is_op_for_next_sumcheck_round)
                        < (max_round - F::from_canonical_usize(1))
                    {
                        let acc_eval = FieldExtension::add(
                            FieldExtension::multiply(alpha_acc, p_evals),
                            FieldExtension::multiply(alpha_denominator, q_evals),
                        );
                        logup_row.acc_eval = acc_eval;
                        eval_acc = FieldExtension::add(eval_acc, acc_eval);
                        logup_row.should_acc = true;
                        logup_row.eval_acc = eval_acc.clone();
                    }

                    curr_timestamp += 3;
                }

                alpha_acc =
                    FieldExtension::multiply(FieldExtension::multiply(alpha_acc, alpha), alpha);

                i = i + F::ONE;
                i_usize += 1;
                observation_records.push(logup_row);
                self.height += 1;
            }

            let (write_r, _) = memory.write::<EXT_DEG>(data_address_space, r_ptr, eval_acc);
            curr_timestamp += 1;
            observation_records[0].write_data_records[0] = write_r;

            for record in &mut observation_records {
                record.final_timestamp_increment = curr_timestamp;
                record.eval_acc = FieldExtension::subtract(eval_acc, record.eval_acc);
            }
            let last_idx = observation_records.len() - 1;
            observation_records[last_idx].continuation = false;

            self.record_set.extend(observation_records);
            */
        } else {
            unreachable!()
        }

        Ok(())
    }

    // GKR layered IOP for product and logup relations
    fn get_opcode_name(&self, opcode: usize) -> String {
        assert_eq!(opcode, SUMCHECK_LAYER_EVAL.global_opcode().as_usize());
        String::from("SUMCHECK_LAYER_EVAL")
    }
}

impl<F: PrimeField32> TraceFiller<F> for NativeSumcheckFiller {
    fn fill_trace_row(&self, mem_helper: &MemoryAuxColsFactory<F>, row_slice: &mut [F]) {
        todo!();
        /*
        let slice = &mut flat_trace[used_cells..used_cells + width];
        let cols: &mut NativeSumcheckCols<F> = slice.borrow_mut();
        cols.first_timestamp = F::from_canonical_u32(record.from_state.timestamp);
        cols.start_timestamp = F::from_canonical_usize(
            record.from_state.timestamp as usize + record.curr_timestamp_increment,
        );
        cols.last_timestamp = F::from_canonical_usize(
            record.from_state.timestamp as usize + record.final_timestamp_increment,
        );
        cols.register_ptrs = record.register_ptrs;
        cols.ctx = record.ctx;
        cols.prod_nested_len = record.ctx[4] * record.ctx[3];
        cols.logup_nested_len = record.ctx[6] * record.ctx[5];
        cols.challenges = record.challenges;
        cols.alpha = record.alpha;
        cols.max_round = record.max_round;
        cols.within_round_limit = if record.within_round_limit {
            F::ONE
        } else {
            F::ZERO
        };
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
            cols.prod_row_within_max_round = if record.within_round_limit {
                F::ONE
            } else {
                F::ZERO
            };
            cols.prod_in_round_evaluation = if record.within_round_limit {
                record.ctx[7]
            } else {
                F::ZERO
            };
            cols.prod_next_round_evaluation = if record.within_round_limit {
                F::ONE - record.ctx[7]
            } else {
                F::ZERO
            };
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
            cols.logup_row_within_max_round = if record.within_round_limit {
                F::ONE
            } else {
                F::ZERO
            };
            cols.logup_in_round_evaluation = if record.within_round_limit {
                record.ctx[7]
            } else {
                F::ZERO
            };
            cols.logup_next_round_evaluation = if record.within_round_limit {
                F::ONE - record.ctx[7]
            } else {
                F::ZERO
            };
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
        }
        */
    }
}
