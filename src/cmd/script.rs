use super::script_args::ScriptFunctionArguments;
use super::script_transaction::ScriptTransaction;
use crate::run_context::RunContext;
use anyhow::{Error, Result};
use clap::Parser;
use move_binary_format::file_format::CompiledScript;
use std::fs;
use std::path::PathBuf;

/// Create a script transaction.
#[derive(Parser, Debug)]
#[clap(about = "smove create-transaction")]
pub struct CreateTransaction {
    #[clap(short, long, help = "Path for the compiled Move script.")]
    compiled_script_path: PathBuf,

    /// Arguments for script functions.
    #[clap(flatten)]
    script_function_args: ScriptFunctionArguments,
}

impl CreateTransaction {
    /// Executes the command.
    pub fn execute(&mut self, ctx: &RunContext) -> Result<()> {
        let compiled_script = &self.compiled_script_path;
        let script_bc = fs::read(compiled_script)
            .map_err(|e| Error::msg(format!("Can't read '{}':\n{e}", compiled_script.display())))?;

        // Check the script bytecode and verify the parameter rules.
        verify_script_integrity(&script_bc)?;

        let type_args = self.script_function_args.type_args()?;
        let args = self.script_function_args.args()?;

        // TODO(Rqnsom): maybe use a function to create this:
        let tx = ScriptTransaction {
            bytecode: script_bc,
            args,
            type_args,
        };

        // Path to the output file.
        let tx_name = compiled_script.file_name().unwrap(); // this can't fail in case fs::read succeds above.
        let output_file_path = ctx.script_tx_output_path(&tx_name)?;
        if output_file_path.exists() {
            fs::remove_file(&output_file_path)?;
        }
        fs::write(&output_file_path, tx.encode()?)?;

        println!(
            "Script transaction is created at:\n{}",
            output_file_path.canonicalize()?.display()
        );

        Ok(())
    }
}

// Check script integrity and ensure the parameters are correct (signers should always come first).
fn verify_script_integrity(bytecode: &[u8]) -> Result<()> {
    let _compiled_script = CompiledScript::deserialize(bytecode)
        .map_err(|e| Error::msg(format!("Cannot deserialize the Move script:\n{e}")))?;

    // TODO(Rqnsom): verify signer rule in the parameter list after the same check is implemented
    // in the substrate layer.

    Ok(())
}
