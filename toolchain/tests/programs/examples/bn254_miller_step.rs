#![cfg_attr(target_os = "zkvm", no_main)]
#![cfg_attr(not(feature = "std"), no_std)]

axvm::entry!(main);

pub fn main() {
    let io = read_vec();
}
