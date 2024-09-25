use halo2curves_axiom::{
    bn256::{Fq, Fq12, Fq2, Fq6, G1Affine, G2Affine},
    ff::Field,
    CurveAffine, CurveAffineExt,
};
use rand::{rngs::StdRng, thread_rng, SeedableRng};

use crate::{
    common::{field::FieldExtension, point::EcPoint},
    curves::bn254::{conv_013_to_fq12, conv_fp2_coeffs_to_fq12, BN254_XI},
    operations::{
        evaluate_line, fp12_square, mul_013_by_013, mul_by_01234, mul_by_013,
        point_to_line_function,
    },
};

#[test]
fn test_fp12_square() {
    let mut rng = StdRng::seed_from_u64(8);
    let rnd = Fq12::random(&mut rng);
    let sq = fp12_square::<Fq12>(rnd);
    let sq_native = rnd.square();
    assert_eq!(sq, sq_native);
}

#[test]
#[ignore]
fn test_evaluate_line() {
    // NOTE: There is probably not a simple way to test this
}

#[test]
fn test_mul_013_by_013() {
    // Generate random curve points
    let mut rng = StdRng::seed_from_u64(8);
    let rnd_pt_0 = G1Affine::random(&mut rng);
    let rnd_pt_1 = G1Affine::random(&mut rng);
    let ec_point_0 = EcPoint::<Fq> {
        x: rnd_pt_0.x,
        y: rnd_pt_0.y,
    };
    let ec_point_1 = EcPoint::<Fq> {
        x: rnd_pt_1.x,
        y: rnd_pt_1.y,
    };
    // Get line evaludated at rnd_pt_0 and rnd_pt_1
    let line_0 = point_to_line_function::<Fq, Fq2>(ec_point_0);
    let line_1 = point_to_line_function::<Fq, Fq2>(ec_point_1);

    // Multiply the two line functions
    let mul_013_by_013 = mul_013_by_013::<Fq, Fq2>(line_0, line_1, BN254_XI);
    let mul_013_by_013 = conv_fp2_coeffs_to_fq12(&mul_013_by_013);

    // Compare with the result of multiplying two Fp12 elements
    let fp12_0 = conv_013_to_fq12(line_0);
    let fp12_1 = conv_013_to_fq12(line_1);
    let mul_fp12 = fp12_0 * fp12_1;
    println!("mul_013_by_013: {:#?}", mul_013_by_013);
    println!("mul_fp12: {:#?}", mul_fp12);
    assert_eq!(mul_013_by_013, mul_fp12);
}

#[test]
fn test_mul_by_013() {
    let mut rng = StdRng::seed_from_u64(8);
    let f = Fq12::random(&mut rng);
    let rnd_pt = G1Affine::random(&mut rng);
    let ec_point = EcPoint::<Fq> {
        x: rnd_pt.x,
        y: rnd_pt.y,
    };
    let line = point_to_line_function::<Fq, Fq2>(ec_point);
    let evaluated_f = mul_by_013::<Fq, Fq2, Fq6, Fq12>(f, line);
    // WIP: validate result

    let y_inv = rnd_pt.y.invert().unwrap();
    let x_over_y = rnd_pt.x * y_inv;

    let evaluated_line = evaluate_line::<Fq, Fq2>(line, x_over_y, y_inv);
    let evalulated_f_cmp = mul_by_013::<Fq, Fq2, Fq6, Fq12>(f, evaluated_line);
    assert_eq!(evaluated_f, evalulated_f_cmp);
}

#[test]
fn test_mul_by_01234() {
    let mut rng = StdRng::seed_from_u64(8);
    let f = Fq12::random(&mut rng);
    let x = [
        Fq2::random(&mut rng),
        Fq2::random(&mut rng),
        Fq2::random(&mut rng),
        Fq2::random(&mut rng),
        Fq2::random(&mut rng),
    ];
    let evaluated_f = mul_by_01234::<Fq, Fq2, Fq6, Fq12>(f, x);
    // WIP: validate result
}
