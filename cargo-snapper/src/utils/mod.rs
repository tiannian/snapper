use std::fs;

use anyhow::Result;
use cargo_metadata::MetadataCommand;

pub fn create_workdir(dir: &str) -> Result<()> {
    let metadata = MetadataCommand::new().current_dir(dir).no_deps().exec()?;

    let root = metadata.workspace_root.into_std_path_buf();

    // --------
    let contracts = root.join("contracts");
    fs::create_dir_all(&contracts)?;
    fs::write(
        contracts.join("Locker.sol"),
        include_str!("../../assets/Locker.sol"),
    )?;

    // ---------
    let scripts = root.join("scripts");
    fs::create_dir_all(&scripts)?;
    fs::write(
        scripts.join("deploy.rs"),
        include_str!("../../assets/deploy.rs"),
    )?;

    // ---------
    let tests = root.join("tests");
    fs::create_dir_all(&tests)?;
    fs::write(
        &tests.join("locker.rs"),
        include_str!("../../assets/locker.rs"),
    )?;

    // ---------
    fs::write(
        &root.join("build.rs"),
        include_str!("../../assets/build.rs"),
    )?;

    Ok(())
}
