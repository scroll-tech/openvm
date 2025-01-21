use std::{cell::RefCell, ops::Deref, rc::Rc};

use num_bigint::{BigInt, BigUint, Sign};
use num_traits::Zero;
use openvm_circuit_primitives::{
    bigint::{
        check_carry_mod_to_zero::{CheckCarryModToZeroCols, CheckCarryModToZeroSubAir},
        check_carry_to_zero::get_carry_max_abs_and_bits,
        utils::*,
        OverflowInt,
    },
    var_range::{VariableRangeCheckerBus, VariableRangeCheckerChip},
    SubAir, TraceSubRowGenerator,
};
use openvm_stark_backend::{
    interaction::InteractionBuilder,
    p3_air::{Air, AirBuilder, BaseAir},
    p3_field::{Field, FieldAlgebra, PrimeField64},
    p3_matrix::Matrix,
    rap::{BaseAirWithPublicValues, PartitionedBaseAir},
};

use super::{FieldVariable, SymbolicExpr};

#[derive(Clone)]
pub struct ExprBuilderConfig {
    pub modulus: BigUint,
    pub num_limbs: usize,
    pub limb_bits: usize,
}

impl ExprBuilderConfig {
    pub fn check_valid(&self) {
        assert!(self.modulus.bits() <= (self.num_limbs * self.limb_bits) as u64);
    }
}

#[derive(Clone)]
pub struct ExprBuilder {
    // The prime field.
    pub prime: BigUint,
    // Same value, but we need BigInt for computing the quotient.
    pub prime_bigint: BigInt,
    pub prime_limbs: Vec<usize>,

    pub num_input: usize,
    pub num_flags: usize,

    // This should be equal to number of constraints, but declare it to be explicit.
    pub num_variables: usize,

    pub constants: Vec<(BigUint, Vec<usize>)>, // value and limbs

    /// The number of bits in a canonical representation of a limb.
    pub limb_bits: usize,
    /// Number of limbs in canonical representation of the bigint field element.
    pub num_limbs: usize,
    // The max bits to range check.
    pub range_checker_bits: usize,

    // The number of limbs of the quotient for each constraint.
    pub q_limbs: Vec<usize>,
    // The number of limbs of the carries for each constraint.
    pub carry_limbs: Vec<usize>,

    // The constraints that should be evaluated to zero mod p (doesn't include - p * q part).
    pub constraints: Vec<SymbolicExpr>,

    // The equations to compute the newly introduced variables. For trace gen only.
    pub computes: Vec<SymbolicExpr>,

    pub output_indices: Vec<usize>,

    /// Whether the builder has been finalized. Only after finalize, we can do generate_subrow and eval etc.
    finalized: bool,

    // Setup opcode is a special op that verifies the modulus is correct.
    // There are some chips that don't need it because we hardcode the modulus. E.g. the pairing ones.
    // For those chips need setup, setup is derived: setup = is_valid - sum(all_flags)
    // Therefore when the chip only supports one opcode, user won't explicitly create a flag for it
    // and we will create a default flag for it on finalizing.
    needs_setup: bool,
}

impl ExprBuilder {
    pub fn new(config: ExprBuilderConfig, range_checker_bits: usize) -> Self {
        let prime_bigint = BigInt::from_biguint(Sign::Plus, config.modulus.clone());
        Self {
            prime: config.modulus.clone(),
            prime_bigint,
            prime_limbs: big_uint_to_limbs(&config.modulus, config.limb_bits),
            num_input: 0,
            num_flags: 0,
            limb_bits: config.limb_bits,
            num_limbs: config.num_limbs,
            range_checker_bits,
            num_variables: 0,
            constants: vec![],
            q_limbs: vec![],
            carry_limbs: vec![],
            constraints: vec![],
            computes: vec![],
            output_indices: vec![],
            finalized: false,
            needs_setup: false,
        }
    }

    pub fn is_finalized(&self) -> bool {
        self.finalized
    }

    pub fn finalize(&mut self, needs_setup: bool) {
        self.finalized = true;
        self.needs_setup = needs_setup;

        // We don't support multi-op chip that doesn't need setup right now.
        assert!(needs_setup || self.num_flags == 0);

        // setup the defalut flag if needed
        if needs_setup && self.num_flags == 0 {
            self.new_flag();
        }
    }

    pub fn new_input(builder: Rc<RefCell<ExprBuilder>>) -> FieldVariable {
        let mut borrowed = builder.borrow_mut();
        let num_limbs = borrowed.num_limbs;
        let limb_bits = borrowed.limb_bits;
        borrowed.num_input += 1;
        let (num_input, range_checker_bits) = (borrowed.num_input, borrowed.range_checker_bits);
        drop(borrowed);
        FieldVariable {
            expr: SymbolicExpr::Input(num_input - 1),
            builder: builder.clone(),
            limb_max_abs: (1 << limb_bits) - 1,
            max_overflow_bits: limb_bits,
            expr_limbs: num_limbs,
            range_checker_bits,
        }
    }

    pub fn new_flag(&mut self) -> usize {
        self.num_flags += 1;
        self.num_flags - 1
    }

    pub fn needs_setup(&self) -> bool {
        assert!(self.finalized); // Should only be used after finalize.
        self.needs_setup
    }

    // Below functions are used when adding variables and constraints manually, need to be careful.
    // Number of variables, constraints and computes should be consistent,
    // so there should be same number of calls to the new_var, add_constraint and add_compute.
    pub fn new_var(&mut self) -> (usize, SymbolicExpr) {
        self.num_variables += 1;
        // Allocate space for the new variable, to make sure they are corresponding to the same variable index.
        self.constraints.push(SymbolicExpr::Input(0));
        self.computes.push(SymbolicExpr::Input(0));
        self.q_limbs.push(0);
        self.carry_limbs.push(0);
        (
            self.num_variables - 1,
            SymbolicExpr::Var(self.num_variables - 1),
        )
    }

    /// Creates a new constant (compile-time known) FieldVariable from `value` where
    /// the big integer `value` is decomposed into `num_limbs` limbs of `limb_bits` bits,
    /// with `num_limbs, limb_bits` specified by the builder config.
    pub fn new_const(builder: Rc<RefCell<ExprBuilder>>, value: BigUint) -> FieldVariable {
        let mut borrowed = builder.borrow_mut();
        let index = borrowed.constants.len();
        let limb_bits = borrowed.limb_bits;
        let num_limbs = borrowed.num_limbs;
        let limbs = big_uint_to_num_limbs(&value, limb_bits, num_limbs);
        let range_checker_bits = borrowed.range_checker_bits;
        borrowed.constants.push((value.clone(), limbs));
        drop(borrowed);

        FieldVariable {
            expr: SymbolicExpr::Const(index, value, num_limbs),
            builder,
            limb_max_abs: (1 << limb_bits) - 1,
            max_overflow_bits: limb_bits,
            expr_limbs: num_limbs,
            range_checker_bits,
        }
    }

    pub fn set_constraint(&mut self, index: usize, constraint: SymbolicExpr) {
        let (q_limbs, carry_limbs) =
            constraint.constraint_limbs(&self.prime, self.limb_bits, self.num_limbs);
        self.constraints[index] = constraint;
        self.q_limbs[index] = q_limbs;
        self.carry_limbs[index] = carry_limbs;
    }

    pub fn set_compute(&mut self, index: usize, compute: SymbolicExpr) {
        self.computes[index] = compute;
    }
}

#[derive(Clone)]
pub struct FieldExpr {
    pub builder: ExprBuilder,

    pub check_carry_mod_to_zero: CheckCarryModToZeroSubAir,

    pub range_bus: VariableRangeCheckerBus,
}

impl FieldExpr {
    pub fn new(
        builder: ExprBuilder,
        range_bus: VariableRangeCheckerBus,
        needs_setup: bool,
    ) -> Self {
        let mut builder = builder;
        builder.finalize(needs_setup);
        let subair = CheckCarryModToZeroSubAir::new(
            builder.prime.clone(),
            builder.limb_bits,
            range_bus.index,
            range_bus.range_max_bits,
        );
        FieldExpr {
            builder,
            check_carry_mod_to_zero: subair,
            range_bus,
        }
    }
}

impl Deref for FieldExpr {
    type Target = ExprBuilder;

    fn deref(&self) -> &ExprBuilder {
        &self.builder
    }
}

impl<F: Field> BaseAirWithPublicValues<F> for FieldExpr {}
impl<F: Field> PartitionedBaseAir<F> for FieldExpr {}
impl<F: Field> BaseAir<F> for FieldExpr {
    fn width(&self) -> usize {
        assert!(self.builder.is_finalized());
        self.num_limbs * (self.builder.num_input + self.builder.num_variables)
            + self.builder.q_limbs.iter().sum::<usize>()
            + self.builder.carry_limbs.iter().sum::<usize>()
            + self.builder.num_flags
            + 1 // is_valid
    }
}

impl<AB: InteractionBuilder> Air<AB> for FieldExpr {
    fn eval(&self, builder: &mut AB) {
        let main = builder.main();
        let local = main.row_slice(0);
        SubAir::eval(self, builder, &local);
    }
}

impl<AB: InteractionBuilder> SubAir<AB> for FieldExpr {
    /// The sub-row slice owned by the expression builder.
    type AirContext<'a>
        = &'a [AB::Var]
    where
        AB: 'a,
        AB::Var: 'a,
        AB::Expr: 'a;

    fn eval<'a>(&'a self, builder: &'a mut AB, local: &'a [AB::Var])
    where
        AB::Var: 'a,
        AB::Expr: 'a,
    {
        assert!(self.builder.is_finalized());
        let FieldExprCols {
            is_valid,
            inputs,
            vars,
            q_limbs,
            carry_limbs,
            flags,
        } = self.load_vars(local);

        if self.builder.needs_setup() {
            let is_setup = flags.iter().fold(is_valid.into(), |acc, &x| acc - x);
            builder.assert_bool(is_setup.clone());
            // TODO[jpw]: currently we enforce at the program code level that:
            // - a valid program must call the correct setup opcodes to be correct
            // - it would be better if we can constraint this in the circuit,
            //   however this has the challenge that when the same chip is used
            //   across continuation segments,
            //   only the first segment will have setup called
            for i in 0..inputs[0].len().max(self.builder.prime_limbs.len()) {
                let lhs = if i < inputs[0].len() {
                    inputs[0][i].into()
                } else {
                    AB::Expr::ZERO
                };
                let rhs = if i < self.builder.prime_limbs.len() {
                    AB::Expr::from_canonical_usize(self.builder.prime_limbs[i])
                } else {
                    AB::Expr::ZERO
                };
                builder.when(is_setup.clone()).assert_eq(lhs, rhs);
            }
        }

        let inputs = load_overflow::<AB>(inputs, self.limb_bits);
        let vars = load_overflow::<AB>(vars, self.limb_bits);
        let constants: Vec<_> = self
            .constants
            .iter()
            .map(|(_, limbs)| {
                let limbs_expr: Vec<_> = limbs
                    .iter()
                    .map(|limb| AB::Expr::from_canonical_usize(*limb))
                    .collect();
                OverflowInt::from_canonical_unsigned_limbs(limbs_expr, self.limb_bits)
            })
            .collect();

        for flag in flags.iter() {
            builder.assert_bool(*flag);
        }
        for i in 0..self.constraints.len() {
            let expr = self.constraints[i]
                .evaluate_overflow_expr::<AB>(&inputs, &vars, &constants, &flags);
            self.check_carry_mod_to_zero.eval(
                builder,
                (
                    expr,
                    CheckCarryModToZeroCols {
                        carries: carry_limbs[i].clone(),
                        quotient: q_limbs[i].clone(),
                    },
                    is_valid.into(),
                ),
            );
        }

        for var in vars.iter() {
            for limb in var.limbs().iter() {
                range_check(
                    builder,
                    self.range_bus.index,
                    self.range_bus.range_max_bits,
                    self.limb_bits,
                    limb.clone(),
                    is_valid,
                );
            }
        }
    }
}

type Vecs<T> = Vec<Vec<T>>;

pub struct FieldExprCols<T> {
    pub is_valid: T,
    pub inputs: Vecs<T>,
    pub vars: Vecs<T>,
    pub q_limbs: Vecs<T>,
    pub carry_limbs: Vecs<T>,
    pub flags: Vec<T>,
}

impl<F: PrimeField64> TraceSubRowGenerator<F> for FieldExpr {
    type TraceContext<'a> = (&'a VariableRangeCheckerChip, Vec<BigUint>, Vec<bool>);
    type ColsMut<'a> = &'a mut [F];

    fn generate_subrow<'a>(
        &'a self,
        (range_checker, inputs, flags): (&'a VariableRangeCheckerChip, Vec<BigUint>, Vec<bool>),
        sub_row: &'a mut [F],
    ) {
        assert!(self.builder.is_finalized());
        assert_eq!(inputs.len(), self.num_input);
        // Remove this if this is no longer the case in the future.
        assert_eq!(self.num_variables, self.constraints.len());

        assert_eq!(flags.len(), self.builder.num_flags);

        let limb_bits = self.limb_bits;
        let mut vars = vec![BigUint::zero(); self.num_variables];

        // BigInt type is required for computing the quotient.
        let input_bigint = inputs
            .iter()
            .map(|x| BigInt::from_biguint(Sign::Plus, x.clone()))
            .collect::<Vec<BigInt>>();
        let mut vars_bigint = vec![BigInt::zero(); self.num_variables];

        // OverflowInt type is required for computing the carries.
        let input_overflow = inputs
            .iter()
            .map(|x| OverflowInt::<isize>::from_biguint(x, self.limb_bits, Some(self.num_limbs)))
            .collect::<Vec<_>>();
        let zero = OverflowInt::<isize>::from_canonical_unsigned_limbs(vec![0], limb_bits);
        let mut vars_overflow = vec![zero; self.num_variables];
        let prime_overflow =
            OverflowInt::<isize>::from_biguint(&self.prime, self.limb_bits, Some(self.num_limbs));

        let constants: Vec<_> = self
            .constants
            .iter()
            .map(|(_, limbs)| {
                let limbs_isize: Vec<_> = limbs.iter().map(|i| *i as isize).collect();
                OverflowInt::from_canonical_unsigned_limbs(limbs_isize, self.limb_bits)
            })
            .collect();

        let mut all_q = vec![];
        let mut all_carry = vec![];
        for i in 0..self.constraints.len() {
            let r = self.computes[i].compute(&inputs, &vars, &flags, &self.prime);
            vars[i] = r.clone();
            vars_bigint[i] = BigInt::from_biguint(Sign::Plus, r);
            vars_overflow[i] =
                OverflowInt::<isize>::from_biguint(&vars[i], self.limb_bits, Some(self.num_limbs));
        }
        // We need to have all variables computed first because, e.g. constraints[2] might need variables[3].
        for i in 0..self.constraints.len() {
            // expr = q * p
            let expr_bigint =
                self.constraints[i].evaluate_bigint(&input_bigint, &vars_bigint, &flags);
            let q = &expr_bigint / &self.prime_bigint;
            // If this is not true then the evaluated constraint is not divisible by p.
            debug_assert_eq!(expr_bigint, &q * &self.prime_bigint);
            let q_limbs = big_int_to_num_limbs(&q, limb_bits, self.q_limbs[i]);
            assert_eq!(q_limbs.len(), self.q_limbs[i]); // If this fails, the q_limbs estimate is wrong.
            for &q in q_limbs.iter() {
                range_checker.add_count((q + (1 << limb_bits)) as u32, limb_bits + 1);
            }
            let q_overflow = OverflowInt::from_canonical_signed_limbs(q_limbs.clone(), limb_bits);
            // compute carries of (expr - q * p)
            let expr = self.constraints[i].evaluate_overflow_isize(
                &input_overflow,
                &vars_overflow,
                &constants,
                &flags,
            );
            let expr = expr - q_overflow * prime_overflow.clone();
            let carries = expr.calculate_carries(limb_bits);
            assert_eq!(carries.len(), self.carry_limbs[i]); // If this fails, the carry limbs estimate is wrong.
            let max_overflow_bits = expr.max_overflow_bits();
            let (carry_min_abs, carry_bits) =
                get_carry_max_abs_and_bits(max_overflow_bits, limb_bits);
            for &carry in carries.iter() {
                range_checker.add_count((carry + carry_min_abs as isize) as u32, carry_bits);
            }
            all_q.push(vec_isize_to_f::<F>(q_limbs));
            all_carry.push(vec_isize_to_f::<F>(carries));
        }
        for var in vars_overflow.iter() {
            for limb in var.limbs().iter() {
                range_checker.add_count(*limb as u32, limb_bits);
            }
        }

        let input_limbs = input_overflow
            .iter()
            .map(|x| vec_isize_to_f::<F>(x.limbs().to_vec()))
            .collect::<Vec<_>>();
        let vars_limbs = vars_overflow
            .iter()
            .map(|x| vec_isize_to_f::<F>(x.limbs().to_vec()))
            .collect::<Vec<_>>();

        // TODO: avoid all these copies and directly allocate
        sub_row.copy_from_slice(
            &[
                vec![F::ONE],
                input_limbs.concat(),
                vars_limbs.concat(),
                all_q.concat(),
                all_carry.concat(),
                flags.iter().map(|x| F::from_bool(*x)).collect::<Vec<_>>(),
            ]
            .concat(),
        );
    }
}

impl FieldExpr {
    pub fn canonical_num_limbs(&self) -> usize {
        self.builder.num_limbs
    }

    pub fn canonical_limb_bits(&self) -> usize {
        self.builder.limb_bits
    }

    pub fn execute(&self, inputs: Vec<BigUint>, flags: Vec<bool>) -> Vec<BigUint> {
        assert!(self.builder.is_finalized());
        let mut vars = vec![BigUint::zero(); self.num_variables];
        for i in 0..self.constraints.len() {
            let r = self.computes[i].compute(&inputs, &vars, &flags, &self.prime);
            vars[i] = r.clone();
        }
        vars
    }

    pub fn execute_with_output(&self, inputs: Vec<BigUint>, flags: Vec<bool>) -> Vec<BigUint> {
        let vars = self.execute(inputs, flags);
        self.builder
            .output_indices
            .iter()
            .map(|i| vars[*i].clone())
            .collect()
    }

    pub fn load_vars<T: Clone>(&self, arr: &[T]) -> FieldExprCols<T> {
        assert!(self.builder.is_finalized());
        let is_valid = arr[0].clone();
        let mut idx = 1;
        let mut inputs = vec![];
        for _ in 0..self.num_input {
            inputs.push(arr[idx..idx + self.num_limbs].to_vec());
            idx += self.num_limbs;
        }
        let mut vars = vec![];
        for _ in 0..self.num_variables {
            vars.push(arr[idx..idx + self.num_limbs].to_vec());
            idx += self.num_limbs;
        }
        let mut q_limbs = vec![];
        for q in self.q_limbs.iter() {
            q_limbs.push(arr[idx..idx + q].to_vec());
            idx += q;
        }
        let mut carry_limbs = vec![];
        for c in self.carry_limbs.iter() {
            carry_limbs.push(arr[idx..idx + c].to_vec());
            idx += c;
        }
        let flags = arr[idx..idx + self.num_flags].to_vec();
        FieldExprCols {
            is_valid,
            inputs,
            vars,
            q_limbs,
            carry_limbs,
            flags,
        }
    }
}

fn load_overflow<AB: AirBuilder>(
    arr: Vecs<AB::Var>,
    limb_bits: usize,
) -> Vec<OverflowInt<AB::Expr>> {
    let mut result = vec![];
    for x in arr.into_iter() {
        let limbs: Vec<AB::Expr> = x.iter().map(|x| (*x).into()).collect();
        result.push(OverflowInt::<AB::Expr>::from_canonical_unsigned_limbs(
            limbs, limb_bits,
        ));
    }
    result
}
