use clap::Parser;
use move_cli::Move;
use move_core_types::{errmap::ErrorMapping, language_storage::CORE_CODE_ADDRESS};
use move_stdlib::natives::{all_natives, GasParameters};

/// CLI frontend for the Move compiler and VM in Substrate
#[derive(Parser)]
#[clap(name = "smove", author, about, long_about = None)]
pub struct SmoveArgs {
    /// Native move-cli arguments
    #[clap(flatten)]
    pub move_args: Move,

    /// Commands
    #[clap(subcommand)]
    pub cmd: SmoveCommand,
}

/// Move CLI and smove subcommands
#[derive(clap::Subcommand)]
pub enum SmoveCommand {
    /// Native move-cli commands
    #[clap(flatten)]
    MoveCommand(move_cli::Command),

    /// Create package bundle
    #[clap(about = "Create package bundle")]
    Package {},

    /// Run the script and create a transcation for the pallet
    #[clap(about = "Run the script and create a transcation for the pallet")]
    Run {},
}

fn main() -> anyhow::Result<()> {
    let SmoveArgs { move_args, cmd } = SmoveArgs::parse();

    let error_descriptions: ErrorMapping = bcs::from_bytes(move_stdlib::doc::error_descriptions())?;
    // TODO(eiger): Use custom cost table
    let cost_table = &move_vm_test_utils::gas_schedule::INITIAL_COST_SCHEDULE;
    let natives = all_natives(CORE_CODE_ADDRESS, GasParameters::zeros());

    match cmd {
        SmoveCommand::MoveCommand(cmd) => {
            move_cli::run_cli(natives, cost_table, &error_descriptions, move_args, cmd)
        }
        SmoveCommand::Package {} => unimplemented!(),
        SmoveCommand::Run {} => unimplemented!(),
    }
}
