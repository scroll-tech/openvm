use core::ops::{Add, Mul, Sub};

#[cfg(target_os = "zkvm")]
use {
    axvm_platform::constants::{Custom1Funct3, PairingBaseFunct7, CUSTOM_1},
    axvm_platform::custom_insn_r,
    core::mem::MaybeUninit,
};

use crate::field::{Field, FieldExtension, Xi};

#[derive(Clone, Copy, Debug)]
pub struct UnevaluatedLine<Fp, Fp2>
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
{
    pub b: Fp2,
    pub c: Fp2,
}

impl<Fp, Fp2> UnevaluatedLine<Fp, Fp2>
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
{
    pub fn evaluate(&self, xy: &(Fp, Fp)) -> EvaluatedLine<Fp, Fp2> {
        #[cfg(not(target_os = "zkvm"))]
        {
            let x_over_y = &xy.0;
            let y_inv = &xy.1;
            EvaluatedLine {
                b: self.b.mul_base(x_over_y.clone()),
                c: self.c.mul_base(y_inv.clone()),
            }
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<EvaluatedLine<Fp, Fp2>> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::EvaluateLine as usize,
                uninit.as_mut_ptr(),
                self as *const UnevaluatedLine<Fp, Fp2>,
                xy as *const (Fp, Fp)
            );
            unsafe { uninit.assume_init() }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EvaluatedLine<Fp, Fp2>
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
{
    pub b: Fp2,
    pub c: Fp2,
}

impl<Fp, Fp2> IntoIterator for EvaluatedLine<Fp, Fp2>
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
{
    type Item = Fp2;
    type IntoIter = core::array::IntoIter<Fp2, 2>;
    fn into_iter(self) -> Self::IntoIter {
        [self.b, self.c].into_iter()
    }
}

/// Convert M-type lines into Fp12 elements
pub trait LineMType<Fp, Fp2, Fp12>
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
    Fp12: FieldExtension<BaseField = Fp2>,
{
    fn from_evaluated_line_m_type(line: EvaluatedLine<Fp, Fp2>) -> Fp12;
}

/// Trait definition for line multiplication opcodes for M-type lines
pub trait LineMulMType<Fp, Fp2, Fp12>
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp> + Xi,
    Fp12: FieldExtension<BaseField = Fp2, Coeffs = [Fp2; 6]>,
    for<'a> &'a Fp2: Add<&'a Fp2, Output = Fp2>,
    for<'a> &'a Fp2: Sub<&'a Fp2, Output = Fp2>,
    for<'a> &'a Fp2: Mul<&'a Fp2, Output = Fp2>,
{
    /// Multiplies two lines in 023-form to get an element in 02345-form
    fn mul_023_by_023(l0: EvaluatedLine<Fp, Fp2>, l1: EvaluatedLine<Fp, Fp2>) -> [Fp2; 5] {
        #[cfg(not(target_os = "zkvm"))]
        {
            let b0 = l0.b;
            let c0 = l0.c;
            let b1 = l1.b;
            let c1 = l1.c;

            // where w⁶ = xi
            // l0 * l1 = c0c1 + (c0b1 + c1b0)w² + (c0 + c1)w³ + (b0b1)w⁴ + (b0 +b1)w⁵ + w⁶
            //         = (c0c1 + xi) + (c0b1 + c1b0)w² + (c0 + c1)w³ + (b0b1)w⁴ + (b0 + b1)w⁵
            let x0 = &c0 * &c1 + Fp2::XI;
            let x2 = &c0 * &b1 + &c1 * &b0;
            let x3 = &c0 + &c1;
            let x4 = &b0 * &b1;
            let x5 = &b0 + &b1;

            [x0, x2, x3, x4, x5]
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<[Fp2; 5]> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::Mul023By023 as usize,
                uninit.as_mut_ptr(),
                &l0 as *const EvaluatedLine<Fp, Fp2>,
                &l1 as *const EvaluatedLine<Fp, Fp2>
            );
            unsafe { uninit.assume_init() }
        }
    }

    /// Multiplies a line in 02345-form with a Fp12 element to get an Fp12 element
    fn mul_by_023(f: Fp12, l: EvaluatedLine<Fp, Fp2>) -> Fp12 {
        #[cfg(not(target_os = "zkvm"))]
        {
            Self::mul_by_02345(f, [l.c, l.b, Fp2::one(), Fp2::zero(), Fp2::zero()])
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<Fp12> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::MulBy023 as usize,
                uninit.as_mut_ptr(),
                &f as *const Fp12,
                &l as *const EvaluatedLine<Fp, Fp2>
            );
            unsafe { uninit.assume_init() }
        }
    }

    /// Multiplies a line in 02345-form with a Fp12 element to get an Fp12 element
    fn mul_by_02345(f: Fp12, x: [Fp2; 5]) -> Fp12 {
        #[cfg(not(target_os = "zkvm"))]
        {
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
            let o0 = &x[0]; // coeff x0
            let o1 = &x[1]; // coeff x2
            let o2 = &x[3]; // coeff x4
            let o4 = &x[2]; // coeff x3
            let o5 = &x[4]; // coeff x5

            let xi = &Fp2::XI;

            let self_coeffs = f.to_coeffs();
            let s0 = &self_coeffs[0];
            let s1 = &self_coeffs[2];
            let s2 = &self_coeffs[4];
            let s3 = &self_coeffs[1];
            let s4 = &self_coeffs[3];
            let s5 = &self_coeffs[5];

            // NOTE[yj]: Hand-calculated multiplication for Fp12 * 02345 ∈ Fp2; this is likely not the most efficient implementation
            // c0 = cs0co0 + xi(cs1co2 + cs2co1 + cs3co5 + cs4co4)
            // c1 = cs0co1 + cs1co0 + xi(cs2co2 + cs4co5 + cs5co4)
            // c2 = cs0co2 + cs1co1 + cs2co0 + cs3co4 + xi(cs5co5)
            // c3 = cs3co0 + xi(cs1co5 + cs2co4 + cs4co2 + cs5co1)
            // c4 = cs0co4 + cs3co1 + cs4co0 + xi(cs2co5 + cs5co2)
            // c5 = cs0co5 + cs1co4 + cs3co2 + cs4co1 + cs5co0
            //   where cs*: self.c*
            let c0 = s0 * o0 + xi * &(s1 * o2 + s2 * o1 + s3 * o5 + s4 * o4);
            let c1 = s0 * o1 + s1 * o0 + xi * &(s2 * o2 + s4 * o5 + s5 * o4);
            let c2 = s0 * o2 + s1 * o1 + s2 * o0 + s3 * o4 + xi * &(s5 * o5);
            let c3 = s3 * o0 + xi * &(s1 * o5 + s2 * o4 + s4 * o2 + s5 * o1);
            let c4 = s0 * o4 + s3 * o1 + s4 * o0 + xi * &(s2 * o5 + s5 * o2);
            let c5 = s0 * o5 + s1 * o4 + s3 * o2 + s4 * o1 + s5 * o0;

            Fp12::from_coeffs([c0, c3, c1, c4, c2, c5])
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<Fp12> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::MulBy02345 as usize,
                uninit.as_mut_ptr(),
                &f as *const Fp12,
                &x as *const [Fp2; 5]
            );
            unsafe { uninit.assume_init() }
        }
    }
}

/// Convert D-type lines into Fp12 elements
pub trait LineDType<Fp, Fp2, Fp12>
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp>,
    Fp12: FieldExtension<BaseField = Fp2>,
{
    fn from_evaluated_line_d_type(line: EvaluatedLine<Fp, Fp2>) -> Fp12;
}

/// Trait definition for line multiplication opcodes for D-type lines
pub trait LineMulDType<Fp, Fp2, Fp12>
where
    Fp: Field,
    Fp2: FieldExtension<BaseField = Fp> + Xi,
    Fp12: FieldExtension<BaseField = Fp2, Coeffs = [Fp2; 6]>,
    for<'a> &'a Fp2: Add<&'a Fp2, Output = Fp2>,
    for<'a> &'a Fp2: Sub<&'a Fp2, Output = Fp2>,
    for<'a> &'a Fp2: Mul<&'a Fp2, Output = Fp2>,
{
    /// Multiplies two lines in 013-form to get an element in 01234-form
    fn mul_013_by_013(l0: EvaluatedLine<Fp, Fp2>, l1: EvaluatedLine<Fp, Fp2>) -> [Fp2; 5] {
        #[cfg(not(target_os = "zkvm"))]
        {
            let b0 = l0.b;
            let c0 = l0.c;
            let b1 = l1.b;
            let c1 = l1.c;

            // where w⁶ = xi
            // l0 * l1 = 1 + (b0 + b1)w + (b0b1)w² + (c0 + c1)w³ + (b0c1 + b1c0)w⁴ + (c0c1)w⁶
            //         = (1 + c0c1 * xi) + (b0 + b1)w + (b0b1)w² + (c0 + c1)w³ + (b0c1 + b1c0)w⁴
            let x0 = Fp2::one() + &c0 * &c1 * &Fp2::XI;
            let x1 = &b0 + &b1;
            let x2 = &b0 * &b1;
            let x3 = &c0 + &c1;
            let x4 = &b0 * &c1 + &b1 * &c0;

            [x0, x1, x2, x3, x4]
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<[Fp2; 5]> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::Mul013By013 as usize,
                uninit.as_mut_ptr(),
                &l0 as *const EvaluatedLine<Fp, Fp2>,
                &l1 as *const EvaluatedLine<Fp, Fp2>
            );
            unsafe { uninit.assume_init() }
        }
    }

    /// Multiplies a line in 013-form with a Fp12 element to get an Fp12 element
    fn mul_by_013(f: Fp12, l: EvaluatedLine<Fp, Fp2>) -> Fp12 {
        #[cfg(not(target_os = "zkvm"))]
        {
            Self::mul_by_01234(f, [Fp2::one(), l.b, Fp2::zero(), l.c, Fp2::zero()])
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<Fp12> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::MulBy013 as usize,
                uninit.as_mut_ptr(),
                &f as *const Fp12,
                &l as *const EvaluatedLine<Fp, Fp2>
            );
            unsafe { uninit.assume_init() }
        }
    }

    /// Multiplies a line in 01234-form with a Fp12 element to get an Fp12 element
    fn mul_by_01234(f: Fp12, x: [Fp2; 5]) -> Fp12 {
        #[cfg(not(target_os = "zkvm"))]
        {
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
            let o3 = &x[1];
            let o4 = &x[3];

            let xi = &Fp2::XI;

            let self_coeffs = f.to_coeffs();
            let s0 = &self_coeffs[0];
            let s1 = &self_coeffs[2];
            let s2 = &self_coeffs[4];
            let s3 = &self_coeffs[1];
            let s4 = &self_coeffs[3];
            let s5 = &self_coeffs[5];

            // NOTE[yj]: Hand-calculated multiplication for Fp12 * 01234 ∈ Fp2; this is likely not the most efficient implementation
            // c0 = cs0co0 + xi(cs1co2 + cs2co1 + cs4co4 + cs5co3)
            // c1 = cs0co1 + cs1co0 + cs3co3 + xi(cs2co2 + cs5co4)
            // c2 = cs0co2 + cs1co1 + cs2co0 + cs3co4 + cs4co3
            // c3 = cs0co3 + cs3co0 + xi(cs2co4 + cs4co2 + cs5co1)
            // c4 = cs0co4 + cs1co3 + cs3co1 + cs4co0 + xi(cs5co2)
            // c5 = cs1co4 + cs2co3 + cs3co2 + cs4co1 + cs5co0
            let c0 = s0 * o0 + xi * &(s1 * o2 + s2 * o1 + s4 * o4 + s5 * o3);
            let c1 = s0 * o1 + s1 * o0 + s3 * o3 + xi * &(s2 * o2 + s5 * o4);
            let c2 = s0 * o2 + s1 * o1 + s2 * o0 + s3 * o4 + s4 * o3;
            let c3 = s0 * o3 + s3 * o0 + xi * &(s2 * o4 + s4 * o2 + s5 * o1);
            let c4 = s0 * o4 + s1 * o3 + s3 * o1 + s4 * o0 + xi * &(s5 * o2);
            let c5 = s1 * o4 + s2 * o3 + s3 * o2 + s4 * o1 + s5 * o0;

            Fp12::from_coeffs([c0, c3, c1, c4, c2, c5])
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<Fp12> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                PairingBaseFunct7::MulBy01234 as usize,
                uninit.as_mut_ptr(),
                &f as *const Fp12,
                &x as *const [Fp2; 5]
            );
            unsafe { uninit.assume_init() }
        }
    }
}
