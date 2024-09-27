use halo2curves_axiom::bn256::{Fq, Fq12, Fq2, G1Affine, G2Affine};
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    common::EcPoint,
    curves::{BN254_XI, GNARK_BN254_PBE_NAF},
    miller::multi_miller_loop,
};

#[test]
#[allow(non_snake_case)]
fn test_multi_miller_loop_bn254() {
    // Generate random G1 and G2 points
    let mut rng0 = StdRng::seed_from_u64(8);
    let rnd_pt0 = G1Affine::random(&mut rng0);
    let P = EcPoint {
        x: rnd_pt0.x,
        y: rnd_pt0.y,
    };
    let mut rng1 = StdRng::seed_from_u64(8 * 2);
    let rnd_pt1 = G2Affine::random(&mut rng1);
    let Q = EcPoint {
        x: rnd_pt1.x,
        y: rnd_pt1.y,
    };
    println!("{:#?}", P);
    println!("{:#?}", Q);

    // // halo2curves pseudo-binary encoding
    // let pbe = SIX_U_PLUS_2_NAF
    //     .iter()
    //     .map(|&x| x as i32)
    //     .collect::<Vec<i32>>();
    // let pbe = pbe.as_slice();
    // println!("{:?}", pbe);

    // Run the multi-miller loop
    let f =
        multi_miller_loop::<Fq, Fq2, Fq12>(&[P], &[Q], GNARK_BN254_PBE_NAF.as_slice(), BN254_XI);
    println!("{:#?}", f);
}
