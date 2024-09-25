use halo2curves_axiom::ff::Field;

pub trait FieldExtension<const N: usize>: Field {
    type BaseField: Field;

    fn from_coeffs(coeffs: [Self::BaseField; N]) -> Self;

    /// Lift a base field element into an extension field element.
    fn embed(base_elem: &Self::BaseField) -> Self;

    /// Frobenius map
    fn frobenius_map(&mut self, power: usize);

    /// Multiply an extension field element by an element in the base field
    fn mul_base(self, rhs: &Self::BaseField) -> Self;
}
