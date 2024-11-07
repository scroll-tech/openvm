use axvm::intrinsics::{Fp2, Fp2Bls12381, BLS12_381_LIMBS};

pub trait LineMulBls12381 {
    fn mul_023_by_023(l0: [Fp2Bls12381; 2], l1: [Fp2Bls12381; 2]) -> [Fp2Bls12381; 6];

    fn mul_by_023(f: [Fp2Bls12381; 6], l: [Fp2Bls12381; 2]) -> [Fp2Bls12381; 6];

    fn mul_by_02345(f: [Fp2Bls12381; 6], x: [Fp2Bls12381; 6]) -> [Fp2Bls12381; 6];

    fn evaluate_line(
        l: [Fp2Bls12381; 2],
        x_over_y: [u8; BLS12_381_LIMBS],
        y_inv: [u8; BLS12_381_LIMBS],
    ) -> [Fp2Bls12381; 2];
}

impl LineMulBls12381 for Fp2Bls12381 {
    fn mul_023_by_023(l0: [Fp2Bls12381; 2], l1: [Fp2Bls12381; 2]) -> [Fp2Bls12381; 6] {
        #[cfg(not(target_os = "zkvm"))]
        {
            let b0 = &l0[0];
            let c0 = &l0[1];
            let b1 = &l1[0];
            let c1 = &l1[1];

            let xi = Fp2Bls12381::from_u32((1, 1));
            // where w⁶ = xi
            // l0 * l1 = c0c1 + (c0b1 + c1b0)w² + (c0 + c1)w³ + (b0b1)w⁴ + (b0 +b1)w⁵ + w⁶
            //         = (c0c1 + xi) + (c0b1 + c1b0)w² + (c0 + c1)w³ + (b0b1)w⁴ + (b0 + b1)w⁵
            let x0 = c0 * c1 + xi;
            let x2 = c0 * b1 + c1 * b0;
            let x3 = c0 + c1;
            let x4 = b0 * b1;
            let x5 = b0 + b1;

            [x0, Fp2Bls12381::ZERO, x2, x3, x4, x5]
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn mul_by_023(f: [Fp2Bls12381; 6], l: [Fp2Bls12381; 2]) -> [Fp2Bls12381; 6] {
        #[cfg(not(target_os = "zkvm"))]
        {
            let one = Fp2Bls12381::from_u32((1, 0));
            Self::mul_by_02345(
                f,
                [
                    l[1].clone(),
                    Fp2Bls12381::ZERO,
                    l[0].clone(),
                    one,
                    Fp2Bls12381::ZERO,
                    Fp2Bls12381::ZERO,
                ],
            )
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn mul_by_02345(f: [Fp2Bls12381; 6], x: [Fp2Bls12381; 6]) -> [Fp2Bls12381; 6] {
        #[cfg(not(target_os = "zkvm"))]
        {
            let xi = Fp2Bls12381::from_u32((1, 1));

            // we update the order of the coefficients to match the Fp12 coefficient ordering:
            // Fp12 {
            //   c0: Fp6 {
            //     c0: x0,
            //     c1: x2,
            //     c2: x4,
            //   },
            //   c1: Fp6 {
            //     c0: x1,
            //     c1: x3,
            //     c2: x5,
            //   },
            // }
            let o0 = &x[0];
            let o1 = &x[2];
            let o2 = &x[4];
            let o4 = &x[3];
            let o5 = &x[5];

            // NOTE[yj]: Hand-calculated multiplication for Fp12 * 02345 ∈ Fp2; this is likely not the most efficient implementation
            // c0 = cs0co0 + xi(cs1co2 + cs2co1 + cs3co5 + cs4co4)
            // c1 = cs0co1 + cs1co0 + xi(cs2co2 + cs4co5 + cs5co4)
            // c2 = cs0co2 + cs1co1 + cs2co0 + cs3co4 + xi(cs5co5)
            // c3 = cs3co0 + xi(cs1co5 + cs2co4 + cs4co2 + cs5co1)
            // c4 = cs0co4 + cs3co1 + cs4co0 + xi(cs2co5 + cs5co2)
            // c5 = cs0co5 + cs1co4 + cs3co2 + cs4co1 + cs5co0
            //   where cs*: self.c*
            let c0 = &f[0] * o0 + xi.clone() * (&f[1] * o2 + &f[2] * o1 + &f[3] * o5 + &f[4] * o4);
            let c1 = &f[0] * o1 + &f[1] * o0 + xi.clone() * (&f[2] * o2 + &f[4] * o5 + &f[5] * o4);
            let c2 = &f[0] * o2 + &f[1] * o1 + &f[2] * o0 + &f[3] * o4 + xi.clone() * (&f[5] * o5);
            let c3 = &f[3] * o0 + xi.clone() * (&f[1] * o5 + &f[2] * o4 + &f[4] * o2 + &f[5] * o1);
            let c4 = &f[0] * o4 + &f[3] * o1 + &f[4] * o0 + xi * (&f[2] * o5 + &f[5] * o2);
            let c5 = &f[0] * o5 + &f[1] * o4 + &f[3] * o2 + &f[4] * o1 + &f[5] * o0;

            [c0, c1, c2, c3, c4, c5]
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }

    fn evaluate_line(
        l: [Fp2Bls12381; 2],
        x_over_y: [u8; BLS12_381_LIMBS],
        y_inv: [u8; BLS12_381_LIMBS],
    ) -> [Fp2Bls12381; 2] {
        #[cfg(not(target_os = "zkvm"))]
        {
            let x_over_y_fp2 = Fp2Bls12381::from_bytes([x_over_y, x_over_y].into());
            let y_inv_fp2 = Fp2Bls12381::from_bytes([y_inv, y_inv].into());

            let r0 = &l[0] * &x_over_y_fp2;
            let r1 = &l[1] * &y_inv_fp2;

            [r0, r1]
        }
        #[cfg(target_os = "zkvm")]
        {
            todo!()
        }
    }
}
