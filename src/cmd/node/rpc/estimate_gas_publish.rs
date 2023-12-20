use anyhow::{Context, Result};
use clap::Parser;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use move_core_types::vm_status::StatusCode;
use serde::Deserialize;
use std::fmt;
use std::path::{Path, PathBuf};

/// Estimate gas for publishing modules.
#[derive(Parser, Debug)]
#[clap(about = "Estimate gas for publishing modules")]
pub struct EstimateGasPublishModule {
    #[clap(short, long, help = "Account ID in the SS58 format")]
    account_id: String,
    #[clap(short, long, help = "Path to the module (compiled by the smove)")]
    module_path: PathBuf,
}

impl EstimateGasPublishModule {
    /// Executes the command.
    pub fn execute(&mut self) -> Result<()> {
        // TODO: provide the rpc_url via the argument
        let rpc_url = "http://localhost:9944/";

        let script_tx = read_bytes(&self.module_path)?;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = HttpClientBuilder::default().build(rpc_url)?;
        let params = rpc_params![&self.account_id, script_tx];
        let response: Result<Estimation, _> =
            rt.block_on(async { client.request("mvm_estimateGasPublishModule", params).await });

        let estimated_gas = response.with_context(|| "RPC result failure")?;

        println!("Estimated gas: {estimated_gas}");

        Ok(())
    }
}

/// Estimate gas for publishing a bunlde.
#[derive(Parser, Debug)]
#[clap(about = "Estimate gas for publishing a bundle")]
pub struct EstimateGasPublishBundle {
    #[clap(short, long, help = "Account ID in the SS58 format")]
    account_id: String,
    #[clap(short, long, help = "Path to the bundle (compiled by the smove)")]
    bundle_path: PathBuf,
}

impl EstimateGasPublishBundle {
    /// Executes the command.
    pub fn execute(&mut self) -> Result<()> {
        // TODO: provide the rpc_url via the argument
        let rpc_url = "http://localhost:9944/";

        let script_tx = read_bytes(&self.bundle_path)?;

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = HttpClientBuilder::default().build(rpc_url)?;
        let params = rpc_params![&self.account_id, script_tx];
        let response: Result<Estimation, _> =
            rt.block_on(async { client.request("mvm_estimateGasPublishBundle", params).await });

        let estimated_gas = response.with_context(|| "RPC result failure")?;

        println!("Estimated gas: {estimated_gas}");

        Ok(())
    }
}

/// Gas estimation information.
#[derive(Deserialize)]
struct Estimation {
    /// Gas used.
    gas_used: u64,
    /// Status code for the MoveVM execution.
    vm_status_code: StatusCode,
}

impl fmt::Display for Estimation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let gas_used = if matches!(self.vm_status_code, StatusCode::EXECUTED) {
            self.gas_used
        } else {
            0
        };

        write!(
            f,
            "Estimate (gas_used: {}, vm_status_code: {:?})",
            gas_used, self.vm_status_code
        )
    }
}

/// Reads bytes from a file for the given path.
fn read_bytes(file_path: &Path) -> Result<Vec<u8>> {
    std::fs::read(file_path)
        .map_err(anyhow::Error::from)
        .with_context(|| format!("Failure to read filename {}", file_path.display()))
}
