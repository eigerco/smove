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
#[clap(name = "smove", author, about, long_about = None, version)]
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

    /// Create a package bundle.
    #[clap(about = "Create a package bundle")]
    Bundle {
        #[clap(flatten)]
        cmd: cmd::bundle::Bundle,
    },

    /// Create a script transaction.
    #[clap(about = "Create a script transaction")]
    CreateTransaction {
        #[clap(flatten)]
        cmd: cmd::script::CreateTransaction,
    },

    /// Commands for accessing the node.
    #[clap(about = "Commands for accessing the node")]
    #[clap(subcommand)]
    Node(cmd::node::Node),
}

/// Run the smove CLI.
pub fn smove_cli(cwd: PathBuf) -> Result<()> {
    let SmoveArgs { move_args, cmd } = SmoveArgs::parse();

    let project_root_dir = if let Some(ref project_path) = move_args.package_path {
        project_path.canonicalize()?
    } else {
        cwd
    };

    let ctx = RunContext::new(project_root_dir, move_args)?;

    match cmd {
        SmoveCommand::MoveCommand(cmd) => run_move_cli::run_command(&ctx, cmd),
        SmoveCommand::Bundle { mut cmd } => cmd.execute(&ctx),
        SmoveCommand::Node(mut cmd) => cmd.execute(),
        SmoveCommand::CreateTransaction { mut cmd } => cmd.execute(&ctx),
    }
}
