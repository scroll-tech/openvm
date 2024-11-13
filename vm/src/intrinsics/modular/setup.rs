use std::{cell::RefCell, rc::Rc, sync::Arc};

use ax_circuit_primitives::{
    var_range::{VariableRangeCheckerBus, VariableRangeCheckerChip},
    SubAir, TraceSubRowGenerator,
};
use ax_ecc_primitives::field_expression::{
    ExprBuilder, ExprBuilderConfig, FieldExpr, FieldExprCols,
};
use ax_stark_backend::{interaction::InteractionBuilder, rap::BaseAirWithPublicValues};
use axvm_instructions::{instruction::Instruction, Rv32ModularArithmeticOpcode};
use itertools::Itertools;
use num_bigint_dig::BigUint;
use p3_air::{AirBuilder, BaseAir};
use p3_field::{AbstractField, Field, PrimeField32};

use crate::{
    arch::{
        AdapterAirContext, AdapterRuntimeContext, DynAdapterInterface, DynArray,
        MinimalInstruction, Result, VmAdapterInterface, VmCoreAir, VmCoreChip,
    },
    utils::limbs_to_biguint,
};

#[derive(Clone)]
pub struct ModularSetupCoreAir<const NUM_LANES: usize, const LANE_SIZE: usize> {
    pub expr: FieldExpr,
    pub modulus_bytes: Vec<u32>,
    pub offset: usize,
}

impl<const NUM_LANES: usize, const LANE_SIZE: usize> ModularSetupCoreAir<NUM_LANES, LANE_SIZE> {
    pub fn new(
        config: ExprBuilderConfig,
        range_bus: VariableRangeCheckerBus,
        modulus: BigUint,
        offset: usize,
    ) -> Self {
        config.check_valid();
        let builder = ExprBuilder::new(config, range_bus.range_max_bits);
        let builder = Rc::new(RefCell::new(builder));
        let _ = ExprBuilder::new_input(builder.clone());
        let builder = builder.borrow().clone();
        let expr = FieldExpr::new(builder, range_bus);

        let modulus_bytes = modulus
            .to_bytes_le()
            .iter()
            .chain(std::iter::repeat(&0))
            .take(NUM_LANES)
            .map(|x| *x as u32)
            .collect_vec();

        Self {
            expr,
            modulus_bytes,
            offset,
        }
    }
}

impl<F: Field, const NUM_LANES: usize, const LANE_SIZE: usize> BaseAir<F>
    for ModularSetupCoreAir<NUM_LANES, LANE_SIZE>
{
    fn width(&self) -> usize {
        BaseAir::<F>::width(&self.expr)
    }
}

impl<F: Field, const NUM_LANES: usize, const LANE_SIZE: usize> BaseAirWithPublicValues<F>
    for ModularSetupCoreAir<NUM_LANES, LANE_SIZE>
{
}

impl<AB: InteractionBuilder, I, const NUM_LANES: usize, const LANE_SIZE: usize> VmCoreAir<AB, I>
    for ModularSetupCoreAir<NUM_LANES, LANE_SIZE>
where
    I: VmAdapterInterface<AB::Expr>,
    AdapterAirContext<AB::Expr, I>:
        From<AdapterAirContext<AB::Expr, DynAdapterInterface<AB::Expr>>>,
{
    fn eval(
        &self,
        builder: &mut AB,
        local: &[AB::Var],
        _from_pc: AB::Var,
    ) -> AdapterAirContext<AB::Expr, I> {
        assert_eq!(local.len(), BaseAir::<AB::F>::width(&self.expr));
        self.expr.eval(builder, local);

        let FieldExprCols {
            is_valid, inputs, ..
        } = self.expr.load_vars(local);

        let bytes = self
            .modulus_bytes
            .iter()
            .map(|x| AB::Expr::from_canonical_u32(*x))
            .collect_vec();

        let reads: Vec<AB::Expr> = inputs.concat().iter().map(|x| (*x).into()).collect();

        let instruction = MinimalInstruction {
            is_valid: is_valid.into(),
            opcode: AB::Expr::from_canonical_usize(Rv32ModularArithmeticOpcode::SETUP as usize)
                + AB::Expr::from_canonical_usize(self.offset),
        };

        assert_eq!(bytes.len(), reads.len());
        for (lhs, rhs) in bytes.iter().zip(reads.iter()) {
            builder
                .when(is_valid.into())
                .assert_eq::<AB::Expr, AB::Expr>(lhs.clone(), rhs.clone());
        }

        let ctx: AdapterAirContext<_, DynAdapterInterface<_>> = AdapterAirContext {
            to_pc: None,
            reads: reads.into(),
            writes: vec![].into(),
            instruction: instruction.into(),
        };
        ctx.into()
    }
}

/// Number of limbs and limb size are determined purely at runtime
pub struct ModularSetupCoreChip<const NUM_LANES: usize, const LANE_SIZE: usize> {
    pub air: ModularSetupCoreAir<NUM_LANES, LANE_SIZE>,
    pub range_checker: Arc<VariableRangeCheckerChip>,
}

impl<const NUM_LANES: usize, const LANE_SIZE: usize> ModularSetupCoreChip<NUM_LANES, LANE_SIZE> {
    pub fn new(
        config: ExprBuilderConfig,
        range_checker: Arc<VariableRangeCheckerChip>,
        modulus: BigUint,
        offset: usize,
    ) -> Self {
        let air = ModularSetupCoreAir::new(config, range_checker.bus(), modulus, offset);
        Self { air, range_checker }
    }
}

impl<F: PrimeField32, const NUM_LANES: usize, const LANE_SIZE: usize, I> VmCoreChip<F, I>
    for ModularSetupCoreChip<NUM_LANES, LANE_SIZE>
where
    I: VmAdapterInterface<F>,
    I::Reads: Into<DynArray<F>>,
    AdapterRuntimeContext<F, I>: From<AdapterRuntimeContext<F, DynAdapterInterface<F>>>,
{
    type Record = BigUint;
    type Air = ModularSetupCoreAir<NUM_LANES, LANE_SIZE>;

    fn execute_instruction(
        &self,
        _instruction: &Instruction<F>,
        _from_pc: u32,
        reads: I::Reads,
    ) -> Result<(AdapterRuntimeContext<F, I>, Self::Record)> {
        let ctx = AdapterRuntimeContext::<_, DynAdapterInterface<_>>::without_pc(vec![]);

        Ok((
            ctx.into(),
            limbs_to_biguint(
                &reads
                    .into()
                    .0
                    .iter()
                    .map(|x| x.as_canonical_u32())
                    .collect_vec(),
                self.air.expr.canonical_limb_bits(),
            ),
        ))
    }

    fn get_opcode_name(&self, _opcode: usize) -> String {
        "ModularSetup".to_string()
    }

    fn generate_trace_row(&self, row_slice: &mut [F], record: Self::Record) {
        self.air
            .expr
            .generate_subrow((&self.range_checker, vec![record], vec![]), row_slice);
    }

    fn air(&self) -> &Self::Air {
        &self.air
    }
}
