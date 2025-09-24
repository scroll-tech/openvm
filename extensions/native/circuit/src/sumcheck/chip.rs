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
use crate::{fri::elem_to_ext, sumcheck::columns::{HeaderSpecificCols, LogupSpecificCols, NativeSumcheckCols, ProdSpecificCols}};
use openvm_native_compiler::{
    conversion::AS,
    SumcheckOpcode::SUMCHECK_LAYER_EVAL,
};
use crate::sumcheck::air::NativeSumcheckAir;
use crate::{
    field_extension::{FieldExtension, EXT_DEG},
    utils::const_max,
};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(bound = "F: Field")]
pub struct SumcheckEvalRecord<F: Field> {
    pub from_state: ExecutionState<u32>,
    pub instruction: Instruction<F>,
    pub row_type: usize,        // 0 - header; 1 - prod; 2 - logup
    pub curr_timestamp_increment: usize,
    pub final_timestamp_increment: usize,
    pub continuation: bool,

    pub register_ptrs: [F; 5],
    pub registers: [F; 5],
    pub ctx: [F; EXT_DEG * 2],
    pub challenges: [F; EXT_DEG * 4],
    pub read_data_records: [RecordId; 7],
    pub write_data_records: [RecordId; 2],

    pub max_round: F,
    pub within_round_limit: bool,
    pub should_acc: bool,
    pub prod_spec_n: usize,
    pub logup_spec_n: usize,
    pub alpha: [F; EXT_DEG],
    pub alpha1: [F; EXT_DEG],
    pub alpha2: [F; EXT_DEG],
    pub data_ptr: F,
    pub p1: [F; EXT_DEG],
    pub p2: [F; EXT_DEG],
    pub q1: [F; EXT_DEG],
    pub q2: [F; EXT_DEG],
    pub p_evals: [F; EXT_DEG],
    pub q_evals: [F; EXT_DEG],
    pub eval_acc: [F; EXT_DEG],
    pub acc_eval: [F; EXT_DEG],
}

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
    pub record_set: Vec<SumcheckEvalRecord<F>>,
    // _debug
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
            record_set: Default::default(),
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
            let mut observation_records: Vec<SumcheckEvalRecord<F>> = vec![];
            let mut curr_timestamp: usize = 0;

            // _debug
            println!("=> column width: {:?}", NativeSumcheckCols::<usize>::width());
            println!("=> header width: {:?}", HeaderSpecificCols::<usize>::width());
            println!("=> prod width: {:?}", ProdSpecificCols::<usize>::width());
            println!("=> logup width: {:?}", LogupSpecificCols::<usize>::width());

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

            let (ctx_read, ctx): (RecordId, [F; EXT_DEG * 2]) = memory.read::<{EXT_DEG * 2}>(data_address_space, ctx_pointer);
            let [
                round,
                num_prod_spec,
                num_logup_spec,
                prod_specs_inner_len,
                prod_specs_inner_inner_len,
                logup_specs_inner_len,
                logup_specs_inner_inner_len,
                in_round,
            ] = ctx;

            let (challenges_read, challenges): (RecordId, [F; EXT_DEG * 4]) = memory.read::<{EXT_DEG * 4}>(data_address_space, cs_pointer);
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
            let c2: [F; 4] = challenges[(EXT_DEG * 2)..(EXT_DEG * 3)].try_into().expect("");

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

                let (read_max_round, max_round) = memory.read_cell(data_address_space, ctx_pointer + F::from_canonical_usize(EXT_DEG * 2) + i);
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

                    let (read_p, ps) = memory.read::<{EXT_DEG * 2}>(data_address_space, prod_ptr + start);
                    let p1: [F; 4] = ps[0..EXT_DEG].try_into().expect("");
                    let p2: [F; 4] = ps[EXT_DEG..(EXT_DEG * 2)].try_into().expect("");

                    prod_row.read_data_records[1] = read_p;
                    prod_row.p1 = p1;
                    prod_row.p2 = p2;

                    let evals = if in_round > F::ZERO {
                        FieldExtension::multiply(p1, p2)
                    } else {
                        FieldExtension::add(
                            FieldExtension::multiply(p1, c1),
                            FieldExtension::multiply(p2, c2),
                        )
                    };
                    prod_row.p_evals = evals;
                    
                    let (write_slice_eval_1, _) = memory.write::<EXT_DEG>(data_address_space, r_ptr + (F::ONE + i) * F::from_canonical_usize(EXT_DEG), evals);
                    prod_row.write_data_records[0] = write_slice_eval_1;

                    let not_in_round = F::ONE - in_round;
                    if (round + not_in_round) < (max_round - F::from_canonical_usize(1)) {
                        let acc_eval = FieldExtension::multiply(alpha_acc, evals);
                        prod_row.acc_eval = acc_eval;
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

                let (read_max_round, max_round) = memory.read_cell(data_address_space, ctx_pointer + F::from_canonical_usize(EXT_DEG * 2) + num_prod_spec + i);
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

                    let (read_pqs, pqs) = memory.read::<{EXT_DEG * 4}>(data_address_space, logup_ptr + start);
                    let p1: [F; 4] = pqs[0..EXT_DEG].try_into().expect("");
                    let p2: [F; 4] = pqs[EXT_DEG..(EXT_DEG * 2)].try_into().expect("");
                    let q1: [F; 4] = pqs[(EXT_DEG * 2)..(EXT_DEG * 3)].try_into().expect("");
                    let q2: [F; 4] = pqs[(EXT_DEG * 3)..(EXT_DEG * 4)].try_into().expect("");

                    logup_row.read_data_records[1] = read_pqs;
                    logup_row.p1 = p1;
                    logup_row.p2 = p2;
                    logup_row.q1 = q1;
                    logup_row.q2 = q2;

                    let p_evals = if in_round > F::ZERO {
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
                    
                    let q_evals = if in_round > F::ZERO {
                        FieldExtension::multiply(q1, q2)
                    } else {
                        FieldExtension::add(
                            FieldExtension::multiply(q1, c1),
                            FieldExtension::multiply(q2, c2),
                        )
                    };

                    logup_row.p_evals = p_evals;
                    logup_row.q_evals = q_evals;

                    let (write_slice_eval_1, _) = memory.write::<EXT_DEG>(data_address_space, r_ptr + (F::ONE + num_prod_spec + i) * F::from_canonical_usize(EXT_DEG), p_evals);
                    let (write_slice_eval_2, _) = memory.write::<EXT_DEG>(data_address_space, r_ptr + (F::ONE + num_prod_spec + num_logup_spec + i) * F::from_canonical_usize(EXT_DEG), q_evals);
                    
                    logup_row.write_data_records[0] = write_slice_eval_1;
                    logup_row.write_data_records[1] = write_slice_eval_2;

                    let not_in_round = F::ONE - in_round;
                    if (round + not_in_round) < (max_round - F::from_canonical_usize(1)) {
                        let alpha_denominator = FieldExtension::multiply(alpha_acc, alpha);
                        let acc_eval = FieldExtension::add(
                            FieldExtension::multiply(alpha_acc, p_evals),
                            FieldExtension::multiply(alpha_denominator, q_evals),
                        );
                        logup_row.acc_eval = acc_eval;
                        eval_acc = FieldExtension::add(eval_acc, acc_eval);
                        logup_row.should_acc = true;
                        logup_row.alpha2 = alpha_denominator;
                        logup_row.eval_acc = eval_acc.clone();
                    }

                    curr_timestamp += 3;
                }

                alpha_acc = FieldExtension::multiply(FieldExtension::multiply(alpha_acc, alpha), alpha);

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
            println!("=> current_height: {:?}", self.height);
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