use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use halo2curves_axiom::{
    bn256::{Fq, Fq12, Fq2, Fq6},
    ff::Field,
};
use subtle::{Choice, CtOption};

#[cfg(test)]
use crate::common::FeltPrint;
use crate::common::{
    EvaluatedLine, ExpBigInt, FieldExtension, Fp12Constructor, Fp2Constructor, LineDType,
};

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct FieldExtFq2(pub(crate) Fq2);

impl Fp2Constructor<Fq> for FieldExtFq2 {
    fn new(c0: Fq, c1: Fq) -> Self {
        FieldExtFq2(Fq2 { c0, c1 })
    }
}

impl Fp12Constructor<FieldExtFq2> for FieldExtFq12 {
    fn new(
        c00: FieldExtFq2,
        c01: FieldExtFq2,
        c02: FieldExtFq2,
        c10: FieldExtFq2,
        c11: FieldExtFq2,
        c12: FieldExtFq2,
    ) -> Self {
        FieldExtFq12(Fq12 {
            c0: Fq6 {
                c0: c00.0,
                c1: c01.0,
                c2: c02.0,
            },
            c1: Fq6 {
                c0: c10.0,
                c1: c11.0,
                c2: c12.0,
            },
        })
    }
}

/// FieldExtension for Fq2 with Fq as base field
impl FieldExtension for FieldExtFq2 {
    type BaseField = Fq;

    fn from_coeffs(coeffs: &[Self::BaseField]) -> Self {
        assert!(coeffs.len() <= 2, "coeffs must have at most 2 elements");
        let mut coeffs = coeffs.to_vec();
        coeffs.resize(2, Self::BaseField::ZERO);

        FieldExtFq2(Fq2 {
            c0: coeffs[0],
            c1: coeffs[1],
        })
    }

    fn embed(base_elem: &Self::BaseField) -> Self {
        FieldExtFq2(Fq2 {
            c0: *base_elem,
            c1: Fq::ZERO,
        })
    }

    fn conjugate(&self) -> Self {
        let mut s = self.0;
        Fq2::conjugate(&mut s);
        FieldExtFq2(s)
    }

    fn frobenius_map(&self, power: Option<usize>) -> Self {
        let mut s = self.0;
        Fq2::frobenius_map(&mut s, power.unwrap());
        FieldExtFq2(s)
    }

    fn mul_base(&self, rhs: &Self::BaseField) -> Self {
        FieldExtFq2(Fq2 {
            c0: self.0.c0 * rhs,
            c1: self.0.c1 * rhs,
        })
    }
}

impl Field for FieldExtFq2 {
    const ZERO: Self = Self(Fq2::ZERO);
    const ONE: Self = Self(Fq2::ONE);

    fn random(mut rng: impl rand_core::RngCore) -> Self {
        Self(Fq2::random(&mut rng))
    }

    fn square(&self) -> Self {
        Self(self.0.square())
    }

    fn double(&self) -> Self {
        Self(self.0.double())
    }

    fn invert(&self) -> CtOption<Self> {
        self.0.invert().map(Self)
    }

    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        let (choice, sqrt) = Fq2::sqrt_ratio(&num.0, &div.0);
        (choice, Self(sqrt))
    }
}

impl Add for FieldExtFq2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<'a> Add<&'a FieldExtFq2> for FieldExtFq2 {
    type Output = Self;
    fn add(self, rhs: &'a FieldExtFq2) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<'a> Add<FieldExtFq2> for &'a FieldExtFq2 {
    type Output = FieldExtFq2;
    fn add(self, rhs: FieldExtFq2) -> FieldExtFq2 {
        FieldExtFq2(self.0 + rhs.0)
    }
}

impl<'a, 'b> Add<&'b FieldExtFq2> for &'a FieldExtFq2 {
    type Output = FieldExtFq2;
    fn add(self, rhs: &'b FieldExtFq2) -> FieldExtFq2 {
        FieldExtFq2(self.0 + rhs.0)
    }
}

impl Sub for FieldExtFq2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl<'a> Sub<&'a FieldExtFq2> for FieldExtFq2 {
    type Output = Self;
    fn sub(self, rhs: &'a FieldExtFq2) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl<'a> Sub<FieldExtFq2> for &'a FieldExtFq2 {
    type Output = FieldExtFq2;
    fn sub(self, rhs: FieldExtFq2) -> FieldExtFq2 {
        FieldExtFq2(self.0 - rhs.0)
    }
}

impl<'a, 'b> Sub<&'b FieldExtFq2> for &'a FieldExtFq2 {
    type Output = FieldExtFq2;
    fn sub(self, rhs: &'b FieldExtFq2) -> FieldExtFq2 {
        FieldExtFq2(self.0 - rhs.0)
    }
}

impl Mul for FieldExtFq2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl<'a> Mul<&'a FieldExtFq2> for FieldExtFq2 {
    type Output = Self;
    fn mul(self, rhs: &'a FieldExtFq2) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl<'a> Mul<FieldExtFq2> for &'a FieldExtFq2 {
    type Output = FieldExtFq2;
    fn mul(self, rhs: FieldExtFq2) -> FieldExtFq2 {
        FieldExtFq2(self.0 * rhs.0)
    }
}

impl<'a, 'b> Mul<&'b FieldExtFq2> for &'a FieldExtFq2 {
    type Output = FieldExtFq2;
    fn mul(self, rhs: &'b FieldExtFq2) -> FieldExtFq2 {
        FieldExtFq2(self.0 * rhs.0)
    }
}

impl Neg for FieldExtFq2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl AddAssign for FieldExtFq2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(&rhs.0);
    }
}

impl<'a> AddAssign<&'a FieldExtFq2> for FieldExtFq2 {
    fn add_assign(&mut self, rhs: &'a FieldExtFq2) {
        self.0.add_assign(&rhs.0);
    }
}

impl SubAssign for FieldExtFq2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(&rhs.0);
    }
}

impl<'a> SubAssign<&'a FieldExtFq2> for FieldExtFq2 {
    fn sub_assign(&mut self, rhs: &'a FieldExtFq2) {
        self.0.sub_assign(&rhs.0);
    }
}

impl MulAssign for FieldExtFq2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign(&rhs.0);
    }
}

impl<'a> MulAssign<&'a FieldExtFq2> for FieldExtFq2 {
    fn mul_assign(&mut self, rhs: &'a FieldExtFq2) {
        self.0.mul_assign(&rhs.0);
    }
}

impl<'a> std::iter::Sum<&'a FieldExtFq2> for FieldExtFq2 {
    fn sum<I: Iterator<Item = &'a FieldExtFq2>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl std::iter::Sum for FieldExtFq2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl<'a> std::iter::Product<&'a FieldExtFq2> for FieldExtFq2 {
    fn product<I: Iterator<Item = &'a FieldExtFq2>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, x| acc * x)
    }
}

impl std::iter::Product for FieldExtFq2 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, x| acc * x)
    }
}

impl subtle::ConstantTimeEq for FieldExtFq2 {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl subtle::ConditionallySelectable for FieldExtFq2 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(Fq2::conditional_select(&a.0, &b.0, choice))
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct FieldExtFq12(pub(crate) Fq12);

/// FieldExtension for Fq12 with Fq6 as base field since halo2curves does not implement `Field` for Fq6.
impl FieldExtension for FieldExtFq12 {
    type BaseField = FieldExtFq2;

    fn from_coeffs(coeffs: &[Self::BaseField]) -> Self {
        assert!(coeffs.len() <= 6, "coeffs must have at most 6 elements");
        let mut coeffs = coeffs.iter().map(|x| x.0).collect::<Vec<_>>();
        coeffs.resize(6, Self::BaseField::ZERO.0);

        FieldExtFq12(Fq12 {
            c0: Fq6 {
                c0: coeffs[0],
                c1: coeffs[2],
                c2: coeffs[4],
            },
            c1: Fq6 {
                c0: coeffs[1],
                c1: coeffs[3],
                c2: coeffs[5],
            },
        })
    }

    fn embed(base_elem: &Self::BaseField) -> Self {
        let fq6_pt = Fq6 {
            c0: base_elem.0,
            c1: Fq2::zero(),
            c2: Fq2::zero(),
        };
        FieldExtFq12(Fq12 {
            c0: fq6_pt,
            c1: Fq6::zero(),
        })
    }

    fn conjugate(&self) -> Self {
        let mut s = self.0;
        Fq12::conjugate(&mut s);
        FieldExtFq12(s)
    }

    fn frobenius_map(&self, power: Option<usize>) -> Self {
        let mut s = self.0;
        Fq12::frobenius_map(&mut s, power.unwrap());
        FieldExtFq12(s)
    }

    fn mul_base(&self, rhs: &Self::BaseField) -> Self {
        let fq6_pt = Fq6 {
            c0: rhs.0,
            c1: Fq2::zero(),
            c2: Fq2::zero(),
        };
        FieldExtFq12(Fq12 {
            c0: self.0.c0 * fq6_pt,
            c1: self.0.c1 * fq6_pt,
        })
    }
}

impl Field for FieldExtFq12 {
    const ZERO: Self = Self(Fq12::ZERO);
    const ONE: Self = Self(Fq12::ONE);

    fn random(mut rng: impl rand_core::RngCore) -> Self {
        Self(Fq12::random(&mut rng))
    }

    fn square(&self) -> Self {
        Self(self.0.square())
    }

    fn double(&self) -> Self {
        Self(self.0.double())
    }

    fn invert(&self) -> CtOption<Self> {
        self.0.invert().map(Self)
    }

    fn sqrt_ratio(num: &Self, div: &Self) -> (Choice, Self) {
        let (choice, sqrt) = Fq12::sqrt_ratio(&num.0, &div.0);
        (choice, Self(sqrt))
    }
}

impl Add for FieldExtFq12 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<'a> Add<&'a FieldExtFq12> for FieldExtFq12 {
    type Output = Self;
    fn add(self, rhs: &'a FieldExtFq12) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<'a> Add<FieldExtFq12> for &'a FieldExtFq12 {
    type Output = FieldExtFq12;
    fn add(self, rhs: FieldExtFq12) -> FieldExtFq12 {
        FieldExtFq12(self.0 + rhs.0)
    }
}

impl<'a, 'b> Add<&'b FieldExtFq12> for &'a FieldExtFq12 {
    type Output = FieldExtFq12;
    fn add(self, rhs: &'b FieldExtFq12) -> FieldExtFq12 {
        FieldExtFq12(self.0 + rhs.0)
    }
}

impl Sub for FieldExtFq12 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl<'a> Sub<&'a FieldExtFq12> for FieldExtFq12 {
    type Output = Self;
    fn sub(self, rhs: &'a FieldExtFq12) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl<'a> Sub<FieldExtFq12> for &'a FieldExtFq12 {
    type Output = FieldExtFq12;
    fn sub(self, rhs: FieldExtFq12) -> FieldExtFq12 {
        FieldExtFq12(self.0 - rhs.0)
    }
}

impl<'a, 'b> Sub<&'b FieldExtFq12> for &'a FieldExtFq12 {
    type Output = FieldExtFq12;
    fn sub(self, rhs: &'b FieldExtFq12) -> FieldExtFq12 {
        FieldExtFq12(self.0 - rhs.0)
    }
}

impl Mul for FieldExtFq12 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl<'a> Mul<&'a FieldExtFq12> for FieldExtFq12 {
    type Output = Self;
    fn mul(self, rhs: &'a FieldExtFq12) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl<'a> Mul<FieldExtFq12> for &'a FieldExtFq12 {
    type Output = FieldExtFq12;
    fn mul(self, rhs: FieldExtFq12) -> FieldExtFq12 {
        FieldExtFq12(self.0 * rhs.0)
    }
}

impl<'a, 'b> Mul<&'b FieldExtFq12> for &'a FieldExtFq12 {
    type Output = FieldExtFq12;
    fn mul(self, rhs: &'b FieldExtFq12) -> FieldExtFq12 {
        FieldExtFq12(self.0 * rhs.0)
    }
}

impl Neg for FieldExtFq12 {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl AddAssign for FieldExtFq12 {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(&rhs.0);
    }
}

impl<'a> AddAssign<&'a FieldExtFq12> for FieldExtFq12 {
    fn add_assign(&mut self, rhs: &'a FieldExtFq12) {
        self.0.add_assign(&rhs.0);
    }
}

impl SubAssign for FieldExtFq12 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0.sub_assign(&rhs.0);
    }
}

impl<'a> SubAssign<&'a FieldExtFq12> for FieldExtFq12 {
    fn sub_assign(&mut self, rhs: &'a FieldExtFq12) {
        self.0.sub_assign(&rhs.0);
    }
}

impl MulAssign for FieldExtFq12 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0.mul_assign(&rhs.0);
    }
}

impl<'a> MulAssign<&'a FieldExtFq12> for FieldExtFq12 {
    fn mul_assign(&mut self, rhs: &'a FieldExtFq12) {
        self.0.mul_assign(&rhs.0);
    }
}

impl<'a> std::iter::Sum<&'a FieldExtFq12> for FieldExtFq12 {
    fn sum<I: Iterator<Item = &'a FieldExtFq12>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl std::iter::Sum for FieldExtFq12 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, x| acc + x)
    }
}

impl<'a> std::iter::Product<&'a FieldExtFq12> for FieldExtFq12 {
    fn product<I: Iterator<Item = &'a FieldExtFq12>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, x| acc * x)
    }
}

impl std::iter::Product for FieldExtFq12 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, x| acc * x)
    }
}

impl subtle::ConstantTimeEq for FieldExtFq12 {
    fn ct_eq(&self, other: &Self) -> Choice {
        self.0.ct_eq(&other.0)
    }
}

impl subtle::ConditionallySelectable for FieldExtFq12 {
    fn conditional_select(a: &Self, b: &Self, choice: Choice) -> Self {
        Self(Fq12::conditional_select(&a.0, &b.0, choice))
    }
}

impl LineDType<Fq, FieldExtFq2, FieldExtFq12> for FieldExtFq12 {
    fn from_evaluated_line_d_type(line: EvaluatedLine<Fq, FieldExtFq2>) -> Self {
        FieldExtFq12::from_coeffs(&[
            FieldExtFq2::ONE,
            line.b,
            FieldExtFq2::ZERO,
            line.c,
            FieldExtFq2::ZERO,
            FieldExtFq2::ZERO,
        ])
    }
}

impl ExpBigInt<FieldExtFq12> for FieldExtFq12 {}

#[cfg(test)]
impl FeltPrint<Fq> for Fq {
    fn felt_print(&self, label: &str) {
        println!("{} {:?}", label, self.0);
    }
}

#[cfg(test)]
impl FeltPrint<Fq12> for Fq12 {
    fn felt_print(&self, label: &str) {
        println!("felt_print - {}", label);
        print!("c0.c0.c0:");
        self.c0.c0.c0.felt_print("");
        print!("c0.c0.c1:");
        self.c0.c0.c1.felt_print("");
        print!("c0.c1.c0:");
        self.c0.c1.c0.felt_print("");
        print!("c0.c1.c1:");
        self.c0.c1.c1.felt_print("");
        print!("c0.c2.c0:");
        self.c0.c2.c0.felt_print("");
        print!("c0.c2.c1:");
        self.c0.c2.c1.felt_print("");
        print!("c1.c0.c0:");
        self.c1.c0.c0.felt_print("");
        print!("c1.c0.c1:");
        self.c1.c0.c1.felt_print("");
        print!("c1.c1.c0:");
        self.c1.c1.c0.felt_print("");
        print!("c1.c1.c1:");
        self.c1.c1.c1.felt_print("");
        print!("c1.c2.c0:");
        self.c1.c2.c0.felt_print("");
        print!("c1.c2.c1:");
        self.c1.c2.c1.felt_print("");
    }
}
