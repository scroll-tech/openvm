use halo2curves_axiom::ff::Field;

pub trait FieldExtension: Field {
    type BaseField: Field;

    /// Lift a base field element into an extension field element.
    fn lift(base: &Self::BaseField) -> Self;

    /// Frobenius map
    fn frobenius_map(&mut self, power: usize);

    /// Multiply an extension field element by an element in the base field
    fn mul_base(self, rhs: &Self::BaseField) -> Self;
}
