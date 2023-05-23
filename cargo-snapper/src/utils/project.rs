use std::{fs, path::Path};

use anyhow::Result;

pub fn create(root: &Path) -> Result<()> {
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

    fs::write(
        &root.join("Snapper.toml"),
        include_str!("../../assets/Snapper.toml"),
    )?;

    Ok(())
}
