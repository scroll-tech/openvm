#[cfg(all(
    any(feature = "bls12_381", feature = "halo2curves"),
    not(target_os = "zkvm")
))]
pub mod bls12_381;

#[cfg(all(
    any(feature = "bn254", feature = "halo2curves"),
    not(target_os = "zkvm")
))]
pub mod bn254;
