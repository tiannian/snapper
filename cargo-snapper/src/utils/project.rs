use std::{fs, path::Path};

use anyhow::Result;
use toml_edit::{table, value, Array, Document};

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
        tests.join("locker.rs"),
        include_str!("../../assets/locker.rs"),
    )?;

    // ---------
    fs::write(root.join("build.rs"), include_str!("../../assets/build.rs"))?;

    fs::write(
        root.join("Snapper.toml"),
        include_str!("../../assets/Snapper.toml"),
    )?;

    let cts_path = root.join("Cargo.toml");

    let cts = fs::read_to_string(&cts_path)?;
    let mut cargo_toml = cts.parse::<Document>()?;

    cargo_toml["build-dependencies"] = table();

    let snapper = &mut cargo_toml["build-dependencies"]["snapper"];
    snapper["version"] = value("0.1");
    snapper["git"] = value("https://github.com/tiannian/snapper");

    let mut features = Array::new();
    features.push("build");
    snapper["features"] = value(features);

    fs::write(&cts_path, cargo_toml.to_string())?;

    Ok(())
}
