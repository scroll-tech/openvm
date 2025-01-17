use derive_more::derive::From;
use openvm_algebra_circuit::*;
use openvm_circuit::arch::{
    SystemConfig, SystemExecutor, SystemPeriphery, VmChipComplex, VmConfig, VmInventoryError,
};
use openvm_circuit_derive::{AnyEnum, InstructionExecutor, VmConfig};
use openvm_circuit_primitives_derive::{Chip, ChipUsageGetter};
use openvm_rv32im_circuit::*;
use openvm_stark_backend::p3_field::PrimeField32;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Clone, Debug, VmConfig, Serialize, Deserialize)]
pub struct Rv32WeierstrassConfig {
    #[system]
    pub system: SystemConfig,
    #[extension]
    pub base: Rv32I,
    #[extension]
    pub mul: Rv32M,
    #[extension]
    pub io: Rv32Io,
    #[extension]
    pub modular: ModularExtension,
    #[extension]
    pub weierstrass: WeierstrassExtension,
}

impl Rv32WeierstrassConfig {
    pub fn new(curves: Vec<CurveConfig>) -> Self {
        let primes: Vec<_> = curves
            .iter()
            .flat_map(|c| [c.modulus.clone(), c.scalar.clone()])
            .collect();
        Self {
            system: SystemConfig::default().with_continuations(),
            base: Default::default(),
            mul: Default::default(),
            io: Default::default(),
            modular: ModularExtension::new(primes),
            weierstrass: WeierstrassExtension::new(curves),
        }
    }
}
