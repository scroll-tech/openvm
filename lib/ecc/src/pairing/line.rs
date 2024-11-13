use core::ops::{Add, Mul, Sub};

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
    pub fn evaluate(&self, (x_over_y, y_inv): &(Fp, Fp)) -> EvaluatedLine<Fp, Fp2> {
        EvaluatedLine {
            b: self.b.mul_base(x_over_y.clone()),
            c: self.c.mul_base(y_inv.clone()),
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
    Fp2: FieldExtension<BaseField = Fp>,
    Fp12: FieldExtension<BaseField = Fp2>,
{
    fn mul_023_by_023(l0: EvaluatedLine<Fp, Fp2>, l1: EvaluatedLine<Fp, Fp2>) -> [Fp2; 5];

    fn mul_by_023(f: Fp12, l: EvaluatedLine<Fp, Fp2>) -> Fp12;

    fn mul_by_02345(f: Fp12, x: [Fp2; 5]) -> Fp12;
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
    fn mul_013_by_013(line0: EvaluatedLine<Fp, Fp2>, line1: EvaluatedLine<Fp, Fp2>) -> [Fp2; 5] {
        #[cfg(not(target_os = "zkvm"))]
        {
            let b0 = line0.b;
            let c0 = line0.c;
            let b1 = line1.b;
            let c1 = line1.c;

            // where w⁶ = xi
            // l0 * l1 = 1 + (b0 + b1)w + (b0b1)w² + (c0 + c1)w³ + (b0c1 + b1c0)w⁴ + (c0c1)w⁶
            //         = (1 + c0c1 * xi) + (b0 + b1)w + (b0b1)w² + (c0 + c1)w³ + (b0c1 + b1c0)w⁴
            let l0 = Fp2::ONE + &c0 * &c1 * Fp2::XI;
            let l1 = &b0 + &b1;
            let l2 = &b0 * &b1;
            let l3 = &c0 + &c1;
            let l4 = &b0 * &c1 + &b1 * &c0;

            [l0, l1, l2, l3, l4]
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<[Fp2; 5]> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                SwBaseFunct7::Mul013By013 as usize,
                uninit.as_mut_ptr(),
                line0 as *const EvaluatedLine<Fp, Fp2>,
                line1 as *const EvaluatedLine<Fp, Fp2>
            );
            unsafe { uninit.assume_init() }
        }
    }

    fn mul_by_013(f: Fp12, l: EvaluatedLine<Fp, Fp2>) -> Fp12 {
        #[cfg(not(target_os = "zkvm"))]
        {
            Self::mul_by_01234(f, [Fp2::ONE, l.b, Fp2::ZERO, l.c, Fp2::ZERO])
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<Fp12> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                SwBaseFunct7::MulBy013 as usize,
                uninit.as_mut_ptr(),
                f as *const Fp12,
                l as *const EvaluatedLine<Fp, Fp2>
            );
            unsafe { uninit.assume_init() }
        }
    }

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
            let s1 = &self_coeffs[1];
            let s2 = &self_coeffs[2];
            let s3 = &self_coeffs[3];
            let s4 = &self_coeffs[4];
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

            Fp12::from_coeffs([c0, c1, c2, c3, c4, c5])
        }
        #[cfg(target_os = "zkvm")]
        {
            let mut uninit: MaybeUninit<Fp12> = MaybeUninit::uninit();
            custom_insn_r!(
                CUSTOM_1,
                Custom1Funct3::Pairing as usize,
                SwBaseFunct7::MulBy01234 as usize,
                uninit.as_mut_ptr(),
                f as *const Fp12,
                x as *const [Fp2; 5]
            );
            unsafe { uninit.assume_init() }
        }
    }
}
