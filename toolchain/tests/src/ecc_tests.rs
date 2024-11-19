use axvm_circuit::arch::{VmConfig, VmExecutor};
use eyre::Result;
use p3_baby_bear::BabyBear;

use crate::utils::build_example_program;

type F = BabyBear;

#[test]
fn test_moduli_setup_runtime() -> Result<()> {
    let elf = build_example_program("moduli_setup")?;
    let exe = axvm_circuit::arch::instructions::exe::AxVmExe::<F>::from(elf.clone());
    let executor =
        VmExecutor::<F>::new(VmConfig::rv32im().add_modular_support(exe.custom_op_config.primes()));
    executor.execute(elf, vec![])?;
    assert!(!executor.config.supported_modulus.is_empty());
    Ok(())
}

#[test]
fn test_modular_runtime() -> Result<()> {
    let elf = build_example_program("little")?;
    let exe = axvm_circuit::arch::instructions::exe::AxVmExe::<F>::from(elf.clone());
    let executor =
        VmExecutor::<F>::new(VmConfig::rv32im().add_modular_support(exe.custom_op_config.primes()));
    executor.execute(elf, vec![])?;
    Ok(())
}

#[test]
fn test_complex_runtime() -> Result<()> {
    let elf = build_example_program("complex")?;
    let exe = axvm_circuit::arch::instructions::exe::AxVmExe::<F>::from(elf.clone());
    println!("num moduli: {}", exe.custom_op_config.primes().len());
    let executor = VmExecutor::<F>::new(
        VmConfig::rv32im()
            .add_modular_support(exe.custom_op_config.primes())
            .add_complex_ext_support(exe.custom_op_config.primes()),
    );
    executor.execute(elf, vec![])?;
    Ok(())
}

#[test]
fn test_ec_runtime() -> Result<()> {
    let elf = build_example_program("ec")?;
    let exe = axvm_circuit::arch::instructions::exe::AxVmExe::<F>::from(elf.clone());
    let executor = VmExecutor::<F>::new(
        VmConfig::rv32im()
            .add_modular_support(exe.custom_op_config.primes())
            .add_canonical_ec_curves(), // TODO: sw_setup should pass the curves and we can read them from there
    );
    executor.execute(elf, vec![])?;
    Ok(())
}
