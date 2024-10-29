#[cfg(test)]
use ax_circuit_primitives::TraceSubRowGenerator;
#[cfg(test)]
use ax_ecc_execution::curves::bn254::{Fq, Fq2};
use ax_ecc_execution::{common::EcPoint, curves::bn254::point_to_013};
use ax_stark_sdk::{
    any_rap_arc_vec, config::baby_bear_blake3::BabyBearBlake3Engine, engine::StarkFriEngine,
};
use axvm_ecc_constants::BN254;
use halo2curves_axiom::bn256::G1Affine;
use num_bigint_dig::BigUint;
use p3_air::BaseAir;
use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use p3_matrix::dense::RowMajorMatrix;
use rand::{rngs::StdRng, SeedableRng};

use super::{
    super::{field_expression::*, test_utils::*},
    *,
};

pub fn bn254_fq_to_biguint(fq: Fq) -> BigUint {
    let bytes = fq.to_bytes();
    BigUint::from_bytes_le(&bytes)
}

pub fn bn254_fq2_to_biguint_vec(x: Fq2) -> Vec<BigUint> {
    vec![bn254_fq_to_biguint(x.c0), bn254_fq_to_biguint(x.c1)]
}

#[test]
fn test_mul_013_by_013() {
    let prime = BN254.MODULUS.clone();
    let (subair, range_checker, builder) = setup(&prime);

    let mut b0 = Fp2::new(builder.clone());
    let mut c0 = Fp2::new(builder.clone());
    let mut b1 = Fp2::new(builder.clone());
    let mut c1 = Fp2::new(builder.clone());
    let mut r =
        crate::field_extension::mul_013_by_013(&mut b0, &mut c0, &mut b1, &mut c1, BN254.XI);

    let builder = builder.borrow().clone();
    let air = FieldExpr {
        builder,
        check_carry_mod_to_zero: subair,
        range_bus: range_checker.bus(),
    };
    let width = BaseAir::<BabyBear>::width(&air);

    let mut rng0 = StdRng::seed_from_u64(8);
    let mut rng1 = StdRng::seed_from_u64(95);
    let rnd_pt_0 = G1Affine::random(&mut rng0);
    let rnd_pt_1 = G1Affine::random(&mut rng1);
    let ec_pt_0 = EcPoint::<Fq> {
        x: rnd_pt_0.x,
        y: rnd_pt_0.y,
    };
    let ec_pt_1 = EcPoint::<Fq> {
        x: rnd_pt_1.x,
        y: rnd_pt_1.y,
    };
    let line0 = point_to_013::<Fq, Fq2>(ec_pt_0);
    let line1 = point_to_013::<Fq, Fq2>(ec_pt_1);
    let r = ax_ecc_execution::curves::bn254::mul_013_by_013::<Fq, Fq2>(
        line0,
        line1,
        Fq2::new(Fq::from_raw([9, 0, 0, 0]), Fq::zero()),
    );
    let inputs = vec![
        bn254_fq2_to_biguint_vec(r[0]),
        bn254_fq2_to_biguint_vec(r[1]),
        bn254_fq2_to_biguint_vec(r[2]),
        bn254_fq2_to_biguint_vec(r[3]),
        bn254_fq2_to_biguint_vec(r[4]),
    ]
    .concat();

    let mut row = vec![BabyBear::zero(); width];
    air.generate_subrow((&range_checker, inputs, vec![]), &mut row);
    let FieldExprCols { vars, .. } = air.load_vars(&row);
    let trace = RowMajorMatrix::new(row, width);
    let range_trace = range_checker.generate_trace();
    assert_eq!(vars.len(), 10);
    let r0_c0 = evaluate_biguint(&vars[0], LIMB_BITS);
    let r0_c1 = evaluate_biguint(&vars[1], LIMB_BITS);
    let r1_c0 = evaluate_biguint(&vars[2], LIMB_BITS);
    let r1_c1 = evaluate_biguint(&vars[3], LIMB_BITS);
    let r2_c0 = evaluate_biguint(&vars[4], LIMB_BITS);
    let r2_c1 = evaluate_biguint(&vars[5], LIMB_BITS);
    let r3_c0 = evaluate_biguint(&vars[6], LIMB_BITS);
    let r3_c1 = evaluate_biguint(&vars[7], LIMB_BITS);
    let r4_c0 = evaluate_biguint(&vars[8], LIMB_BITS);
    let r4_c1 = evaluate_biguint(&vars[9], LIMB_BITS);
    let expected_r0_c0 = bn254_fq_to_biguint(r[0].c0);
    let expected_r0_c1 = bn254_fq_to_biguint(r[0].c1);
    let expected_r1_c0 = bn254_fq_to_biguint(r[1].c0);
    let expected_r1_c1 = bn254_fq_to_biguint(r[1].c1);
    let expected_r2_c0 = bn254_fq_to_biguint(r[2].c0);
    let expected_r2_c1 = bn254_fq_to_biguint(r[2].c1);
    let expected_r3_c0 = bn254_fq_to_biguint(r[3].c0);
    let expected_r3_c1 = bn254_fq_to_biguint(r[3].c1);
    let expected_r4_c0 = bn254_fq_to_biguint(r[4].c0);
    let expected_r4_c1 = bn254_fq_to_biguint(r[4].c1);
    assert_eq!(r0_c0, expected_r0_c0);
    assert_eq!(r0_c1, expected_r0_c1);
    assert_eq!(r1_c0, expected_r1_c0);
    assert_eq!(r1_c1, expected_r1_c1);
    assert_eq!(r2_c0, expected_r2_c0);
    assert_eq!(r2_c1, expected_r2_c1);
    assert_eq!(r3_c0, expected_r3_c0);
    assert_eq!(r3_c1, expected_r3_c1);
    assert_eq!(r4_c0, expected_r4_c0);
    assert_eq!(r4_c1, expected_r4_c1);

    BabyBearBlake3Engine::run_simple_test_no_pis_fast(
        any_rap_arc_vec![air, range_checker.air],
        vec![trace, range_trace],
    )
    .expect("Verification failed");
}
