use super::{Field, FieldExt};

#[derive(Clone, PartialEq, Eq)]
#[repr(C)]
pub struct SexticExtField<F> {
    pub(crate) c: [F; 6],
}

impl<F: Field> FieldExt for SexticExtField<F> {
    type BaseField = F;
    type Coeffs = [F; 6];
    type SelfRef<'a>
    where
        F: 'a,
    = &'a Self;

    fn from_coeffs(coeffs: Self::Coeffs) -> Self {
        Self { c: coeffs }
    }
}
