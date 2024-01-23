use clap::Parser;
use move_core_types::language_storage::TypeTag;
use type_args::TypeArgVec;

mod type_args;

/// Arguments for script functions.
#[derive(Debug, Parser)]
pub struct ScriptFunctionArguments {
    /// Type args.
    #[clap(flatten)]
    type_arg_vec: TypeArgVec,
    //TODO(rqnsom): impl below in the next PR
    //#[clap(flatten)]
    //arg_vec: ArgWithTypeVec,
}

impl ScriptFunctionArguments {
    /// Get type args.
    pub fn type_args(&self) -> anyhow::Result<Vec<TypeTag>> {
        let mut type_args = vec![];

        for arg in self.type_arg_vec.type_args.iter() {
            let type_arg: TypeTag = arg.try_into()?;
            type_args.push(type_arg);
        }

        Ok(type_args)
    }
}
