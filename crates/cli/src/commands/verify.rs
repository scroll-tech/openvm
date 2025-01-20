use std::path::PathBuf;

use clap::Parser;
use eyre::{eyre, Result};
use openvm_circuit::{arch::SingleSegmentVmExecutor, system::program::trace::VmCommittedExe};
use openvm_native_recursion::hints::Hintable;
use openvm_sdk::{
    fs::{
        read_app_proof_from_file, read_app_vk_from_file, read_evm_proof_from_file,
        read_evm_verifier_from_file, read_root_pk_from_file, read_root_proof_from_file,
    },
    Sdk,
};

use crate::default::{
    DEFAULT_APP_PROOF_PATH, DEFAULT_APP_VK_PATH, DEFAULT_EVM_PROOF_PATH, DEFAULT_ROOT_PK_PATH,
    DEFAULT_ROOT_PROOF_PATH, DEFAULT_VERIFIER_PATH,
};

#[derive(Parser)]
#[command(name = "verify", about = "Verify a proof")]
pub struct VerifyCmd {
    #[clap(subcommand)]
    command: VerifySubCommand,
}

#[derive(Parser)]
enum VerifySubCommand {
    App {
        #[clap(long, action, help = "Path to app verifying key", default_value = DEFAULT_APP_VK_PATH)]
        app_vk: PathBuf,

        #[clap(long, action, help = "Path to app proof", default_value = DEFAULT_APP_PROOF_PATH)]
        proof: PathBuf,
    },
    Root {
        #[clap(long, action, help = "Path to root proof", default_value = DEFAULT_ROOT_PROOF_PATH)]
        proof: PathBuf,
    },
    Evm {
        #[clap(long, action, help = "Path to EVM proof", default_value = DEFAULT_EVM_PROOF_PATH)]
        proof: PathBuf,
    },
}

impl VerifyCmd {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            VerifySubCommand::App { app_vk, proof } => {
                let app_vk = read_app_vk_from_file(app_vk)?;
                let app_proof = read_app_proof_from_file(proof)?;
                Sdk.verify_app_proof(&app_vk, &app_proof)?;
            }
            VerifySubCommand::Root { proof } => {
                let root_proof = read_root_proof_from_file(proof)?;
                let input = root_proof.write();
                let agg_stark_pk = read_root_pk_from_file(DEFAULT_ROOT_PK_PATH).map_err(|e| {
                    eyre::eyre!("Failed to read root aggregation proving key: {}\nPlease run 'cargo openvm setup' first", e)
                })?;
                let root_verifier_pk = agg_stark_pk.root_verifier_pk;
                //let config: openvm_native_circuit::NativeConfig =  root_verifier_pk.vm_pk.vm_config.clone();
                let vm = SingleSegmentVmExecutor::new(root_verifier_pk.vm_pk.vm_config.clone());
                let exe: &VmCommittedExe<_> = &root_verifier_pk.root_committed_exe;
                let result = vm.execute_and_compute_heights(exe.exe.clone(), input)?;
                println!("root program execution result: {:?}", result);
            }
            VerifySubCommand::Evm { proof } => {
                let evm_verifier = read_evm_verifier_from_file(DEFAULT_VERIFIER_PATH).map_err(|e| {
                    eyre::eyre!("Failed to read EVM verifier: {}\nPlease run 'cargo openvm evm-proving-setup' first", e)
                })?;
                let evm_proof = read_evm_proof_from_file(proof)?;
                if !Sdk.verify_evm_proof(&evm_verifier, &evm_proof) {
                    return Err(eyre!("EVM proof verification failed"));
                }
            }
        }
        Ok(())
    }
}
