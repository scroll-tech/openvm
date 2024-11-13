use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    marker::PhantomData,
};

use ax_circuit_derive::AlignedBorrow;
use ax_stark_backend::interaction::InteractionBuilder;
use axvm_instructions::instruction::Instruction;
use p3_air::BaseAir;
use p3_field::{AbstractField, Field, PrimeField32};

use super::RV32_REGISTER_NUM_LIMBS;
use crate::{
    arch::{
        AdapterAirContext, AdapterRuntimeContext, BasicAdapterInterface, ExecutionBridge,
        ExecutionBus, ExecutionState, ImmInstruction, Result, VmAdapterAir, VmAdapterChip,
        VmAdapterInterface,
    },
    system::{
        memory::{
            offline_checker::{MemoryBridge, MemoryReadAuxCols},
            MemoryAddress, MemoryAuxColsFactory, MemoryController, MemoryControllerRef,
            MemoryReadRecord,
        },
        program::ProgramBus,
    },
};

/// This adapter doesn't write anything, and reads from [a:4]_d, where d == 1
#[derive(Debug)]
pub struct Rv32RdReadAdapterChip<F: Field> {
    pub air: Rv32RdReadAdapterAir,
    _marker: PhantomData<F>,
}

impl<F: PrimeField32> Rv32RdReadAdapterChip<F> {
    pub fn new(
        execution_bus: ExecutionBus,
        program_bus: ProgramBus,
        memory_controller: MemoryControllerRef<F>,
    ) -> Self {
        let memory_controller = RefCell::borrow(&memory_controller);
        let memory_bridge = memory_controller.memory_bridge();
        Self {
            air: Rv32RdReadAdapterAir {
                execution_bridge: ExecutionBridge::new(execution_bus, program_bus),
                memory_bridge,
            },
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Rv32RdReadReadRecord<F: Field> {
    pub rd: Option<MemoryReadRecord<F, RV32_REGISTER_NUM_LIMBS>>,
}

#[derive(Debug, Clone)]
pub struct Rv32RdReadWriteRecord {
    pub from_state: ExecutionState<u32>,
}

#[repr(C)]
#[derive(Debug, Clone, AlignedBorrow)]
pub struct Rv32RdReadAdapterCols<T> {
    pub from_state: ExecutionState<T>,
    pub rd_ptr: T,
    pub rd_aux_cols: MemoryReadAuxCols<T, RV32_REGISTER_NUM_LIMBS>,
}

#[derive(Clone, Copy, Debug, derive_new::new)]
pub struct Rv32RdReadAdapterAir {
    pub(super) memory_bridge: MemoryBridge,
    pub(super) execution_bridge: ExecutionBridge,
}

impl<F: Field> BaseAir<F> for Rv32RdReadAdapterAir {
    fn width(&self) -> usize {
        Rv32RdReadAdapterCols::<F>::width()
    }
}

impl<AB: InteractionBuilder> VmAdapterAir<AB> for Rv32RdReadAdapterAir {
    type Interface =
        BasicAdapterInterface<AB::Expr, ImmInstruction<AB::Expr>, 1, 0, RV32_REGISTER_NUM_LIMBS, 0>;

    fn eval(
        &self,
        builder: &mut AB,
        local: &[AB::Var],
        ctx: AdapterAirContext<AB::Expr, Self::Interface>,
    ) {
        let local_cols: &Rv32RdReadAdapterCols<AB::Var> = (*local).borrow();
        let timestamp: AB::Var = local_cols.from_state.timestamp;
        let timestamp_delta = 1;
        self.memory_bridge
            .read(
                MemoryAddress::new(AB::Expr::ONE, local_cols.rd_ptr),
                ctx.reads[0].clone(),
                timestamp,
                &local_cols.rd_aux_cols,
            )
            .eval(builder, ctx.instruction.is_valid.clone());

        let to_pc = ctx
            .to_pc
            .unwrap_or(local_cols.from_state.pc + AB::F::from_canonical_u32(4));
        self.execution_bridge
            .execute(
                ctx.instruction.opcode,
                [
                    local_cols.rd_ptr.into(),
                    AB::Expr::ZERO,
                    ctx.instruction.immediate,
                    AB::Expr::ONE,
                    AB::Expr::ZERO,
                ],
                local_cols.from_state,
                ExecutionState {
                    pc: to_pc,
                    timestamp: timestamp + AB::F::from_canonical_usize(timestamp_delta),
                },
            )
            .eval(builder, ctx.instruction.is_valid);
    }

    fn get_from_pc(&self, local: &[AB::Var]) -> AB::Var {
        let cols: &Rv32RdReadAdapterCols<_> = local.borrow();
        cols.from_state.pc
    }
}

impl<F: PrimeField32> VmAdapterChip<F> for Rv32RdReadAdapterChip<F> {
    type ReadRecord = Rv32RdReadReadRecord<F>;
    type WriteRecord = Rv32RdReadWriteRecord;
    type Air = Rv32RdReadAdapterAir;
    type Interface = BasicAdapterInterface<F, ImmInstruction<F>, 1, 0, RV32_REGISTER_NUM_LIMBS, 0>;

    fn preprocess(
        &mut self,
        memory: &mut MemoryController<F>,
        instruction: &Instruction<F>,
    ) -> Result<(
        <Self::Interface as VmAdapterInterface<F>>::Reads,
        Self::ReadRecord,
    )> {
        let Instruction { a, d, .. } = *instruction;
        debug_assert_eq!(d.as_canonical_u32(), 1);
        let rd = memory.read(d, a);

        Ok(([rd.data], Self::ReadRecord { rd: Some(rd) }))
    }

    fn postprocess(
        &mut self,
        memory: &mut MemoryController<F>,
        _instruction: &Instruction<F>,
        from_state: ExecutionState<u32>,
        output: AdapterRuntimeContext<F, Self::Interface>,
        _read_record: &Self::ReadRecord,
    ) -> Result<(ExecutionState<u32>, Self::WriteRecord)> {
        Ok((
            ExecutionState {
                pc: output.to_pc.unwrap_or(from_state.pc + 4),
                timestamp: memory.timestamp(),
            },
            Self::WriteRecord { from_state },
        ))
    }

    fn generate_trace_row(
        &self,
        row_slice: &mut [F],
        read_record: Self::ReadRecord,
        write_record: Self::WriteRecord,
        aux_cols_factory: &MemoryAuxColsFactory<F>,
    ) {
        let adapter_cols: &mut Rv32RdReadAdapterCols<F> = row_slice.borrow_mut();
        adapter_cols.from_state = write_record.from_state.map(F::from_canonical_u32);
        let rd = read_record.rd.unwrap();
        adapter_cols.rd_ptr = rd.pointer;
        adapter_cols.rd_aux_cols = aux_cols_factory.make_read_aux_cols(rd);
    }

    fn air(&self) -> &Self::Air {
        &self.air
    }
}
