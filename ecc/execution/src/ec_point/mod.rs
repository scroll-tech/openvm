pub struct EcPoint<F> {
    pub x: F,
    pub y: F,
}

/// \xi = constant + u
pub struct Xi<F> {
    pub constant: F,
    pub u: F,
}
