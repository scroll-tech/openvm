use std::{cell::RefCell, rc::Rc};

use ax_circuit_primitives::{
    bigint::check_carry_mod_to_zero::CheckCarryModToZeroSubAir, var_range::VariableRangeCheckerBus,
};
use ax_ecc_primitives::{
    field_expression::{ExprBuilder, FieldExpr},
    field_extension::{mul_013_by_013, Fp12, Fp2},
};
use num_bigint_dig::BigUint;

use crate::intrinsics::ecc::FIELD_ELEMENT_BITS;

pub fn mul_013_by_013_expr(
    modulus: BigUint,
    num_limbs: usize,
    limb_bits: usize,
    range_bus: VariableRangeCheckerBus,
    xi: [isize; 2],
) -> FieldExpr {
    assert!(modulus.bits() <= num_limbs * limb_bits);
    let subair = CheckCarryModToZeroSubAir::new(
        modulus.clone(),
        limb_bits,
        range_bus.index,
        range_bus.range_max_bits,
        FIELD_ELEMENT_BITS,
    );
    let builder = ExprBuilder::new(modulus, limb_bits, num_limbs, range_bus.range_max_bits);
    let builder = Rc::new(RefCell::new(builder));

    let mut b0 = Fp2::new(builder.clone()); // x1
    let mut c0 = Fp2::new(builder.clone()); // x3
    let mut b1 = Fp2::new(builder.clone()); // y1
    let mut c1 = Fp2::new(builder.clone()); // y3

    let [mut l0, mut l1, mut l2, mut l3, mut l4] =
        mul_013_by_013(&mut b0, &mut c0, &mut b1, &mut c1, xi);
    l0.save_output();
    l1.save_output();
    l2.save_output();
    l3.save_output();
    l4.save_output();

    let builder = builder.borrow().clone();
    FieldExpr {
        builder,
        check_carry_mod_to_zero: subair,
        range_bus,
    }
}
