use halo2curves_axiom::ff::Field;

pub trait FieldExtension<const DEG: usize>: Field {
    type BaseField: Field;

    /// Generate an extension field element from its base field coefficients.
    fn from_coeffs(coeffs: [Self::BaseField; DEG]) -> Self;

    /// Embed a base field element into an extension field element.
    fn embed(base_elem: &Self::BaseField) -> Self;

    /// Conjuagte an extension field element.
    // fn conjugate(&mut self);

    /// Frobenius map
    fn frobenius_map(&mut self, power: Option<usize>);

    /// Multiply an extension field element by an element in the base field
    fn mul_base(self, rhs: &Self::BaseField) -> Self;
}
