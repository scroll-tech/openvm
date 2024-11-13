use axvm::{
    intrinsics::{DivUnsafe, IntMod},
    moduli_setup,
};

use crate::field::{Complex, Field, FieldExtension, SexticExtField, Xi};

mod line;
pub use line::*;

pub struct Bn254;

// impl Bn254 {
//     pub const XI: Fp2 = Fp2::new(Fp::from_const_u8(9), Fp::from_const_u8(1));
// }

moduli_setup! {
    Fp = "21888242871839275222246405745257275088696311157297823662689037894645226208583";
}
// type Fp2 = Complex<Fp>;
// type Fp12 = SexticExtField<Fp2>;

// impl Xi for Fp2 {
//     const XI: Self = Self::new(Fp::from_const_u8(9), Fp::from_const_u8(1));
// }

mod field_impl {
    use axvm::intrinsics::IntMod;

    use super::Fp;
    use crate::field::Field;

    impl Field for Fp
    where
        Self: IntMod + Clone + core::fmt::Debug,
    {
        type SelfRef<'a> = &'a Self;

        const ZERO: Self = <Self as IntMod>::ZERO;
        const ONE: Self = <Self as IntMod>::ONE;

        fn square(&self) -> Self {
            todo!()
            // IntMod::square(self)
        }

        fn invert(&self) -> Option<Self> {
            todo!()
            // Some(<Fp as IntMod>::ONE.div_unsafe(self))
        }
    }
}
pub use field_impl::*;

// impl FieldExtension for Bn254Fp2 {
//     type BaseField = Bn254Fp;
//     type Coeffs = [Bn254Fp; 2];
//     type SelfRef<'a> = &'a Self;

//     fn from_coeffs(coeffs: Self::Coeffs) -> Self {
//         Self {
//             c0: coeffs[0].clone(),
//             c1: coeffs[1].clone(),
//         }
//     }

//     fn to_coeffs(self) -> Self::Coeffs {
//         [self.c0, self.c1]
//     }

//     fn embed(base_elem: Self::BaseField) -> Self {
//         Self {
//             c0: base_elem,
//             c1: <Self::BaseField as Field>::ZERO,
//         }
//     }

//     fn conjugate(&self) -> Self {
//         Self {
//             c0: self.c0.clone(),
//             c1: -self.c1.clone(),
//         }
//     }

//     fn frobenius_map(&self, power: Option<usize>) -> Self {
//         todo!()
//     }

//     fn mul_base(&self, rhs: Self::BaseField) -> Self {
//         Self {
//             c0: &self.c0 * &rhs,
//             c1: &self.c1 * &rhs,
//         }
//     }
// }
