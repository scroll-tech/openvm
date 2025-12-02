use std::borrow::BorrowMut;

use openvm_circuit::{
    arch::{
        CustomBorrow, ExecutionError, MultiRowLayout, MultiRowMetadata, PreflightExecutor,
        RecordArena, SizedRecord, TraceFiller, VmChipWrapper, VmStateMut,
    },
    system::{
        memory::{online::TracingMemory, MemoryAuxColsFactory},
        native_adapter::util::{memory_read_native, tracing_write_native_inplace},
    },
};
use openvm_instructions::{
    instruction::Instruction, program::DEFAULT_PC_STEP, LocalOpcode, NATIVE_AS,
};
use openvm_native_compiler::SumcheckOpcode::SUMCHECK_LAYER_EVAL;
use openvm_stark_backend::p3_field::PrimeField32;

use crate::{
    field_extension::{FieldExtension, EXT_DEG},
    fri::elem_to_ext,
    mem_fill_helper,
    sumcheck::columns::{
        HeaderSpecificCols, LogupSpecificCols, NativeSumcheckCols, ProdSpecificCols,
    },
    tracing_read_native_helper,
};

pub(crate) const CONTEXT_ARR_BASE_LEN: usize = EXT_DEG * 2;
pub(crate) const CURRENT_LAYER_MODE: u32 = 1;
pub(crate) const NEXT_LAYER_MODE: u32 = 0;

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
        // each instruction uses, and can thus treat each NativeSumcheckCols<...>
        // as a single record.
        NativeSumcheckRecordLayout {
            metadata: NativeSumcheckMetadata { num_rows: 1 },
        }
    }
}

impl<F: PrimeField32> SizedRecord<NativeSumcheckRecordLayout> for NativeSumcheckRecordMut<'_, F> {
    fn size(layout: &NativeSumcheckRecordLayout) -> usize {
        layout.metadata.num_rows * size_of::<NativeSumcheckCols<F>>()
    }

    fn alignment(_layout: &NativeSumcheckRecordLayout) -> usize {
        align_of::<NativeSumcheckCols<F>>()
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
            a: r_evals_reg,
            b: ctx_reg,
            c: challenges_reg,
            d: data_address_space,
            e: register_address_space,
            f: prod_evals_reg,
            g: logup_evals_reg,
        } = instruction;

        // This opcode supports two modes of operation:
        // 1. calculate the expected evaluation of two types of sumchecks for the current round
        //   a. product sumcheck: v' = v[0] * v[1]
        //   b. logup sumcheck: p'= p[0] * q[1] + p[1] * q[0] and q'= q[0] * q[1].
        // 2. calculate the expected value of next layer:
        //   a. product sumcheck: v[r] = eq(0,r) * v[0] + eq(1,r) * v[1]
        //   b. logup sumcheck: p[r] = eq(0,r) * p[0] + eq(1,r) * p[1]
        //         and q[r] = eq(0,r) * q[0] + eq(1,r) * q[1]
        assert_eq!(op, SUMCHECK_LAYER_EVAL.global_opcode());
        assert_eq!(data_address_space.as_canonical_u32(), NATIVE_AS);
        assert_eq!(register_address_space.as_canonical_u32(), NATIVE_AS);

        let [ctx_ptr]: [F; 1] = memory_read_native(state.memory.data(), ctx_reg.as_canonical_u32());
        let ctx: [u32; 8] = memory_read_native(state.memory.data(), ctx_ptr.as_canonical_u32())
            .map(|x: F| x.as_canonical_u32());

        let [round, num_prod_spec, num_logup_spec, prod_specs_inner_len, prod_specs_inner_inner_len, logup_specs_inner_len, logup_specs_inner_inner_len, mode] =
            ctx;
        // allocate n rows
        let num_rows = (1 + num_prod_spec + num_logup_spec) as usize;
        let rows = state
            .ctx
            .alloc(MultiRowLayout::new(NativeSumcheckMetadata { num_rows }))
            .0;

        let mut cur_timestamp = state.memory.timestamp();
        // head row
        let head_row: &mut NativeSumcheckCols<F> = &mut rows[0];
        let head_specific: &mut HeaderSpecificCols<F> =
            head_row.specific[..HeaderSpecificCols::<F>::width()].borrow_mut();

        head_row.header_row = F::ONE;
        head_row.first_timestamp = F::from_canonical_u32(cur_timestamp);
        head_row.start_timestamp = F::from_canonical_u32(cur_timestamp);

        head_specific.pc = F::from_canonical_u32(*state.pc);

        head_specific.registers[0] = ctx_reg;
        head_specific.registers[1] = challenges_reg;
        head_specific.registers[2] = prod_evals_reg;
        head_specific.registers[3] = logup_evals_reg;
        head_specific.registers[4] = r_evals_reg;

        // read pointers
        let [ctx_ptr]: [F; 1] = tracing_read_native_helper(
            state.memory,
            ctx_reg.as_canonical_u32(),
            head_specific.read_records[0].as_mut(),
        );
        let [challenges_ptr]: [F; 1] = tracing_read_native_helper(
            state.memory,
            challenges_reg.as_canonical_u32(),
            head_specific.read_records[1].as_mut(),
        );
        let [prod_evals_ptr]: [F; 1] = tracing_read_native_helper(
            state.memory,
            prod_evals_reg.as_canonical_u32(),
            head_specific.read_records[2].as_mut(),
        );
        let [logup_evals_ptr]: [F; 1] = tracing_read_native_helper(
            state.memory,
            logup_evals_reg.as_canonical_u32(),
            head_specific.read_records[3].as_mut(),
        );
        let [r_evals_ptr]: [F; 1] = tracing_read_native_helper(
            state.memory,
            r_evals_reg.as_canonical_u32(),
            head_specific.read_records[4].as_mut(),
        );

        let ctx: [F; CONTEXT_ARR_BASE_LEN] = tracing_read_native_helper(
            state.memory,
            ctx_ptr.as_canonical_u32(),
            head_specific.read_records[5].as_mut(),
        );

        let challenges: [F; EXT_DEG * 4] = tracing_read_native_helper(
            state.memory,
            challenges_ptr.as_canonical_u32(),
            head_specific.read_records[6].as_mut(),
        );
        cur_timestamp += 7; // 5 register reads + ctx read + challenges read
        head_row.challenges.copy_from_slice(&challenges);

        // challenges = [alpha, c1=r, c2=1-r]
        let alpha: [F; 4] = challenges[0..EXT_DEG].try_into().unwrap();
        let c1: [F; 4] = challenges[EXT_DEG..(EXT_DEG * 2)].try_into().unwrap();
        let c2: [F; 4] = challenges[(EXT_DEG * 2)..(EXT_DEG * 3)].try_into().unwrap();

        let mut eval_acc = elem_to_ext(F::from_canonical_u32(0));
        let mut alpha_acc = elem_to_ext(F::from_canonical_u32(1));

        // all rows share same register values, ctx, challenges
        for row in rows.iter_mut() {
            // c1, c2 are same during the entire execution
            row.challenges[EXT_DEG..3 * EXT_DEG].copy_from_slice(&challenges[EXT_DEG..3 * EXT_DEG]);
            row.alpha = alpha;
            row.ctx = ctx;
            row.prod_nested_len =
                F::from_canonical_u32(prod_specs_inner_len * prod_specs_inner_inner_len);
            row.logup_nested_len =
                F::from_canonical_u32(logup_specs_inner_len * logup_specs_inner_inner_len);
            row.register_ptrs[0] = ctx_ptr;
            row.register_ptrs[1] = challenges_ptr;
            row.register_ptrs[2] = prod_evals_ptr;
            row.register_ptrs[3] = logup_evals_ptr;
            row.register_ptrs[4] = r_evals_ptr;
        }

        // product rows
        for (i, prod_row) in rows
            .iter_mut()
            .skip(1)
            .take(num_prod_spec as usize)
            .enumerate()
        {
            let prod_specific: &mut ProdSpecificCols<F> =
                prod_row.specific[..ProdSpecificCols::<F>::width()].borrow_mut();

            prod_row.prod_row = F::ONE;
            prod_row.prod_continued = if i < (num_prod_spec - 1) as usize {
                F::ONE
            } else {
                F::ZERO
            };
            prod_row.curr_prod_n = F::from_canonical_usize(i + 1); // curr_prod_n starts from 1
            prod_row.start_timestamp = F::from_canonical_u32(cur_timestamp);

            // read max_round
            let [max_round]: [F; 1] = tracing_read_native_helper(
                state.memory,
                ctx_ptr.as_canonical_u32() + (CONTEXT_ARR_BASE_LEN + i) as u32,
                prod_specific.read_records[0].as_mut(),
            );
            cur_timestamp += 1;

            prod_row.challenges[0..EXT_DEG].copy_from_slice(&alpha_acc);
            prod_row.max_round = max_round;

            let max_round = max_round.as_canonical_u32();
            // round starts from 0
            if round < max_round - 1 {
                prod_row.within_round_limit = F::ONE;
                let start = calculate_3d_ext_idx(
                    prod_specs_inner_inner_len,
                    prod_specs_inner_len,
                    i as u32,
                    round,
                    0,
                );
                prod_specific.data_ptr = F::from_canonical_u32(start);

                // read p1, p2
                let ps: [F; EXT_DEG * 2] = tracing_read_native_helper(
                    state.memory,
                    prod_evals_ptr.as_canonical_u32() + start,
                    prod_specific.read_records[1].as_mut(),
                );
                let p1: [F; EXT_DEG] = ps[0..EXT_DEG].try_into().unwrap();
                let p2: [F; EXT_DEG] = ps[EXT_DEG..(EXT_DEG * 2)].try_into().unwrap();

                prod_specific.p = ps;

                // compute expected eval
                let eval = match mode {
                    NEXT_LAYER_MODE => FieldExtension::add(
                        FieldExtension::multiply(p1, c1),
                        FieldExtension::multiply(p2, c2),
                    ),
                    CURRENT_LAYER_MODE => FieldExtension::multiply(p1, p2),
                    _ => unreachable!("mode should be {CURRENT_LAYER_MODE} or {NEXT_LAYER_MODE}"),
                };
                prod_specific.p_evals = eval;

                match mode {
                    NEXT_LAYER_MODE => {
                        prod_row.prod_next_round_evaluation = F::ONE;
                    }
                    CURRENT_LAYER_MODE => {
                        prod_row.prod_in_round_evaluation = F::ONE;
                    }
                    _ => unreachable!("mode should be {CURRENT_LAYER_MODE} or {NEXT_LAYER_MODE}"),
                }

                // write p eval
                tracing_write_native_inplace(
                    state.memory,
                    r_evals_ptr.as_canonical_u32() + (1 + i as u32) * (EXT_DEG as u32),
                    eval,
                    &mut prod_specific.write_record,
                );
                cur_timestamp += 2;

                let eval_rlc = FieldExtension::multiply(alpha_acc, eval);
                prod_specific.eval_rlc = eval_rlc;

                let to_next_round = if mode == NEXT_LAYER_MODE { 1 } else { 0 };
                if round + to_next_round < max_round - 1 {
                    eval_acc = FieldExtension::add(eval_acc, eval_rlc);
                    prod_row.should_acc = F::ONE;
                    prod_row.prod_acc = F::ONE;
                    prod_row.eval_acc = eval_acc;
                }
            }

            alpha_acc = FieldExtension::multiply(alpha_acc, alpha);
        }

        // logup rows
        for (i, logup_row) in rows.iter_mut().skip(1 + num_prod_spec as usize).enumerate() {
            let logup_specific: &mut LogupSpecificCols<F> =
                logup_row.specific[..LogupSpecificCols::<F>::width()].borrow_mut();

            logup_row.logup_row = F::ONE;
            logup_row.logup_continued = if i < (num_logup_spec - 1) as usize {
                F::ONE
            } else {
                F::ZERO
            };
            logup_row.curr_logup_n = F::from_canonical_usize(i + 1); // curr_logup_n starts from 1
            logup_row.start_timestamp = F::from_canonical_u32(cur_timestamp);

            let [max_round]: [F; 1] = tracing_read_native_helper(
                state.memory,
                ctx_ptr.as_canonical_u32() + num_prod_spec + (CONTEXT_ARR_BASE_LEN + i) as u32,
                logup_specific.read_records[0].as_mut(),
            );
            logup_row.max_round = max_round;
            cur_timestamp += 1;

            let alpha_numerator = alpha_acc;
            let alpha_denominator = FieldExtension::multiply(alpha_acc, alpha);
            logup_row.challenges[0..EXT_DEG].copy_from_slice(&alpha_acc);
            logup_row.challenges[3 * EXT_DEG..(4 * EXT_DEG)].copy_from_slice(&alpha_denominator);

            let max_round = max_round.as_canonical_u32();
            if round < max_round - 1 {
                logup_row.within_round_limit = F::ONE;
                let start = calculate_3d_ext_idx(
                    logup_specs_inner_inner_len,
                    logup_specs_inner_len,
                    i as u32,
                    round,
                    0,
                );
                logup_specific.data_ptr = F::from_canonical_u32(start);

                // read p1, p2, q1, q2
                let pqs: [F; EXT_DEG * 4] = tracing_read_native_helper(
                    state.memory,
                    logup_evals_ptr.as_canonical_u32() + start,
                    logup_specific.read_records[1].as_mut(),
                );
                let p1: [F; EXT_DEG] = pqs[0..EXT_DEG].try_into().unwrap();
                let p2: [F; EXT_DEG] = pqs[EXT_DEG..(EXT_DEG * 2)].try_into().unwrap();
                let q1: [F; EXT_DEG] = pqs[(EXT_DEG * 2)..(EXT_DEG * 3)].try_into().unwrap();
                let q2: [F; EXT_DEG] = pqs[(EXT_DEG * 3)..(EXT_DEG * 4)].try_into().unwrap();

                logup_specific.pq = pqs;

                // compute expected evals
                let p_eval = match mode {
                    NEXT_LAYER_MODE => FieldExtension::add(
                        FieldExtension::multiply(p1, c1),
                        FieldExtension::multiply(p2, c2),
                    ),
                    CURRENT_LAYER_MODE => FieldExtension::add(
                        FieldExtension::multiply(p1, q2),
                        FieldExtension::multiply(p2, q1),
                    ),
                    _ => unreachable!("mode should be {CURRENT_LAYER_MODE} or {NEXT_LAYER_MODE}"),
                };
                let q_eval = match mode {
                    NEXT_LAYER_MODE => FieldExtension::add(
                        FieldExtension::multiply(q1, c1),
                        FieldExtension::multiply(q2, c2),
                    ),
                    CURRENT_LAYER_MODE => FieldExtension::multiply(q1, q2),
                    _ => unreachable!("mode should be {CURRENT_LAYER_MODE} or {NEXT_LAYER_MODE}"),
                };

                match mode {
                    NEXT_LAYER_MODE => {
                        logup_row.logup_next_round_evaluation = F::ONE;
                    }
                    CURRENT_LAYER_MODE => {
                        logup_row.logup_in_round_evaluation = F::ONE;
                    }
                    _ => unreachable!("mode should be {CURRENT_LAYER_MODE} or {NEXT_LAYER_MODE}"),
                }

                logup_specific.p_evals = p_eval;
                logup_specific.q_evals = q_eval;

                // write p_eval
                tracing_write_native_inplace(
                    state.memory,
                    r_evals_ptr.as_canonical_u32()
                        + (1 + num_prod_spec + i as u32) * (EXT_DEG as u32),
                    p_eval,
                    &mut logup_specific.write_records[0],
                );
                // write q_eval
                tracing_write_native_inplace(
                    state.memory,
                    r_evals_ptr.as_canonical_u32()
                        + (1 + num_prod_spec + num_logup_spec + i as u32) * (EXT_DEG as u32),
                    q_eval,
                    &mut logup_specific.write_records[1],
                );
                cur_timestamp += 3; // 1 read, 2 writes

                let eval_rlc = FieldExtension::add(
                    FieldExtension::multiply(alpha_numerator, p_eval),
                    FieldExtension::multiply(alpha_denominator, q_eval),
                );
                logup_specific.eval_rlc = eval_rlc;
                let to_next_round = if mode == NEXT_LAYER_MODE { 1 } else { 0 };
                if round + to_next_round < max_round - 1 {
                    eval_acc = FieldExtension::add(eval_acc, eval_rlc);
                    logup_row.should_acc = F::ONE;
                    logup_row.logup_acc = F::ONE;
                    logup_row.eval_acc = eval_acc;
                }
            }

            alpha_acc = FieldExtension::multiply(alpha_denominator, alpha);
        }

        if let Some(last_row) = rows.last_mut() {
            last_row.is_end = F::ONE;
        }

        let head_row = &mut rows[0];
        head_row.last_timestamp = F::from_canonical_u32(cur_timestamp + 1);

        let head_specific: &mut HeaderSpecificCols<F> =
            head_row.specific[..HeaderSpecificCols::<F>::width()].borrow_mut();

        tracing_write_native_inplace(
            state.memory,
            r_evals_ptr.as_canonical_u32(),
            eval_acc,
            &mut head_specific.write_records,
        );

        for row in rows.iter_mut() {
            if row.header_row == F::ONE {
                row.eval_acc = eval_acc;
            } else if row.prod_row == F::ONE {
                let specific: &mut ProdSpecificCols<F> =
                    row.specific[..ProdSpecificCols::<F>::width()].borrow_mut();

                if row.should_acc == F::ONE {
                    eval_acc = FieldExtension::subtract(eval_acc, specific.eval_rlc);
                }
                row.eval_acc = eval_acc;
            } else if row.logup_row == F::ONE {
                let specific: &mut LogupSpecificCols<F> =
                    row.specific[..LogupSpecificCols::<F>::width()].borrow_mut();
                if row.should_acc == F::ONE {
                    eval_acc = FieldExtension::subtract(eval_acc, specific.eval_rlc);
                }
                row.eval_acc = eval_acc;
            }
        }
        assert_eq!(eval_acc, elem_to_ext(F::from_canonical_u32(0)),);

        *state.pc = state.pc.wrapping_add(DEFAULT_PC_STEP);
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
        let cols: &mut NativeSumcheckCols<F> = row_slice.borrow_mut();
        let start_timestamp = cols.start_timestamp.as_canonical_u32();
        let last_timestamp = cols.last_timestamp.as_canonical_u32();

        if cols.header_row == F::ONE {
            let header: &mut HeaderSpecificCols<F> =
                cols.specific[..HeaderSpecificCols::<F>::width()].borrow_mut();

            for i in 0..7usize {
                mem_fill_helper(
                    mem_helper,
                    start_timestamp + i as u32,
                    header.read_records[i].as_mut(),
                );
            }
            mem_fill_helper(
                mem_helper,
                last_timestamp - 1,
                header.write_records.as_mut(),
            );
        } else if cols.prod_row == F::ONE {
            let prod_row_specific: &mut ProdSpecificCols<F> =
                cols.specific[..ProdSpecificCols::<F>::width()].borrow_mut();

            // read max_round
            mem_fill_helper(
                mem_helper,
                start_timestamp,
                prod_row_specific.read_records[0].as_mut(),
            );
            if cols.within_round_limit == F::ONE {
                // read p1, p2
                mem_fill_helper(
                    mem_helper,
                    start_timestamp + 1,
                    prod_row_specific.read_records[1].as_mut(),
                );
                // write p_eval
                mem_fill_helper(
                    mem_helper,
                    start_timestamp + 2,
                    prod_row_specific.write_record.as_mut(),
                );
            }
        } else if cols.logup_row == F::ONE {
            let logup_row_specific: &mut LogupSpecificCols<F> =
                cols.specific[..LogupSpecificCols::<F>::width()].borrow_mut();

            // read max_round
            mem_fill_helper(
                mem_helper,
                start_timestamp,
                logup_row_specific.read_records[0].as_mut(),
            );
            if cols.within_round_limit == F::ONE {
                // read p1, p2, q1, q2
                mem_fill_helper(
                    mem_helper,
                    start_timestamp + 1,
                    logup_row_specific.read_records[1].as_mut(),
                );
                // write p_eval
                mem_fill_helper(
                    mem_helper,
                    start_timestamp + 2,
                    logup_row_specific.write_records[0].as_mut(),
                );
                // write q_eval
                mem_fill_helper(
                    mem_helper,
                    start_timestamp + 3,
                    logup_row_specific.write_records[1].as_mut(),
                );
            }
        }
    }

    fn fill_dummy_trace_row(&self, row_slice: &mut [F]) {
        let cols: &mut NativeSumcheckCols<F> = row_slice.borrow_mut();

        cols.is_end = F::ONE;
    }
}
