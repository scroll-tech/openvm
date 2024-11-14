#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

use axvm::io::read_vec;
use axvm_ecc::miller_loop;

axvm::entry!(main);

pub fn main() {
    let actual = bls12_381_final_exp_hint(f);
    assert_eq!(&actual, expected);
}
