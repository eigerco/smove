use anyhow::Result;
use clap::Parser;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use serde::Deserialize;
use std::fmt;

/// Estimate gas for publishing modules.
#[derive(Parser, Debug)]
#[clap(about = "Convert gas to weight")]
pub struct GasToWeight {
    #[clap(short, long, help = "Gas should be an u64 value")]
    gas: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Weight {
    /// The weight of computational time used based on some reference hardware.
    ref_time: u64,
    /// The weight of storage space used by proof of validity.
    proof_size: u64,
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Weight (ref_time: {}, proof_size: {})",
            self.ref_time, self.proof_size
        )
    }
}

impl GasToWeight {
    /// Executes the command.
    pub fn execute(&mut self) -> Result<()> {
        // TODO: provide the rpc_url via the argument
        let rpc_url = "http://localhost:9944/";

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = HttpClientBuilder::default().build(rpc_url)?;
        let params = rpc_params![self.gas];
        let response: Result<Weight, _> =
            rt.block_on(async { client.request("mvm_gasToWeight", params).await });

        let converted_weight = response?;

        println!(
            "Value of {} gas converted to weight has a value of {converted_weight}",
            self.gas
        );

        Ok(())
    }
}
