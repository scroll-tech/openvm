use std::{
    borrow::{Borrow, BorrowMut},
    mem::size_of,
};

use openvm_circuit::{arch::*, system::memory::online::GuestMemory};
use openvm_circuit_primitives::AlignedBytesBorrow;
use openvm_instructions::{instruction::Instruction, program::DEFAULT_PC_STEP};
use openvm_native_compiler::conversion::AS;
use openvm_stark_backend::p3_field::PrimeField32;

use super::{elem_to_ext, FriReducedOpeningExecutor};
use crate::field_extension::{FieldExtension, EXT_DEG};

#[derive(AlignedBytesBorrow, Clone)]
#[repr(C)]
struct NativeSumcheckPreCompute {
    a_ptr_ptr: u32,
    b_ptr_ptr: u32,
    length_ptr: u32,
    alpha_ptr: u32,
    result_ptr: u32,
    hint_id_ptr: u32,
    is_init_ptr: u32,
}

impl NativeSumcheckPreCompute {
    #[inline(always)]
    fn pre_compute_impl<F: PrimeField32>(
        &self,
        _pc: u32,
        inst: &Instruction<F>,
        data: &mut FriReducedOpeningPreCompute,
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

        let a_ptr_ptr = a.as_canonical_u32();
        let b_ptr_ptr = b.as_canonical_u32();
        let length_ptr = c.as_canonical_u32();
        let alpha_ptr = d.as_canonical_u32();
        let result_ptr = e.as_canonical_u32();
        let hint_id_ptr = f.as_canonical_u32();
        let is_init_ptr = g.as_canonical_u32();

        *data = FriReducedOpeningPreCompute {
            a_ptr_ptr,
            b_ptr_ptr,
            length_ptr,
            alpha_ptr,
            result_ptr,
            hint_id_ptr,
            is_init_ptr,
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
    todo!()
}
