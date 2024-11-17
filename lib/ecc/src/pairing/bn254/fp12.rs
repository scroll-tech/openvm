use axvm_algebra::IntMod;
#[cfg(target_os = "zkvm")]
use {
    axvm_platform::constants::{Custom1Funct3, PairingBaseFunct7, CUSTOM_1},
    axvm_platform::custom_insn_r,
    core::mem::MaybeUninit,
};

use super::{Bn254Fp, Bn254Fp2};
use crate::field::{Field, FieldExtension, Fp12Mul, FrobeniusCoeffs, SexticExtField, Xi};

pub type Bn254Fp12 = SexticExtField<Bn254Fp2>;

impl FieldExtension for Bn254Fp12 {
    type BaseField = Bn254Fp2;
    type Coeffs = [Bn254Fp2; 6];
    type SelfRef<'a> = &'a Self;

    fn from_coeffs(coeffs: Self::Coeffs) -> Self {
        Self::new([
            coeffs[0].clone(),
            coeffs[2].clone(),
            coeffs[4].clone(),
            coeffs[1].clone(),
            coeffs[3].clone(),
            coeffs[5].clone(),
        ])
    }

    fn to_coeffs(self) -> Self::Coeffs {
        [
            self.c[0].clone(),
            self.c[1].clone(),
            self.c[2].clone(),
            self.c[3].clone(),
            self.c[4].clone(),
            self.c[5].clone(),
        ]
    }

    fn embed(base_elem: Self::BaseField) -> Self {
        Self::new([
            base_elem,
            <Self::BaseField as Field>::zero(),
            <Self::BaseField as Field>::zero(),
            <Self::BaseField as Field>::zero(),
            <Self::BaseField as Field>::zero(),
            <Self::BaseField as Field>::zero(),
        ])
    }

    fn conjugate(&self) -> Self {
        Self::new([
            self.c[0].clone(),
            self.c[1].clone(),
            self.c[2].clone(),
            -self.c[3].clone(),
            -self.c[4].clone(),
            -self.c[5].clone(),
        ])
    }

    fn frobenius_map(&self, power: usize) -> Self {
        let mut c0 = self.c[0].clone();
        let mut c1 = self.c[1].clone();
        let mut c2 = self.c[2].clone();
        let mut c3 = self.c[3].clone();
        let mut c4 = self.c[4].clone();
        let mut c5 = self.c[5].clone();

        if power % 2 != 0 {
            c0 = c0.conjugate();
            c1 = c1.conjugate();
            c2 = c2.conjugate();
            c3 = c3.conjugate();
            c4 = c4.conjugate();
            c5 = c5.conjugate();
        }

        c1 *= &Self::FROBENIUS_COEFFS[power][0];
        c2 *= &Self::FROBENIUS_COEFFS[power][1];
        c3 *= &Self::FROBENIUS_COEFFS[power][2];
        c4 *= &Self::FROBENIUS_COEFFS[power][3];
        c5 *= &Self::FROBENIUS_COEFFS[power][4];

        Self::new([c0, c1, c2, c3, c4, c5])
    }

    fn mul_base(&self, rhs: Self::BaseField) -> Self {
        Self::new([
            &self.c[0] * &rhs,
            &self.c[1] * &rhs,
            &self.c[2] * &rhs,
            &self.c[3] * &rhs,
            &self.c[4] * &rhs,
            &self.c[5] * &rhs,
        ])
    }
}

impl Fp12Mul for Bn254Fp12 {
    type Fp = Bn254Fp;
    type Fp2 = Bn254Fp2;
    const XI: Self::Fp2 = Bn254Fp2::XI;

    #[inline(always)]
    fn fp12_mul(&mut self, other: &Self) {
        #[cfg(not(target_os = "zkvm"))]
        {
            // The following multiplication is hand-derived for Fp12 * Fp12:
            // c0 = cs0co0 + xi(cs1co2 + cs2co1 + cs3co5 + cs4co4 + cs5co3)
            // c1 = cs0co1 + cs1co0 + cs3co3 + xi(cs2co2 + cs4co5 + cs5co4)
            // c2 = cs0co2 + cs1co1 + cs2co0 + cs3co4 + cs4co3 + xi(cs5co5)
            // c3 = cs0co3 + cs3co0 + xi(cs1co5 + cs2co4 + cs4co2 + cs5co1)
            // c4 = cs0co4 + cs1co3 + cs3co1 + cs4co0 + xi(cs2co5 + cs5co2)
            // c5 = cs0co5 + cs1co4 + cs2co3 + cs3co2 + cs4co1 + cs5co0
            //   where cs*: self.c*, co*: other.c*

            let (s0, s1, s2, s3, s4, s5) = (
                &self.c[0], &self.c[2], &self.c[4], &self.c[1], &self.c[3], &self.c[5],
            );
            let (o0, o1, o2, o3, o4, o5) = (
                &other.c[0],
                &other.c[2],
                &other.c[4],
                &other.c[1],
                &other.c[3],
                &other.c[5],
            );

            let c0 = s0 * o0 + Self::XI * &(s1 * o2 + s2 * o1 + s3 * o5 + s4 * o4 + s5 * o3);
            let c1 = s0 * o1 + s1 * o0 + s3 * o3 + Self::XI * &(s2 * o2 + s4 * o5 + s5 * o4);
            let c2 = s0 * o2 + s1 * o1 + s2 * o0 + s3 * o4 + s4 * o3 + Self::XI * &(s5 * o5);
            let c3 = s0 * o3 + s3 * o0 + Self::XI * &(s1 * o5 + s2 * o4 + s4 * o2 + s5 * o1);
            let c4 = s0 * o4 + s1 * o3 + s3 * o1 + s4 * o0 + Self::XI * &(s2 * o5 + s5 * o2);
            let c5 = s0 * o5 + s1 * o4 + s2 * o3 + s3 * o2 + s4 * o1 + s5 * o0;

            *self = Self::new([c0, c3, c1, c4, c2, c5]);
        }
        #[cfg(target_os = "zkvm")]
        {
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::Fp12Mul as usize,
                self as *mut Self,
                self as *const Self,
                other as *const Self
            )
        }
    }

    #[inline(always)]
    fn fp12_mul_refs(&self, other: &Self) -> Self {
        #[cfg(not(target_os = "zkvm"))]
        {
            let mut res = self.clone();
            res.fp12_mul(other);
            res
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<Self> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::Fp12Mul as usize,
                uninit.as_mut_ptr(),
                self as *const Self,
                other as *const Self
            );
            unsafe { uninit.assume_init() }
        }
    }
}

impl FrobeniusCoeffs for Bn254Fp12 {
    type Fp = Bn254Fp;
    type Fp2 = Bn254Fp2;
    const FROBENIUS_COEFFS: [[Self::Fp2; 5]; 4] = [
        [
            Bn254Fp2 {
                c0: Bn254Fp([
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
        ],
        [
            Bn254Fp2 {
                c0: Bn254Fp([
                    61, 85, 111, 23, 87, 149, 227, 153, 12, 51, 195, 194, 16, 195, 140, 183, 67,
                    177, 89, 245, 60, 236, 11, 76, 247, 17, 121, 79, 152, 71, 179, 47,
                ]),
                c1: Bn254Fp([
                    162, 203, 15, 100, 28, 213, 101, 22, 206, 157, 124, 11, 29, 42, 174, 50, 148,
                    7, 90, 215, 139, 204, 164, 75, 32, 174, 235, 97, 80, 229, 201, 22,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    98, 167, 30, 146, 85, 31, 138, 132, 114, 236, 148, 190, 247, 101, 51, 211, 132,
                    30, 24, 90, 183, 192, 243, 128, 1, 168, 238, 100, 94, 79, 181, 5,
                ]),
                c1: Bn254Fp([
                    38, 129, 43, 205, 17, 71, 59, 193, 99, 199, 222, 27, 234, 210, 133, 54, 146,
                    28, 11, 59, 176, 128, 58, 159, 238, 138, 253, 231, 219, 94, 20, 44,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    112, 228, 201, 220, 218, 53, 11, 214, 118, 33, 47, 41, 8, 30, 82, 92, 96, 139,
                    230, 118, 221, 159, 185, 232, 223, 167, 101, 40, 28, 183, 132, 18,
                ]),
                c1: Bn254Fp([
                    172, 98, 243, 128, 95, 240, 92, 202, 229, 199, 238, 142, 119, 146, 121, 116,
                    142, 11, 21, 18, 254, 124, 50, 166, 230, 231, 250, 180, 243, 150, 105, 36,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    90, 19, 160, 113, 70, 1, 84, 220, 152, 89, 201, 169, 237, 224, 170, 219, 185,
                    249, 226, 182, 152, 198, 94, 220, 220, 245, 154, 72, 5, 243, 60, 6,
                ]),
                c1: Bn254Fp([
                    227, 176, 35, 38, 99, 127, 211, 130, 210, 91, 162, 143, 201, 125, 128, 33, 43,
                    111, 121, 236, 167, 181, 4, 7, 154, 4, 65, 172, 188, 60, 192, 7,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    47, 105, 183, 234, 16, 200, 162, 46, 211, 27, 170, 85, 155, 69, 92, 66, 244,
                    63, 53, 164, 97, 54, 58, 233, 73, 134, 121, 79, 231, 193, 131, 1,
                ]),
                c1: Bn254Fp([
                    75, 44, 12, 110, 238, 184, 198, 36, 192, 42, 142, 103, 153, 203, 128, 176, 125,
                    159, 114, 199, 70, 178, 127, 162, 117, 6, 253, 118, 202, 242, 172, 18,
                ]),
            },
        ],
        [
            Bn254Fp2 {
                c0: Bn254Fp([
                    72, 253, 124, 96, 229, 68, 189, 228, 61, 110, 150, 187, 159, 6, 143, 194, 176,
                    204, 172, 224, 231, 217, 109, 94, 41, 160, 49, 225, 114, 78, 100, 48,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    254, 255, 255, 119, 49, 71, 99, 87, 79, 92, 219, 172, 241, 99, 242, 212, 172,
                    139, 212, 160, 206, 107, 226, 89, 0, 0, 0, 0, 0, 0, 0, 0,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    73, 253, 124, 96, 229, 68, 189, 228, 61, 110, 150, 187, 159, 6, 143, 194, 176,
                    204, 172, 224, 231, 217, 109, 94, 41, 160, 49, 225, 114, 78, 100, 48,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    70, 253, 124, 216, 22, 140, 32, 60, 141, 202, 113, 104, 145, 106, 129, 151, 93,
                    88, 129, 129, 182, 69, 80, 184, 41, 160, 49, 225, 114, 78, 100, 48,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    255, 255, 255, 119, 49, 71, 99, 87, 79, 92, 219, 172, 241, 99, 242, 212, 172,
                    139, 212, 160, 206, 107, 226, 89, 0, 0, 0, 0, 0, 0, 0, 0,
                ]),
                c1: Bn254Fp([
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0,
                ]),
            },
        ],
        [
            Bn254Fp2 {
                c0: Bn254Fp([
                    109, 251, 220, 123, 232, 110, 116, 123, 211, 66, 105, 93, 61, 253, 95, 128,
                    172, 37, 159, 149, 119, 28, 255, 186, 10, 239, 85, 183, 120, 224, 86, 8,
                ]),
                c1: Bn254Fp([
                    222, 134, 165, 170, 43, 171, 12, 56, 49, 38, 255, 152, 191, 49, 223, 15, 79, 9,
                    38, 236, 109, 14, 243, 169, 111, 118, 209, 179, 65, 222, 241, 4,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    102, 240, 203, 60, 188, 146, 26, 14, 203, 107, 176, 117, 69, 9, 51, 230, 78,
                    68, 178, 181, 247, 224, 190, 25, 171, 141, 192, 17, 102, 140, 197, 11,
                ]),
                c1: Bn254Fp([
                    159, 35, 12, 115, 157, 237, 227, 95, 229, 150, 127, 115, 8, 158, 74, 164, 4,
                    29, 210, 12, 239, 246, 176, 254, 18, 10, 145, 225, 153, 233, 213, 35,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    127, 166, 212, 30, 57, 125, 111, 232, 74, 210, 85, 190, 141, 179, 76, 137, 144,
                    170, 172, 208, 140, 96, 233, 239, 187, 228, 130, 204, 207, 129, 220, 25,
                ]),
                c1: Bn254Fp([
                    1, 193, 192, 244, 43, 170, 148, 118, 236, 57, 212, 151, 227, 165, 3, 127, 157,
                    19, 118, 53, 227, 238, 203, 6, 115, 125, 231, 11, 182, 248, 171, 0,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    237, 233, 220, 102, 208, 138, 204, 95, 244, 112, 168, 190, 163, 137, 214, 187,
                    163, 94, 158, 202, 29, 127, 241, 219, 76, 170, 150, 152, 109, 91, 39, 42,
                ]),
                c1: Bn254Fp([
                    100, 76, 89, 178, 179, 12, 77, 185, 186, 110, 207, 216, 199, 236, 0, 118, 50,
                    233, 7, 149, 14, 144, 75, 177, 143, 155, 240, 52, 182, 17, 164, 40,
                ]),
            },
            Bn254Fp2 {
                c0: Bn254Fp([
                    67, 27, 38, 118, 112, 132, 222, 235, 165, 132, 124, 150, 152, 128, 214, 46,
                    105, 63, 77, 59, 250, 153, 22, 113, 5, 9, 44, 149, 68, 144, 196, 19,
                ]),
                c1: Bn254Fp([
                    153, 36, 40, 132, 19, 4, 37, 31, 33, 128, 2, 32, 234, 218, 45, 62, 61, 99, 72,
                    42, 40, 178, 177, 159, 11, 221, 177, 89, 106, 54, 219, 22,
                ]),
            },
        ],
    ];
}
