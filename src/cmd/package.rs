use anyhow::Result;
use clap::Parser;

use crate::run_context::RunContext;
use crate::run_move_cli;

/// Package command bundles modules into packages.
#[derive(Parser, Debug)]
#[clap(about = "smove package")]
pub struct Package {}

impl Package {
    /// Executes the command.
    pub fn run(&mut self, ctx: &RunContext) -> Result<()> {
        // Build all move modules
        run_move_cli::execute_build(ctx)?;

        // TODO(Rqnsom): Impl module filtering and actual bundling
        Ok(())
    }
}
