use std::{
    borrow::{Borrow, BorrowMut},
    mem::size_of,
};

use openvm_circuit::{arch::*, system::memory::online::GuestMemory};
use openvm_circuit_primitives::AlignedBytesBorrow;
use openvm_instructions::{instruction::Instruction, program::DEFAULT_PC_STEP, NATIVE_AS};
use openvm_stark_backend::p3_field::PrimeField32;

use crate::{
    field_extension::{FieldExtension, EXT_DEG},
    fri::elem_to_ext,
    sumcheck::chip::{
        calculate_3d_ext_idx, NativeSumcheckExecutor, CONTEXT_ARR_BASE_LEN, CURRENT_LAYER_MODE,
        NEXT_LAYER_MODE,
    },
};

#[derive(AlignedBytesBorrow, Clone)]
#[repr(C)]
struct NativeSumcheckPreCompute {
    r_evals_reg: u32,
    ctx_reg: u32,
    challenges_reg: u32,
    prod_evals_reg: u32,
    logup_evals_reg: u32,
}

impl NativeSumcheckExecutor {
    #[inline(always)]
    fn pre_compute_impl<F: PrimeField32>(
        &self,
        pc: u32,
        inst: &Instruction<F>,
        data: &mut NativeSumcheckPreCompute,
    ) -> Result<(), StaticProgramError> {
        let &Instruction {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            ..
        } = inst;

        let r_evals_reg = a.as_canonical_u32();
        let ctx_reg = b.as_canonical_u32();
        let challenges_reg = c.as_canonical_u32();
        let prod_evals_reg = f.as_canonical_u32();
        let logup_evals_reg = g.as_canonical_u32();

        if d.as_canonical_u32() != NATIVE_AS {
            return Err(StaticProgramError::InvalidInstruction(pc));
        }
        if e.as_canonical_u32() != NATIVE_AS {
            return Err(StaticProgramError::InvalidInstruction(pc));
        }

        *data = NativeSumcheckPreCompute {
            r_evals_reg,
            ctx_reg,
            challenges_reg,
            prod_evals_reg,
            logup_evals_reg,
        };

        Ok(())
    }
}

impl<F> Executor<F> for NativeSumcheckExecutor
where
    F: PrimeField32,
{
    #[cfg(feature = "tco")]
    fn handler<Ctx>(
        &self,
        pc: u32,
        inst: &Instruction<F>,
        data: &mut [u8],
    ) -> Result<Handler<F, Ctx>, StaticProgramError>
    where
        Ctx: ExecutionCtxTrait,
    {
        let pre_compute: &mut NativeSumcheckPreCompute = data.borrow_mut();

        self.pre_compute_impl(pc, inst, pre_compute)?;

        let fn_ptr = execute_e1_handler;
        Ok(fn_ptr)
    }

    #[inline(always)]
    fn pre_compute_size(&self) -> usize {
        size_of::<NativeSumcheckPreCompute>()
    }

    #[cfg(not(feature = "tco"))]
    #[inline(always)]
    fn pre_compute<Ctx: ExecutionCtxTrait>(
        &self,
        pc: u32,
        inst: &Instruction<F>,
        data: &mut [u8],
    ) -> Result<ExecuteFunc<F, Ctx>, StaticProgramError> {
        let pre_compute: &mut NativeSumcheckPreCompute = data.borrow_mut();

        self.pre_compute_impl(pc, inst, pre_compute)?;

        let fn_ptr = execute_e1_impl;
        Ok(fn_ptr)
    }
}

impl<F> MeteredExecutor<F> for NativeSumcheckExecutor
where
    F: PrimeField32,
{
    #[inline(always)]
    fn metered_pre_compute_size(&self) -> usize {
        size_of::<E2PreCompute<NativeSumcheckPreCompute>>()
    }

    #[cfg(not(feature = "tco"))]
    #[inline(always)]
    fn metered_pre_compute<Ctx: MeteredExecutionCtxTrait>(
        &self,
        chip_idx: usize,
        pc: u32,
        inst: &Instruction<F>,
        data: &mut [u8],
    ) -> Result<ExecuteFunc<F, Ctx>, StaticProgramError> {
        let pre_compute: &mut E2PreCompute<NativeSumcheckPreCompute> = data.borrow_mut();
        pre_compute.chip_idx = chip_idx as u32;

        self.pre_compute_impl(pc, inst, &mut pre_compute.data)?;

        let fn_ptr = execute_e2_impl;
        Ok(fn_ptr)
    }

    #[cfg(feature = "tco")]
    fn metered_handler<Ctx: MeteredExecutionCtxTrait>(
        &self,
        chip_idx: usize,
        pc: u32,
        inst: &Instruction<F>,
        data: &mut [u8],
    ) -> Result<Handler<F, Ctx>, StaticProgramError> {
        let pre_compute: &mut E2PreCompute<NativeSumcheckPreCompute> = data.borrow_mut();
        pre_compute.chip_idx = chip_idx as u32;

        self.pre_compute_impl(pc, inst, &mut pre_compute.data)?;

        let fn_ptr = execute_e2_handler;
        Ok(fn_ptr)
    }
}

#[create_handler]
#[inline(always)]
unsafe fn execute_e1_impl<F: PrimeField32, CTX: ExecutionCtxTrait>(
    pre_compute: &[u8],
    instret: &mut u64,
    pc: &mut u32,
    _instret_end: u64,
    exec_state: &mut VmExecState<F, GuestMemory, CTX>,
) {
    let pre_compute: &NativeSumcheckPreCompute = pre_compute.borrow();
    execute_e12_impl(pre_compute, instret, pc, exec_state);
}

#[create_handler]
#[inline(always)]
unsafe fn execute_e2_impl<F: PrimeField32, CTX: MeteredExecutionCtxTrait>(
    pre_compute: &[u8],
    instret: &mut u64,
    pc: &mut u32,
    _arg: u64,
    exec_state: &mut VmExecState<F, GuestMemory, CTX>,
) {
    let pre_compute: &E2PreCompute<NativeSumcheckPreCompute> = pre_compute.borrow();
    let height = execute_e12_impl(&pre_compute.data, instret, pc, exec_state);
    exec_state
        .ctx
        .on_height_change(pre_compute.chip_idx as usize, height);
}

#[inline(always)]
unsafe fn execute_e12_impl<F: PrimeField32, CTX: ExecutionCtxTrait>(
    pre_compute: &NativeSumcheckPreCompute,
    instret: &mut u64,
    pc: &mut u32,
    exec_state: &mut VmExecState<F, GuestMemory, CTX>,
) -> u32 {
    let [r_evals_ptr]: [F; 1] = exec_state.vm_read(NATIVE_AS, pre_compute.r_evals_reg);
    let [ctx_ptr]: [F; 1] = exec_state.vm_read(NATIVE_AS, pre_compute.ctx_reg);
    let [challenges_ptr]: [F; 1] = exec_state.vm_read(NATIVE_AS, pre_compute.challenges_reg);
    let [prod_evals_ptr]: [F; 1] = exec_state.vm_read(NATIVE_AS, pre_compute.prod_evals_reg);
    let [logup_evals_ptr]: [F; 1] = exec_state.vm_read(NATIVE_AS, pre_compute.logup_evals_reg);

    let r_evals_ptr_u32 = r_evals_ptr.as_canonical_u32();
    let ctx_ptr_u32 = ctx_ptr.as_canonical_u32();
    let logup_evals_ptr = logup_evals_ptr.as_canonical_u32();
    let prod_evals_ptr = prod_evals_ptr.as_canonical_u32();

    let ctx: [u32; 8] = exec_state
        .vm_read(NATIVE_AS, ctx_ptr_u32)
        .map(|x: F| x.as_canonical_u32());
    let [round, num_prod_spec, num_logup_spec, prod_specs_inner_len, prod_specs_inner_inner_len, logup_specs_inner_len, logup_specs_inner_inner_len, mode] =
        ctx;
    let challenges: [F; EXT_DEG * 4] =
        exec_state.vm_read(NATIVE_AS, challenges_ptr.as_canonical_u32());
    let alpha: [F; EXT_DEG] = challenges[0..EXT_DEG].try_into().unwrap();
    let c1: [F; EXT_DEG] = challenges[EXT_DEG..EXT_DEG * 2].try_into().unwrap();
    let c2: [F; EXT_DEG] = challenges[EXT_DEG * 2..EXT_DEG * 3].try_into().unwrap();

    let mut height = 1;
    let mut alpha_acc = elem_to_ext(F::ONE);
    let mut eval_acc = elem_to_ext(F::ZERO);

    let prod_offset = ctx_ptr_u32 + CONTEXT_ARR_BASE_LEN as u32;
    for i in 0..num_prod_spec {
        let [max_round]: [u32; 1] = exec_state
            .vm_read(NATIVE_AS, prod_offset + i)
            .map(|x: F| x.as_canonical_u32());

        let start = calculate_3d_ext_idx(
            prod_specs_inner_inner_len,
            prod_specs_inner_len,
            i,
            round,
            0,
        );

        if round < max_round - 1 {
            let ps: [F; EXT_DEG * 2] = exec_state.vm_read(NATIVE_AS, prod_evals_ptr + start);
            let p1: [F; EXT_DEG] = ps[0..EXT_DEG].try_into().unwrap();
            let p2: [F; EXT_DEG] = ps[EXT_DEG..EXT_DEG * 2].try_into().unwrap();

            let eval = match mode {
                CURRENT_LAYER_MODE => FieldExtension::multiply(p1, p2),
                NEXT_LAYER_MODE => FieldExtension::add(
                    FieldExtension::multiply(p1, c1),
                    FieldExtension::multiply(p2, c2),
                ),
                _ => unreachable!("mode can only be {CURRENT_LAYER_MODE} or {NEXT_LAYER_MODE}"),
            };

            exec_state.vm_write(NATIVE_AS, r_evals_ptr_u32 + (1 + i) * EXT_DEG as u32, &eval);

            let to_next_round = if mode == NEXT_LAYER_MODE { 1 } else { 0 };
            if round + to_next_round < max_round - 1 {
                // update eval_acc
                eval_acc = FieldExtension::add(eval_acc, FieldExtension::multiply(alpha_acc, eval));
            }
        }

        // update alpha_acc
        alpha_acc = FieldExtension::multiply(alpha_acc, alpha);
        height += 1;
    }

    let logup_offset = ctx_ptr_u32 + CONTEXT_ARR_BASE_LEN as u32 + num_prod_spec;
    for i in 0..num_logup_spec {
        // read max_round
        let [max_round]: [u32; 1] = exec_state
            .vm_read(NATIVE_AS, logup_offset + i)
            .map(|x: F| x.as_canonical_u32());
        let start = calculate_3d_ext_idx(
            logup_specs_inner_inner_len,
            logup_specs_inner_len,
            i,
            round,
            0,
        );

        let alpha_denominator = FieldExtension::multiply(alpha_acc, alpha);
        let alpha_numerator = alpha_acc;

        if round < max_round - 1 {
            // read logup_evals
            let pqs: [F; EXT_DEG * 4] = exec_state.vm_read(NATIVE_AS, logup_evals_ptr + start);
            let p1: [F; EXT_DEG] = pqs[0..EXT_DEG].try_into().unwrap();
            let p2: [F; EXT_DEG] = pqs[EXT_DEG..EXT_DEG * 2].try_into().unwrap();
            let q1: [F; EXT_DEG] = pqs[EXT_DEG * 2..EXT_DEG * 3].try_into().unwrap();
            let q2: [F; EXT_DEG] = pqs[EXT_DEG * 3..EXT_DEG * 4].try_into().unwrap();

            // compute p_eval and q_eval
            let p_eval = match mode {
                CURRENT_LAYER_MODE => FieldExtension::add(
                    FieldExtension::multiply(p1, q2),
                    FieldExtension::multiply(p2, q1),
                ),
                NEXT_LAYER_MODE => FieldExtension::add(
                    FieldExtension::multiply(p1, c1),
                    FieldExtension::multiply(p2, c2),
                ),
                _ => unreachable!("mode can only be {CURRENT_LAYER_MODE} or {NEXT_LAYER_MODE}"),
            };
            let q_eval = match mode {
                CURRENT_LAYER_MODE => FieldExtension::multiply(q1, q2),
                NEXT_LAYER_MODE => FieldExtension::add(
                    FieldExtension::multiply(q1, c1),
                    FieldExtension::multiply(q2, c2),
                ),
                _ => unreachable!("mode can only be {CURRENT_LAYER_MODE} or {NEXT_LAYER_MODE}"),
            };

            // write eval to r_evals
            exec_state.vm_write(
                NATIVE_AS,
                r_evals_ptr_u32 + (1 + num_prod_spec + i) * EXT_DEG as u32,
                &p_eval,
            );
            exec_state.vm_write(
                NATIVE_AS,
                r_evals_ptr_u32 + (1 + num_prod_spec + num_logup_spec + i) * EXT_DEG as u32,
                &q_eval,
            );

            let eval_rlc = FieldExtension::add(
                FieldExtension::multiply(alpha_numerator, p_eval),
                FieldExtension::multiply(alpha_denominator, q_eval),
            );
            let to_next_round = if mode == NEXT_LAYER_MODE { 1 } else { 0 };
            if round + to_next_round < max_round - 1 {
                // update eval_acc
                eval_acc = FieldExtension::add(eval_acc, eval_rlc);
            }
        }

        // update alpha_acc
        alpha_acc = FieldExtension::multiply(alpha_denominator, alpha);
        height += 1;
    }

    *pc += DEFAULT_PC_STEP;
    *instret += 1;

    exec_state.vm_write(NATIVE_AS, r_evals_ptr_u32, &eval_acc);
    // return height delta
    height
}
