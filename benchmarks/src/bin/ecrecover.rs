use clap::Parser;
use derive_more::derive::From;
use eyre::Result;
use k256::ecdsa::{SigningKey, VerifyingKey};
use num_bigint::BigUint;
use openvm_algebra_circuit::{
    ModularExtension, ModularExtensionExecutor, ModularExtensionPeriphery,
};
use openvm_algebra_transpiler::ModularTranspilerExtension;
use openvm_benchmarks::utils::BenchmarkCli;
use openvm_circuit::{
    arch::{
        instructions::exe::VmExe, SystemConfig, SystemExecutor, SystemPeriphery, VmChipComplex,
        VmConfig, VmInventoryError,
    },
    circuit_derive::{Chip, ChipUsageGetter},
    derive::{AnyEnum, InstructionExecutor, VmConfig},
};
use openvm_ecc_circuit::{
    CurveConfig, WeierstrassExtension, WeierstrassExtensionExecutor, WeierstrassExtensionPeriphery,
    SECP256K1_CONFIG,
};
use openvm_ecc_transpiler::EccTranspilerExtension;
use openvm_keccak256_circuit::{Keccak256, Keccak256Executor, Keccak256Periphery};
use openvm_keccak256_transpiler::Keccak256TranspilerExtension;
use openvm_rv32im_circuit::{
    Rv32I, Rv32IExecutor, Rv32IPeriphery, Rv32Io, Rv32IoExecutor, Rv32IoPeriphery, Rv32M,
    Rv32MExecutor, Rv32MPeriphery,
};
use openvm_rv32im_transpiler::{
    Rv32ITranspilerExtension, Rv32IoTranspilerExtension, Rv32MTranspilerExtension,
};
use openvm_stark_backend::p3_field::{FieldAlgebra, PrimeField32};
use openvm_stark_sdk::{bench::run_with_metric_collection, p3_baby_bear::BabyBear};
use openvm_transpiler::{transpiler::Transpiler, FromElf};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

fn make_input(signing_key: &SigningKey, msg: &[u8]) -> Vec<BabyBear> {
    let mut hasher = Keccak::v256();
    hasher.update(msg);
    let mut prehash = [0u8; 32];
    hasher.finalize(&mut prehash);
    let (signature, recid) = signing_key.sign_prehash_recoverable(&prehash).unwrap();
    // Input format: https://www.evm.codes/precompiled?fork=cancun#0x01
    let mut input = prehash.to_vec();
    let v = recid.to_byte() + 27u8;
    input.extend_from_slice(&[0; 31]);
    input.push(v);
    input.extend_from_slice(signature.to_bytes().as_ref());

    input.into_iter().map(BabyBear::from_canonical_u8).collect()
}

#[derive(Clone, Debug, VmConfig, derive_new::new, Serialize, Deserialize)]
pub struct Rv32ImEcRecoverConfig {
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
    pub keccak: Keccak256,
    #[extension]
    pub weierstrass: WeierstrassExtension,
}

impl Rv32ImEcRecoverConfig {
    pub fn for_curves(curves: Vec<CurveConfig>) -> Self {
        let primes: Vec<BigUint> = curves
            .iter()
            .flat_map(|c| [c.modulus.clone(), c.scalar.clone()])
            .collect();
        Self {
            system: SystemConfig::default().with_continuations(),
            base: Default::default(),
            mul: Default::default(),
            io: Default::default(),
            modular: ModularExtension::new(primes),
            keccak: Default::default(),
            weierstrass: WeierstrassExtension::new(curves),
        }
    }
}

fn main() -> Result<()> {
    let args = BenchmarkCli::parse();

    let elf = args.build_bench_program("ecrecover")?;
    // TODO: update sw_setup macros and read it from elf.
    let exe = VmExe::from_elf(
        elf,
        Transpiler::<BabyBear>::default()
            .with_extension(Rv32ITranspilerExtension)
            .with_extension(Rv32MTranspilerExtension)
            .with_extension(Rv32IoTranspilerExtension)
            .with_extension(Keccak256TranspilerExtension)
            .with_extension(ModularTranspilerExtension)
            .with_extension(EccTranspilerExtension),
    )?;

    run_with_metric_collection("OUTPUT_PATH", || -> Result<()> {
        let mut rng = ChaCha8Rng::seed_from_u64(12345);
        let signing_key: SigningKey = SigningKey::random(&mut rng);
        let verifying_key = VerifyingKey::from(&signing_key);
        let mut hasher = Keccak::v256();
        let mut expected_address = [0u8; 32];
        hasher.update(
            &verifying_key
                .to_encoded_point(/* compress = */ false)
                .as_bytes()[1..],
        );
        hasher.finalize(&mut expected_address);
        expected_address[..12].fill(0); // 20 bytes as the address.
        let mut input_stream = vec![expected_address
            .into_iter()
            .map(BabyBear::from_canonical_u8)
            .collect::<Vec<_>>()];

        let msg = ["Elliptic", "Curve", "Digital", "Signature", "Algorithm"];
        input_stream.extend(
            msg.iter()
                .map(|s| make_input(&signing_key, s.as_bytes()))
                .collect::<Vec<_>>(),
        );
        args.bench_from_exe(
            "ecrecover_program",
            Rv32ImEcRecoverConfig::for_curves(vec![SECP256K1_CONFIG.clone()]),
            exe,
            input_stream.into(),
        )
    })
}
