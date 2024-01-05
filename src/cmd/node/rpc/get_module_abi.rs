use anyhow::{Context, Result};
use clap::Parser;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;

/// Estimate gas for publishing modules.
#[derive(Parser, Debug)]
#[clap(about = "Get a move module's ABI")]
pub struct GetModuleAbi {
    #[clap(short, long, help = "Address of the module")]
    address: String,
    #[clap(short, long, help = "Name of the module")]
    name: String,
}

impl GetModuleAbi {
    /// Executes the command.
    pub fn execute(&mut self) -> Result<()> {
        // TODO: provide the rpc_url via the argument
        let rpc_url = "http://localhost:9944/";

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let client = HttpClientBuilder::default().build(rpc_url)?;
        let params = rpc_params![&self.address, &self.name];
        let response: Result<Option<Vec<u8>>, _> =
            rt.block_on(async { client.request("mvm_getModuleABI", params).await });

        let module_abi = response.with_context(|| "RPC result failure")?;

        println!("Module ABI: {module_abi:?}");

        Ok(())
    }
}
