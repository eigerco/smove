use crate::run_context::RunContext;
use anyhow::Result;
use clap::Parser;
use move_cli::Move;
use std::path::PathBuf;

mod cmd;
mod run_context;
mod run_move_cli;

/// CLI frontend for the Move compiler and VM in Substrate.
#[derive(Parser)]
#[clap(name = "smove", author, about, long_about = None)]
struct SmoveArgs {
    /// Native move-cli arguments.
    #[clap(flatten)]
    move_args: Move,

    /// Commands.
    #[clap(subcommand)]
    cmd: SmoveCommand,
}

/// Move CLI and smove subcommands.
#[derive(clap::Subcommand)]
enum SmoveCommand {
    /// Native move-cli commands.
    #[clap(flatten)]
    MoveCommand(move_cli::Command),

    /// Create package bundle.
    #[clap(about = "Create package bundle")]
    Bundle {
        #[clap(flatten)]
        cmd: cmd::bundle::Bundle,
    },

    /// Run the script and create a transcation for the pallet.
    #[clap(about = "Run the script and create a transcation for the pallet")]
    Run {},
}

/// Run the smove CLI.
pub fn smove_cli(cwd: PathBuf) -> Result<()> {
    let SmoveArgs { move_args, cmd } = SmoveArgs::parse();
    let ctx = RunContext::new(cwd, move_args)?;

    match cmd {
        SmoveCommand::MoveCommand(cmd) => run_move_cli::run_command(&ctx, cmd),
        SmoveCommand::Bundle { mut cmd } => cmd.run(&ctx),
        SmoveCommand::Run {} => unimplemented!(),
    }
}
