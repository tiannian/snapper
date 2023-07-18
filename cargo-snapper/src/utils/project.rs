use std::{fs, path::Path};

use anyhow::{anyhow, Result};
use toml_edit::{Array, Document, InlineTable, Item, Value};

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

    let cts = fs::read_to_string(root.join("Cargo.toml"))?;
    let mut cargo_toml = cts.parse::<Document>()?;

    if let Some(item) = cargo_toml.get_mut("build-dependencies") {
        let t = item
            .as_table_mut()
            .ok_or(anyhow!("Failed to parse Cargo.toml"))?;

        let mut snapper_dep = InlineTable::new();

        snapper_dep.insert("version", "0.1".into());
        snapper_dep.insert("git", "https://github.com/tiannian/snapper".into());

        let mut features = Array::new();
        features.push("build");

        snapper_dep.insert("features", Value::Array(features));

        t.insert("snapper", Item::Table(snapper_dep.into_table()));
    }

    Ok(())
}
