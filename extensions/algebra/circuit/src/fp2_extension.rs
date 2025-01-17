use derive_more::derive::From;
use num_bigint::BigUint;
use openvm_algebra_transpiler::Fp2Opcode;
use openvm_circuit::{
    arch::{SystemPort, VmExtension, VmInventory, VmInventoryBuilder, VmInventoryError},
    system::phantom::PhantomChip,
};
use openvm_circuit_derive::{AnyEnum, InstructionExecutor};
use openvm_circuit_primitives::bitwise_op_lookup::{
    BitwiseOperationLookupBus, SharedBitwiseOperationLookupChip,
};
use openvm_circuit_primitives_derive::{BytesStateful, Chip, ChipUsageGetter};
use openvm_instructions::{UsizeOpcode, VmOpcode};
use openvm_mod_circuit_builder::ExprBuilderConfig;
use openvm_rv32_adapters::Rv32VecHeapAdapterChip;
use openvm_stark_backend::p3_field::PrimeField32;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use strum::EnumCount;

use crate::fp2_chip::{Fp2AddSubChip, Fp2MulDivChip};

#[serde_as]
#[derive(Clone, Debug, derive_new::new, Serialize, Deserialize)]
pub struct Fp2Extension {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    pub supported_modulus: Vec<BigUint>,
}

#[derive(ChipUsageGetter, Chip, InstructionExecutor, AnyEnum, From, BytesStateful)]
pub enum Fp2ExtensionExecutor<F: PrimeField32> {
    // 32 limbs prime
    Fp2AddSubRv32_32(Fp2AddSubChip<F, 2, 32>),
    Fp2MulDivRv32_32(Fp2MulDivChip<F, 2, 32>),
    // 48 limbs prime
    Fp2AddSubRv32_48(Fp2AddSubChip<F, 6, 16>),
    Fp2MulDivRv32_48(Fp2MulDivChip<F, 6, 16>),
}

#[derive(ChipUsageGetter, Chip, AnyEnum, From, BytesStateful)]
pub enum Fp2ExtensionPeriphery<F: PrimeField32> {
    BitwiseOperationLookup(SharedBitwiseOperationLookupChip<8>),
    // We put this only to get the <F> generic to work
    Phantom(PhantomChip<F>),
}

impl<F: PrimeField32> VmExtension<F> for Fp2Extension {
    type Executor = Fp2ExtensionExecutor<F>;
    type Periphery = Fp2ExtensionPeriphery<F>;

    fn build(
        &self,
        builder: &mut VmInventoryBuilder<F>,
    ) -> Result<VmInventory<Self::Executor, Self::Periphery>, VmInventoryError> {
        let mut inventory = VmInventory::new();
        let SystemPort {
            execution_bus,
            program_bus,
            memory_bridge,
        } = builder.system_port();
        let bitwise_lu_chip = if let Some(&chip) = builder
            .find_chip::<SharedBitwiseOperationLookupChip<8>>()
            .first()
        {
            chip.clone()
        } else {
            let bitwise_lu_bus = BitwiseOperationLookupBus::new(builder.new_bus_idx());
            let chip = SharedBitwiseOperationLookupChip::new(bitwise_lu_bus);
            inventory.add_periphery_chip(chip.clone());
            chip
        };
        let offline_memory = builder.system_base().offline_memory();
        let range_checker = builder.system_base().range_checker_chip.clone();
        let address_bits = builder.system_config().memory_config.pointer_max_bits;

        let addsub_opcodes = (Fp2Opcode::ADD as usize)..=(Fp2Opcode::SETUP_ADDSUB as usize);
        let muldiv_opcodes = (Fp2Opcode::MUL as usize)..=(Fp2Opcode::SETUP_MULDIV as usize);

        for (i, modulus) in self.supported_modulus.iter().enumerate() {
            // determine the number of bytes needed to represent a prime field element
            let bytes = modulus.bits().div_ceil(8);
            let class_offset = Fp2Opcode::default_offset() + i * Fp2Opcode::COUNT;

            let config32 = ExprBuilderConfig {
                modulus: modulus.clone(),
                num_limbs: 32,
                limb_bits: 8,
            };
            let config48 = ExprBuilderConfig {
                modulus: modulus.clone(),
                num_limbs: 48,
                limb_bits: 8,
            };
            let adapter_chip_32 = Rv32VecHeapAdapterChip::new(
                execution_bus,
                program_bus,
                memory_bridge,
                address_bits,
                bitwise_lu_chip.clone(),
            );
            let adapter_chip_48 = Rv32VecHeapAdapterChip::new(
                execution_bus,
                program_bus,
                memory_bridge,
                address_bits,
                bitwise_lu_chip.clone(),
            );

            if bytes <= 32 {
                let addsub_chip = Fp2AddSubChip::new(
                    adapter_chip_32.clone(),
                    config32.clone(),
                    class_offset,
                    range_checker.clone(),
                    offline_memory.clone(),
                );
                inventory.add_executor(
                    Fp2ExtensionExecutor::Fp2AddSubRv32_32(addsub_chip),
                    addsub_opcodes
                        .clone()
                        .map(|x| VmOpcode::from_usize(x + class_offset)),
                )?;
                let muldiv_chip = Fp2MulDivChip::new(
                    adapter_chip_32.clone(),
                    config32.clone(),
                    class_offset,
                    range_checker.clone(),
                    offline_memory.clone(),
                );
                inventory.add_executor(
                    Fp2ExtensionExecutor::Fp2MulDivRv32_32(muldiv_chip),
                    muldiv_opcodes
                        .clone()
                        .map(|x| VmOpcode::from_usize(x + class_offset)),
                )?;
            } else if bytes <= 48 {
                let addsub_chip = Fp2AddSubChip::new(
                    adapter_chip_48.clone(),
                    config48.clone(),
                    class_offset,
                    range_checker.clone(),
                    offline_memory.clone(),
                );
                inventory.add_executor(
                    Fp2ExtensionExecutor::Fp2AddSubRv32_48(addsub_chip),
                    addsub_opcodes
                        .clone()
                        .map(|x| VmOpcode::from_usize(x + class_offset)),
                )?;
                let muldiv_chip = Fp2MulDivChip::new(
                    adapter_chip_48.clone(),
                    config48.clone(),
                    class_offset,
                    range_checker.clone(),
                    offline_memory.clone(),
                );
                inventory.add_executor(
                    Fp2ExtensionExecutor::Fp2MulDivRv32_48(muldiv_chip),
                    muldiv_opcodes
                        .clone()
                        .map(|x| VmOpcode::from_usize(x + class_offset)),
                )?;
            } else {
                panic!("Modulus too large");
            }
        }

        Ok(inventory)
    }
}
