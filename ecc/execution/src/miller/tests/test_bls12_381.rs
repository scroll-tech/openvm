use halo2curves_axiom::bls12_381::{
    Fq, Fq12, Fq2, G1Affine, G2Affine, G2Prepared, MillerLoopResult,
};
use rand::{rngs::StdRng, SeedableRng};
use subtle::ConditionallySelectable;

use crate::{
    common::EcPoint,
    curves::bls12_381::{BLS12_381_XI, GNARK_BLS12_381_PBE},
    miller::{multi_miller_loop, multi_miller_loop_separate_double_plus_add},
};

#[test]
#[allow(non_snake_case)]
fn test_multi_miller_loop_bls12_381() {
    // Generate random G1 and G2 points
    let mut rng0 = StdRng::seed_from_u64(8);
    let P = G1Affine::random(&mut rng0);
    let mut rng1 = StdRng::seed_from_u64(8 * 2);
    let Q = G2Affine::random(&mut rng1);
    let either_identity = P.is_identity() | Q.is_identity();
    let P = G1Affine::conditional_select(&P, &G1Affine::generator(), either_identity);
    let Q = G2Affine::conditional_select(&Q, &G2Affine::generator(), either_identity);

    let P_ecpoint = EcPoint { x: P.x, y: P.y };
    let Q_ecpoint = EcPoint { x: Q.x, y: Q.y };

    // Compare against halo2curves implementation
    let g2_prepared = G2Prepared::from(Q);
    let compare_miller = halo2curves_axiom::bls12_381::multi_miller_loop(&[(&P, &g2_prepared)]);
    let compare_final = compare_miller.final_exponentiation();
    // let compare_final = halo2curves_axiom::bls12_381::pairing(&rnd_g1_affine, &rnd_g2_affine);

    // Run the multi-miller loop
    // let f = multi_miller_loop::<Fq, Fq2, Fq12>(
    let f = multi_miller_loop_separate_double_plus_add::<Fq, Fq2, Fq12>(
        &[P_ecpoint],
        &[Q_ecpoint],
        GNARK_BLS12_381_PBE.as_slice(),
        BLS12_381_XI,
    );
    println!("{:#?}", f);
    let wrapped_f = MillerLoopResult(f);
    let final_f = wrapped_f.final_exponentiation();

    // Run halo2curves final exponentiation on our multi_miller_loop output
    assert_eq!(final_f, compare_final);
}
