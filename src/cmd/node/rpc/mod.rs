//! List of RPC commands.

pub(super) mod estimate_gas_execute;
pub(super) mod estimate_gas_publish;
pub(super) mod gas_to_weight;
pub(super) mod get_module_abi;

use move_core_types::vm_status::StatusCode;
use serde::Deserialize;
use std::fmt;

/// Gas estimation information.
#[derive(Deserialize)]
struct Estimation {
    /// Gas used.
    gas_used: u64,
    /// Status code for the MoveVM execution.
    vm_status_code: StatusCode,
}

impl fmt::Display for Estimation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let gas_used = if matches!(self.vm_status_code, StatusCode::EXECUTED) {
            self.gas_used
        } else {
            0
        };

        write!(
            f,
            "Estimate (gas_used: {gas_used}, vm_status_code: {:?})",
            self.vm_status_code
        )
    }
}
