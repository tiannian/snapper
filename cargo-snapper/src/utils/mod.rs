use std::{fs, path::Path};

use anyhow::Result;
use cargo_metadata::MetadataCommand;

pub fn create_workdir() -> Result<()> {
    let metadata = MetadataCommand::new().no_deps().exec()?;

    let root = metadata.workspace_root.into_std_path_buf();

    fs::create_dir_all(root.join("contracts"))?;

    fs::create_dir_all(root.join("scripts"))?;
    fs::create_dir_all(root.join("tests"))?;

    Ok(())
}
