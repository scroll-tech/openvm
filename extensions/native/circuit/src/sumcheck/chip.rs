use std::sync::{Arc, Mutex};
use openvm_circuit::{
    arch::{
        ExecutionBridge, ExecutionError, ExecutionState, InstructionExecutor, Streams, SystemPort,
    },
    system::memory::{MemoryController, OfflineMemory, RecordId},
};
use openvm_instructions::{instruction::Instruction, program::DEFAULT_PC_STEP, LocalOpcode};
use openvm_stark_backend::{
    p3_field::{Field, PrimeField, PrimeField32},
    p3_maybe_rayon::prelude::{ParallelIterator, ParallelSlice},
};
use crate::fri::elem_to_ext;
use openvm_native_compiler::{
    conversion::AS,
    SumcheckOpcode::SUMCHECK_LAYER_EVAL,
};
use crate::sumcheck::air::NativeSumcheckAir;
use crate::{
    field_extension::{FieldExtension, EXT_DEG},
    utils::const_max,
};

fn calculate_3d_ext_idx<F: Field>(
    inner_inner_len: F,
    inner_len: F,
    outer_idx: F,
    inner_idx: F,
    inner_inner_idx: F,
) -> F {
    (inner_inner_len * inner_len * outer_idx + inner_inner_len * inner_idx + inner_inner_idx) * F::from_canonical_usize(EXT_DEG)
}

pub struct NativeSumcheckChip<F: Field> {
    pub height: usize,
    pub(super) air: NativeSumcheckAir<F>,
    pub(super) offline_memory: Arc<Mutex<OfflineMemory<F>>>,
    //     pub record_set: NativeSumcheckRecordSet<F>,
    //     pub(super) streams: Arc<Mutex<Streams<F>>>,
}

impl<F: PrimeField32> NativeSumcheckChip<F> {
    pub fn new(
        port: SystemPort,
        offline_memory: Arc<Mutex<OfflineMemory<F>>>,
    ) -> Self {
        let air = NativeSumcheckAir {
            execution_bridge: ExecutionBridge::new(port.execution_bus, port.program_bus),
            memory_bridge: port.memory_bridge,
            address_space: F::from_canonical_u32(AS::Native as u32),
        };

        Self { 
            height: 0, 
            air,
            offline_memory,
        }
    }
}

impl<F: PrimeField32> InstructionExecutor<F> for NativeSumcheckChip<F> {
    fn execute(
        &mut self,
        memory: &mut MemoryController<F>,
        instruction: &Instruction<F>,
        from_state: ExecutionState<u32>,
    ) -> Result<ExecutionState<u32>, ExecutionError> {
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

            let (ctx_read, ctx) = memory.read::<{EXT_DEG * 2}>(data_address_space, ctx_pointer);

            let [
                round,
                num_prod_spec,
                num_logup_spec,
                prod_specs_inner_len,
                prod_specs_inner_inner_len,
                logup_specs_inner_len,
                logup_specs_inner_inner_len,
                _,
            ] = ctx;

            let (alpha_read, alpha) = memory.read::<EXT_DEG>(data_address_space, cs_pointer);
            let (c1_read, c1) = memory.read::<EXT_DEG>(data_address_space, cs_pointer + F::from_canonical_usize(EXT_DEG * 1));
            let (c2_read, c2) = memory.read::<EXT_DEG>(data_address_space, cs_pointer + F::from_canonical_usize(EXT_DEG * 2));

            let mut eval_acc = elem_to_ext(F::from_canonical_u32(0));
            let mut alpha_acc = elem_to_ext(F::from_canonical_u32(1));

            let mut i = F::ZERO;
            while i < num_prod_spec {
                let (read_max_round, max_round) = memory.read_cell(data_address_space, ctx_pointer + F::from_canonical_usize(EXT_DEG * 2) + i);

                if round < (max_round - F::from_canonical_usize(1)) {
                    let start = calculate_3d_ext_idx(
                        prod_specs_inner_inner_len,
                        prod_specs_inner_len,
                        i,
                        round,
                        F::from_canonical_usize(0),
                    );
                    let (read_p1, p1) = memory.read::<EXT_DEG>(data_address_space, prod_ptr + start);
                    let (read_p2, p2) = memory.read::<EXT_DEG>(data_address_space, prod_ptr + start + F::from_canonical_usize(EXT_DEG));
                    let evals = FieldExtension::add(
                        FieldExtension::multiply(p1, c1),
                        FieldExtension::multiply(p2, c2),
                    );

                    let (write_slice_eval_1, _) = memory.write::<EXT_DEG>(data_address_space, r_ptr + (F::ONE + i) * F::from_canonical_usize(EXT_DEG), evals);

                    if (round + F::from_canonical_usize(1)) < (max_round - F::from_canonical_usize(1)) {
                        eval_acc = FieldExtension::add(eval_acc, FieldExtension::multiply(alpha_acc, evals));
                    }
                }

                alpha_acc = FieldExtension::multiply(alpha_acc, alpha);

                i = i + F::ONE;
            }

            let mut i = F::ZERO;
            while i < num_logup_spec {
                let (read_max_round, max_round) = memory.read_cell(data_address_space, ctx_pointer + num_prod_spec + F::from_canonical_usize(EXT_DEG * 2) + i);

                if round < (max_round - F::from_canonical_usize(1)) {
                    let start = calculate_3d_ext_idx(
                        logup_specs_inner_inner_len, 
                        logup_specs_inner_len, 
                        i, 
                        round,
                        F::from_canonical_usize(0),
                    );

                    let (read_p1, p1) = memory.read::<EXT_DEG>(data_address_space, logup_ptr + start);
                    let (read_p2, p2) = memory.read::<EXT_DEG>(data_address_space, logup_ptr + start + F::from_canonical_usize(EXT_DEG));
                    let (read_q1, q1) = memory.read::<EXT_DEG>(data_address_space, logup_ptr + start + F::from_canonical_usize(EXT_DEG * 2));
                    let (read_q2, q2) = memory.read::<EXT_DEG>(data_address_space, logup_ptr + start + F::from_canonical_usize(EXT_DEG * 3));

                    let p_evals = FieldExtension::add(
                        FieldExtension::multiply(p1, c1),
                        FieldExtension::multiply(p2, c2),
                    );
                    let q_evals = FieldExtension::add(
                        FieldExtension::multiply(q1, c1),
                        FieldExtension::multiply(q2, c2),
                    );

                    let (write_slice_eval_1, _) = memory.write::<EXT_DEG>(data_address_space, r_ptr + (F::ONE + num_prod_spec + i) * F::from_canonical_usize(EXT_DEG), p_evals);
                    let (write_slice_eval_2, _) = memory.write::<EXT_DEG>(data_address_space, r_ptr + (F::ONE + num_prod_spec + num_logup_spec + i) * F::from_canonical_usize(EXT_DEG), q_evals);

                    if (round + F::from_canonical_usize(1)) < (max_round - F::from_canonical_usize(1)) {
                        eval_acc = FieldExtension::add(eval_acc, FieldExtension::multiply(alpha_acc, p_evals));
                        let alpha_denominator = FieldExtension::multiply(alpha_acc, alpha);
                        eval_acc = FieldExtension::add(eval_acc, FieldExtension::multiply(alpha_denominator, q_evals));
                    }
                }

                alpha_acc = FieldExtension::multiply(FieldExtension::multiply(alpha_acc, alpha), alpha);

                i = i + F::ONE;
            }

            let (write_r, _) = memory.write::<EXT_DEG>(data_address_space, r_ptr, eval_acc);
        } else {
            unreachable!()
        }

        Ok(ExecutionState {
            pc: from_state.pc + DEFAULT_PC_STEP,
            timestamp: memory.timestamp(),
        })
    }


    fn get_opcode_name(&self, opcode: usize) -> String {
        if opcode == SUMCHECK_LAYER_EVAL.global_opcode().as_usize() {
            String::from("SUMCHECK_LAYER_EVAL")
        } else {
            unreachable!("unsupported opcode: {}", opcode)
        }
    }
}