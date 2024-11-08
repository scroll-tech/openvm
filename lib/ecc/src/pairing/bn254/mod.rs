use lazy_static::lazy_static;

mod line;
pub use line::*;

mod miller_step;
pub use miller_step::*;

pub struct Bn254;

lazy_static! {
    pub static ref BN254_XI: Fp2 = Fp2::from_u32((9, 1));
    pub static ref BN254_TWO: Fp2 = Fp2::from_u32((2, 0));
    pub static ref BN254_THREE: Fp2 = Fp2::from_u32((3, 0));
}
