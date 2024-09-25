use halo2curves_axiom::ff::Field;

pub fn fp12_square<Fp12: Field>(x: Fp12) -> Fp12 {
    fp12_multiply(x, x)
}

pub fn fp12_multiply<Fp12: Field>(x: Fp12, y: Fp12) -> Fp12 {
    x * y
}
