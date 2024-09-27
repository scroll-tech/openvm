use halo2curves_axiom::bls12_381::{
    Fq, Fq12, Fq2, G1Affine, G2Affine, G2Prepared, MillerLoopResult,
};
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    common::EcPoint,
    curves::{BLS12_381_XI, GNARK_BLS12_381_PBE},
    miller::multi_miller_loop,
};

#[test]
#[allow(non_snake_case)]
fn test_multi_miller_loop_bls12_381() {
    // Generate random G1 and G2 points
    let mut rng0 = StdRng::seed_from_u64(8);
    let rnd_g1_affine = G1Affine::random(&mut rng0);
    let P = EcPoint {
        x: rnd_g1_affine.x,
        y: rnd_g1_affine.y,
    };
    let mut rng1 = StdRng::seed_from_u64(8 * 2);
    let rnd_g2_affine = G2Affine::random(&mut rng1);
    let Q = EcPoint {
        x: rnd_g2_affine.x,
        y: rnd_g2_affine.y,
    };
    println!("{:#?}", P);
    println!("{:#?}", Q);

    // Compare against halo2curves implementation
    let g2_prepared = G2Prepared::from(rnd_g2_affine);
    let compare_miller =
        halo2curves_axiom::bls12_381::multi_miller_loop(&[(&rnd_g1_affine, &g2_prepared)]);
    let compare_final = compare_miller.final_exponentiation();

    // Run the multi-miller loop
    let f = multi_miller_loop::<Fq, Fq2, Fq12>(
        &[P],
        &[Q],
        GNARK_BLS12_381_PBE.as_slice(),
        BLS12_381_XI,
    );
    println!("{:#?}", f);
    let wrapped_f = MillerLoopResult(f);
    let final_f = wrapped_f.final_exponentiation();

    // Run halo2curves final exponentiation on our multi_miller_loop output
    assert_eq!(final_f, compare_final);
}
