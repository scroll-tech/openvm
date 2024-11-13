use super::{Bn254, Fp, Fp12, Fp2};
use crate::pairing::{EvaluatedLine, LineMulDType};

impl LineMulDType<Fp, Fp2, Fp12> for Bn254 {}
//     fn mul_013_by_013(line_0: EvaluatedLine<Fp, Fp2>, line_1: EvaluatedLine<Fp, Fp2>) -> [Fp2; 5] {
//         #[cfg(not(target_os = "zkvm"))]
//         {
//             let b0 = line_0.b;
//             let c0 = line_0.c;
//             let b1 = line_1.b;
//             let c1 = line_1.c;

//             // where w⁶ = xi
//             // l0 * l1 = 1 + (b0 + b1)w + (b0b1)w² + (c0 + c1)w³ + (b0c1 + b1c0)w⁴ + (c0c1)w⁶
//             //         = (1 + c0c1 * xi) + (b0 + b1)w + (b0b1)w² + (c0 + c1)w³ + (b0c1 + b1c0)w⁴
//             let l0 = Fp2::ONE + &c0 * &c1 * &Self::XI;
//             let l1 = &b0 + &b1;
//             let l2 = &b0 * &b1;
//             let l3 = &c0 + &c1;
//             let l4 = &b0 * &c1 + &b1 * &c0;

//             [l0, l1, l2, l3, l4]
//         }
//         #[cfg(target_os = "zkvm")]
//         {
//             todo!()
//         }
//     }

//     fn mul_by_013(f: Fp12, l: EvaluatedLine<Fp, Fp2>) -> Fp12 {
//         #[cfg(not(target_os = "zkvm"))]
//         {
//             Self::mul_by_01234(f, [Fp2::ONE, l.b, Fp2::ZERO, l.c, Fp2::ZERO])
//         }
//         #[cfg(target_os = "zkvm")]
//         {
//             todo!()
//         }
//     }

//     fn mul_by_01234(f: Fp12, x: [Fp2; 5]) -> Fp12 {
//         #[cfg(not(target_os = "zkvm"))]
//         {
//             // we update the order of the coefficients to match the Fp12 coefficient ordering:
//             // Fp12 {
//             //   c0: Fp6 {
//             //     c0: x0,
//             //     c1: x2,
//             //     c2: x4,
//             //   },
//             //   c1: Fp6 {
//             //     c0: x1,
//             //     c1: x3,
//             //     c2: x5,
//             //   },
//             // }
//             let o0 = &x[0];
//             let o1 = &x[2];
//             let o2 = &x[4];
//             let o3 = &x[1];
//             let o4 = &x[3];

//             let xi = Self::XI;

//             // NOTE[yj]: Hand-calculated multiplication for Fp12 * 01234 ∈ Fp2; this is likely not the most efficient implementation
//             // c0 = cs0co0 + xi(cs1co2 + cs2co1 + cs4co4 + cs5co3)
//             // c1 = cs0co1 + cs1co0 + cs3co3 + xi(cs2co2 + cs5co4)
//             // c2 = cs0co2 + cs1co1 + cs2co0 + cs3co4 + cs4co3
//             // c3 = cs0co3 + cs3co0 + xi(cs2co4 + cs4co2 + cs5co1)
//             // c4 = cs0co4 + cs1co3 + cs3co1 + cs4co0 + xi(cs5co2)
//             // c5 = cs1co4 + cs2co3 + cs3co2 + cs4co1 + cs5co0
//             let c0 =
//                 &f.c[0] * o0 + xi * (&f.c[1] * o2 + &f.c[2] * o1 + &f.c[4] * o4 + &f.c[5] * o3);
//             let c1 =
//                 &f.c[0] * o1 + &f.c[1] * o0 + &f.c[3] * o3 + xi * (&f.c[2] * o2 + &f.c[5] * o4);
//             let c2 = &f.c[0] * o2 + &f.c[1] * o1 + &f.c[2] * o0 + &f.c[3] * o4 + &f.c[4] * o3;
//             let c3 =
//                 &f.c[0] * o3 + &f.c[3] * o0 + xi * (&f.c[2] * o4 + &f.c[4] * o2 + &f.c[5] * o1);
//             let c4 =
//                 &f.c[0] * o4 + &f.c[1] * o3 + &f.c[3] * o1 + &f.c[4] * o0 + xi * (&f.c[5] * o2);
//             let c5 = &f.c[1] * o4 + &f.c[2] * o3 + &f.c[3] * o2 + &f.c[4] * o1 + &f.c[5] * o0;

//             Fp12::new([c0, c1, c2, c3, c4, c5, Fp2::ZERO])
//         }
//         #[cfg(target_os = "zkvm")]
//         {
//             todo!()
//         }
//     }
// }
