pub struct EcPoint<F> {
    pub x: F,
    pub y: F,
}

/// \xi = u + xi_0
pub struct Xi<F> {
    pub u: F,
    pub xi_0: F,
}
