use std::{
    borrow::{Borrow, BorrowMut},
    sync::Arc,
};

use openvm_stark_backend::{
    Chip,
    ChipUsageGetter,
    config::{StarkGenericConfig, Val},
    interaction::InteractionBuilder,
    p3_air::{Air, AirBuilder, BaseAir},
    p3_field::{Field, FieldAlgebra, PrimeField32},
    p3_matrix::{dense::RowMajorMatrix, Matrix},
    p3_maybe_rayon::prelude::*,
    prover::types::AirProofInput, rap::{AnyRap, BaseAirWithPublicValues, PartitionedBaseAir},
};

use chip::{VerifyBatchChip, VerifyBatchRecord};
use columns::VerifyBatchCols;
use openvm_circuit::{
    arch::InstructionExecutor,
    system::memory::{MemoryAuxColsFactory, OfflineMemory},
};
use openvm_circuit_primitives::{
    is_zero::IsZeroSubAir, SubAir, TraceSubRowGenerator, utils::next_power_of_two_or_zero,
};
use openvm_instructions::instruction::Instruction;

use crate::NATIVE_POSEIDON2_CHUNK_SIZE;

use super::field_extension::{EXT_DEG, FieldExtension};

mod air;
mod chip;
mod columns;
#[cfg(test)]
mod tests;
mod trace;

const CHUNK: usize = NATIVE_POSEIDON2_CHUNK_SIZE;