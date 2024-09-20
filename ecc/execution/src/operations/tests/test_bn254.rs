use ff::Field;
use halo2curves_axiom::{
    bn256::{Fq, Fq12, Fq2, Fq6, G2Affine},
    CurveAffine,
};
use rand::thread_rng;

use crate::{
    common::field::FieldExtension,
    operations::{evaluate_line, fp12_square, mul_013_by_013, mul_by_01234, mul_by_013},
};

#[test]
fn test_fp12_square() {
    let mut rng = thread_rng();
    let rnd = Fq12::random(&mut rng);
    let sq = fp12_square::<Fq, Fq2, Fq6, Fq12>(rnd);
    let sq_native = rnd.square();
    assert_eq!(sq, sq_native);
}

#[test]
fn test_evaluate_line() {
    let mut rng = thread_rng();
    let line = [Fq2::random(&mut rng), Fq2::random(&mut rng)];
    let x_over_y = Fq::random(&mut rng);
    let y_inv = Fq::random(&mut rng);
    let res = evaluate_line::<Fq, Fq2>(line, x_over_y, y_inv);
    println!("res: {:?}", res);
    // let y = y_inv.invert().unwrap();
    // let x = x_over_y * y;
    let affine = G2Affine::from_xy(res[0], res[1]).unwrap();
    let on_curve: bool = affine.is_on_curve().into();
    println!("on_curve: {}", on_curve);
    assert!(on_curve);
}

#[test]
fn test_mul_013_by_013() {
    let mut rng = thread_rng();
    let line_0 = [Fq2::random(&mut rng), Fq2::random(&mut rng)];
    let line_1 = [Fq2::random(&mut rng), Fq2::random(&mut rng)];
    let evaluated_line = mul_013_by_013::<Fq, Fq2>(line_0, line_1, Fq2::lift(&Fq::ONE));
    // WIP: validate result
}

#[test]
fn test_mul_by_013() {
    let mut rng = thread_rng();
    let f = Fq12::random(&mut rng);
    let line = [Fq2::random(&mut rng), Fq2::random(&mut rng)];
    let evaluated_f = mul_by_013::<Fq, Fq2, Fq6, Fq12>(f, line);
    // WIP: validate result
}

#[test]
fn test_mul_by_01234() {
    let mut rng = thread_rng();
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
