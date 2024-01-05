use anyhow::Result;

mod rpc;

/// Commands for accessing the node.
#[derive(clap::Subcommand)]
pub enum Node {
    #[clap(subcommand, about = "Access node's RPC requests")]
    Rpc(Rpc),
}

impl Node {
    /// Executes the command.
    pub fn execute(&mut self) -> Result<()> {
        match self {
            Self::Rpc(cmd) => cmd.execute(),
        }
    }
}

#[derive(clap::Subcommand)]
pub enum Rpc {
    /// Estimate gas for publishing modules.
    #[clap(about = "Estimate gas for publishing modules")]
    EstimateGasPublishModule {
        #[clap(flatten)]
        cmd: rpc::estimate_gas_publish::EstimateGasPublishModule,
    },

    /// Estimate gas for publishing a bundle.
    #[clap(about = "Estimate gas for publishing a bundle")]
    EstimateGasPublishBundle {
        #[clap(flatten)]
        cmd: rpc::estimate_gas_publish::EstimateGasPublishBundle,
    },

    /// Convert gas to weight.
    #[clap(about = "Convert gas to weight")]
    GasToWeight {
        #[clap(flatten)]
        cmd: rpc::gas_to_weight::GasToWeight,
    },

    /// Get a module's ABI.
    #[clap(about = "Convert gas to weight")]
    GetModuleAbi {
        #[clap(flatten)]
        cmd: rpc::get_module_abi::GetModuleAbi,
    },
}

impl Rpc {
    /// Executes the command.
    pub fn execute(&mut self) -> Result<()> {
        match self {
            Self::EstimateGasPublishModule { cmd } => cmd.execute(),
            Self::EstimateGasPublishBundle { cmd } => cmd.execute(),
            Self::GasToWeight { cmd } => cmd.execute(),
            Self::GetModuleAbi { cmd } => cmd.execute(),
        }
    }
}
