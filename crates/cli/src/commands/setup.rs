use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};

use aws_config::{defaults, BehaviorVersion, Region};
use aws_sdk_s3::Client;
use clap::Parser;
use eyre::{eyre, Ok, Result};
use openvm_native_recursion::halo2::utils::CacheHalo2ParamsReader;
use openvm_sdk::{
    config::AggConfig,
    fs::{write_agg_pk_to_file, write_evm_verifier_to_file, write_root_pk_to_file},
    keygen::AggStarkProvingKey,
    Sdk,
};

use crate::default::{
    DEFAULT_AGG_PK_PATH, DEFAULT_PARAMS_DIR, DEFAULT_ROOT_PK_PATH, DEFAULT_VERIFIER_PATH,
};

#[derive(Parser)]
#[command(
    name = "evm-proving-setup",
    about = "Set up for generating EVM proofs. ATTENTION: this requires large amounts of computation and memory. "
)]
pub struct EvmProvingSetupCmd {
    #[arg(long, default_value = "false", help = "Only generate root proving key")]
    pub stark_only: bool,
}

impl EvmProvingSetupCmd {
    pub async fn run(&self) -> Result<()> {
        if self.stark_only {
            let agg_config = AggConfig::default();
            let (agg_stark_pk, _) =
                AggStarkProvingKey::dummy_proof_and_keygen(agg_config.agg_stark_config);
            println!("Writing stark proving key to file...");
            write_root_pk_to_file(agg_stark_pk, DEFAULT_ROOT_PK_PATH)?;
            return Ok(());
        }
        if PathBuf::from(DEFAULT_AGG_PK_PATH).exists()
            && PathBuf::from(DEFAULT_VERIFIER_PATH).exists()
        {
            println!("Aggregation proving key and verifier contract already exist");
            return Ok(());
        } else if !Self::check_solc_installed() {
            return Err(eyre!(
                "solc is not installed, please install solc to continue"
            ));
        }

        Self::download_params(10, 24).await?;
        let params_reader = CacheHalo2ParamsReader::new(DEFAULT_PARAMS_DIR);
        let agg_config = AggConfig::default();

        println!("Generating proving key...");
        let agg_pk = Sdk.agg_keygen(agg_config, &params_reader)?;

        println!("Generating verifier contract...");
        let verifier = Sdk.generate_snark_verifier_contract(&params_reader, &agg_pk)?;

        println!("Writing stark proving key to file...");
        write_root_pk_to_file(agg_pk.agg_stark_pk.clone(), DEFAULT_ROOT_PK_PATH)?;

        println!("Writing proving key to file...");
        write_agg_pk_to_file(agg_pk, DEFAULT_AGG_PK_PATH)?;

        println!("Writing verifier contract to file...");
        write_evm_verifier_to_file(verifier, DEFAULT_VERIFIER_PATH)?;

        Ok(())
    }

    fn check_solc_installed() -> bool {
        std::process::Command::new("solc")
            .arg("--version")
            .output()
            .is_ok()
    }

    async fn download_params(min_k: u32, max_k: u32) -> Result<()> {
        create_dir_all(DEFAULT_PARAMS_DIR)?;
        let config = defaults(BehaviorVersion::latest())
            .region(Region::new("us-east-1"))
            .no_credentials()
            .load()
            .await;
        let client = Client::new(&config);

        for k in min_k..=max_k {
            let file_name = format!("kzg_bn254_{}.srs", k);
            let local_file_path = PathBuf::from(DEFAULT_PARAMS_DIR).join(&file_name);
            if !local_file_path.exists() {
                println!("Downloading {}", file_name);
                let key = format!("challenge_0085/{}", file_name);
                let resp = client
                    .get_object()
                    .bucket("axiom-crypto")
                    .key(&key)
                    .send()
                    .await?;
                let data = resp.body.collect().await?;
                write(local_file_path, data.into_bytes())?;
            }
        }

        Ok(())
    }
}
