//! IMPORTANT:
//! Temporary location for the ScriptTransaction implementation during the development
//! before we move it to the move-vm-backend-common crate.

use anyhow::{Error, Result};
use core::convert::TryFrom;
use move_core_types::language_storage::TypeTag;
use serde::{Deserialize, Serialize};

/// Transaction representation used in execute call.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptTransaction {
    /// Script bytecode.
    pub bytecode: Vec<u8>,
    /// Script args.
    pub args: Vec<Vec<u8>>,
    /// Script type arguments.
    pub type_args: Vec<TypeTag>,
}

impl TryFrom<&[u8]> for ScriptTransaction {
    type Error = Error;

    fn try_from(blob: &[u8]) -> Result<Self, Self::Error> {
        bcs::from_bytes(blob).map_err(Error::msg)
    }
}

impl ScriptTransaction {
    /// Serializes data.
    pub fn encode(self) -> Result<Vec<u8>> {
        bcs::to_bytes(&self).map_err(Error::msg)
    }
}
