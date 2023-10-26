use anyhow::{anyhow, Result};
use move_cli::Move as MoveCliArgs;
use move_core_types::errmap::ErrorMapping;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_package::source_package::parsed_manifest::SourceManifest;
use move_package::source_package::{layout, manifest_parser};
use move_stdlib::natives::{all_natives, GasParameters};
use move_vm_runtime::native_functions::NativeFunctionTable;
use move_vm_test_utils::gas_schedule::CostTable;
use std::fs::read_to_string;
use std::path::PathBuf;

/// Move compilation related data.
pub struct RunContext {
    /// Project directory.
    pub project_root_dir: PathBuf,
    /// `move_cli` arguments.
    pub move_args: MoveCliArgs,
    /// `Move.toml` contents.
    pub manifest: SourceManifest,
    /// Error descriptions.
    pub error_descriptions: ErrorMapping,
    /// Native functions.
    pub natives: NativeFunctionTable,
    /// Cost table.
    pub cost_table: CostTable,
}

impl RunContext {
    /// Create a new instance.
    pub fn new(project_root_dir: PathBuf, move_args: MoveCliArgs) -> Result<Self> {
        // TODO(eiger): Use a custom cost table
        let cost_table = move_vm_test_utils::gas_schedule::INITIAL_COST_SCHEDULE.clone();
        let natives = all_natives(CORE_CODE_ADDRESS, GasParameters::zeros());
        let error_descriptions = bcs::from_bytes(move_stdlib::doc::error_descriptions())?;

        let manifest_string =
            read_to_string(project_root_dir.join(layout::SourcePackageLayout::Manifest.path()))
                .map_err(|_| anyhow!("Move.toml not found. Path: {:?}", &project_root_dir))?;
        let toml_manifest = manifest_parser::parse_move_manifest_string(manifest_string)?;
        let manifest = manifest_parser::parse_source_manifest(toml_manifest)?;

        Ok(Self {
            project_root_dir,
            move_args,
            manifest,
            error_descriptions,
            natives,
            cost_table,
        })
    }
}
