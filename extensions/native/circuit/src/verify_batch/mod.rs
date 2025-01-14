use std::{
    borrow::{Borrow, BorrowMut},
    sync::Arc,
};

use chip::{VerifyBatchChip, VerifyBatchRecord};
use columns::VerifyBatchCols;
use openvm_circuit::{
    arch::InstructionExecutor,
    system::memory::{MemoryAuxColsFactory, OfflineMemory},
};
use openvm_circuit_primitives::{
    is_zero::IsZeroSubAir, utils::next_power_of_two_or_zero, SubAir, TraceSubRowGenerator,
};
use openvm_instructions::instruction::Instruction;
use openvm_stark_backend::{
    config::{StarkGenericConfig, Val},
    interaction::InteractionBuilder,
    p3_air::{Air, AirBuilder, BaseAir},
    p3_field::{Field, FieldAlgebra, PrimeField32},
    p3_matrix::{dense::RowMajorMatrix, Matrix},
    p3_maybe_rayon::prelude::*,
    prover::types::AirProofInput,
    rap::{AnyRap, BaseAirWithPublicValues, PartitionedBaseAir},
    Chip, ChipUsageGetter,
};

use super::field_extension::{FieldExtension, EXT_DEG};
use crate::NATIVE_POSEIDON2_CHUNK_SIZE;

mod air;
mod chip;
mod columns;
#[cfg(test)]
mod tests;
mod trace;

const CHUNK: usize = NATIVE_POSEIDON2_CHUNK_SIZE;
