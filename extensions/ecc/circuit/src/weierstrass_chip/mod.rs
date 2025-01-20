mod add_ne;
mod double;

use std::sync::Arc;

pub use add_ne::*;
pub use double::*;

#[cfg(test)]
mod tests;

use std::sync::Mutex;

use num_bigint::BigUint;
use openvm_circuit::{arch::VmChipWrapper, system::memory::OfflineMemory};
use openvm_circuit_derive::InstructionExecutor;
use openvm_circuit_primitives::var_range::SharedVariableRangeCheckerChip;
use openvm_circuit_primitives_derive::{BytesStateful, Chip, ChipUsageGetter};
use openvm_ecc_transpiler::Rv32WeierstrassOpcode;
use openvm_mod_circuit_builder::{ExprBuilderConfig, FieldExpressionCoreChip};
use openvm_rv32_adapters::Rv32VecHeapAdapterChip;
use openvm_stark_backend::p3_field::PrimeField32;

/// BLOCK_SIZE: how many cells do we read at a time, must be a power of 2.
/// BLOCKS: how many blocks do we need to represent one input or output
/// For example, for bls12_381, BLOCK_SIZE = 16, each element has 3 blocks and with two elements per input AffinePoint, BLOCKS = 6.
/// For secp256k1, BLOCK_SIZE = 32, BLOCKS = 2.
#[derive(Chip, ChipUsageGetter, InstructionExecutor, BytesStateful)]
pub struct EcAddNeChip<F: PrimeField32, const BLOCKS: usize, const BLOCK_SIZE: usize>(
    VmChipWrapper<
        F,
        Rv32VecHeapAdapterChip<F, 2, BLOCKS, BLOCKS, BLOCK_SIZE, BLOCK_SIZE>,
        FieldExpressionCoreChip,
    >,
);

impl<F: PrimeField32, const BLOCKS: usize, const BLOCK_SIZE: usize>
    EcAddNeChip<F, BLOCKS, BLOCK_SIZE>
{
    pub fn new(
        adapter: Rv32VecHeapAdapterChip<F, 2, BLOCKS, BLOCKS, BLOCK_SIZE, BLOCK_SIZE>,
        config: ExprBuilderConfig,
        offset: usize,
        range_checker: SharedVariableRangeCheckerChip,
        offline_memory: Arc<Mutex<OfflineMemory<F>>>,
    ) -> Self {
        let expr = ec_add_ne_expr(config, range_checker.bus());
        let core = FieldExpressionCoreChip::new(
            expr,
            offset,
            vec![
                Rv32WeierstrassOpcode::EC_ADD_NE as usize,
                Rv32WeierstrassOpcode::SETUP_EC_ADD_NE as usize,
            ],
            vec![],
            range_checker,
            "EcAddNe",
            false,
        );
        Self(VmChipWrapper::new(adapter, core, offline_memory))
    }
}

#[derive(Chip, ChipUsageGetter, InstructionExecutor, BytesStateful)]
pub struct EcDoubleChip<F: PrimeField32, const BLOCKS: usize, const BLOCK_SIZE: usize>(
    VmChipWrapper<
        F,
        Rv32VecHeapAdapterChip<F, 1, BLOCKS, BLOCKS, BLOCK_SIZE, BLOCK_SIZE>,
        EcDoubleCoreChip,
    >,
);

impl<F: PrimeField32, const BLOCKS: usize, const BLOCK_SIZE: usize>
    EcDoubleChip<F, BLOCKS, BLOCK_SIZE>
{
    pub fn new(
        adapter: Rv32VecHeapAdapterChip<F, 1, BLOCKS, BLOCKS, BLOCK_SIZE, BLOCK_SIZE>,
        range_checker: SharedVariableRangeCheckerChip,
        config: ExprBuilderConfig,
        offset: usize,
        a: BigUint,
        offline_memory: Arc<Mutex<OfflineMemory<F>>>,
    ) -> Self {
        let core = EcDoubleCoreChip::new(config, range_checker.clone(), a, offset);
        Self(VmChipWrapper::new(adapter, core, offline_memory))
    }
}
