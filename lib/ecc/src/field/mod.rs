#[cfg(feature = "halo2curves")]
mod exp_bytes_be;
use core::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[cfg(feature = "halo2curves")]
pub use exp_bytes_be::*;

/// This is a simplified trait for field elements.
pub trait Field:
    Sized
    + Eq
    + Copy
    + Clone
    + Default
    + Send
    + Sync
    + Debug
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Sum
    + Product
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> Sum<&'a Self>
    + for<'a> Product<&'a Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a Self>
{
    /// The zero element of the field, the additive identity.
    const ZERO: Self;

    /// The one element of the field, the multiplicative identity.
    const ONE: Self;

    /// Inverts this element, returning `None` if this element is zero.
    fn invert(&self) -> Option<Self>;
}

pub trait FieldExtension: Field {
    type BaseField: Field;
    type Coeffs: Sized;

    /// Generate an extension field element from its base field coefficients.
    fn from_coeffs(coeffs: Self::Coeffs) -> Self;

    /// Embed a base field element into an extension field element.
    fn embed(base_elem: Self::BaseField) -> Self;

    /// Conjuagte an extension field element.
    fn conjugate(&self) -> Self;

    /// Frobenius map
    fn frobenius_map(&self, power: Option<usize>) -> Self;

    /// Multiply an extension field element by an element in the base field
    fn mul_base(&self, rhs: Self::BaseField) -> Self;
}
