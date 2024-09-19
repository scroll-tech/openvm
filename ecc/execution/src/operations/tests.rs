use ff::Field;
use halo2curves_axiom::bn256::{Fq, Fq12, Fq2, G2Affine};
use rand::thread_rng;

use super::{evaluate_line, fp12_square};

#[test]
fn test_fp12_square() {
    let mut rng = thread_rng();
    let rnd = Fq12::random(&mut rng);
    let sq = fp12_square::<Fq12>(rnd);
    let sq_native = rnd.square();
    assert_eq!(sq, sq_native);
}

#[test]
fn test_evaluate_line() {
    let mut rng = thread_rng();
    let line = [Fq2::random(&mut rng), Fq2::random(&mut rng)];
    let x_over_y = Fq::random(&mut rng);
    let y_inv = Fq::random(&mut rng);
    // let evaluated_line = evaluate_line::<Fq, Fq2>(line, x_over_y, y_inv);
}

#[test]
fn test_mul_013_by_013() {}

#[test]
fn test_mul_by_013() {}

#[test]
fn test_mul_by_01234() {}
