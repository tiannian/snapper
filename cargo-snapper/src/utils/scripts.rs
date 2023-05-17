use std::{fs, path::Path};

use anyhow::{anyhow, Result};
use toml_edit::{value, ArrayOfTables, Document, Item, Table};

pub fn add_manifest_bin(project_path: &Path, path: &Path, script: &str) -> Result<()> {
    let manifest_path = project_path.join("Cargo.toml");

    let content = fs::read_to_string(&manifest_path)?;
    let mut document = content.parse::<Document>()?;

    let mut table = Table::new();

    let p = path
        .strip_prefix(project_path)?
        .to_string_lossy()
        .to_string();

    table.insert("name", value(script));
    table.insert("path", value(p));

    if let Some(bins) = document.get_mut("bin") {
        let arr = bins
            .as_array_of_tables_mut()
            .ok_or(anyhow!("bin is not a vec"))?;

        arr.push(table);
    } else {
        let mut arr = ArrayOfTables::new();

        arr.push(table);

        document.insert("bin", Item::ArrayOfTables(arr));
    }

    fs::write(manifest_path, document.to_string())?;

    Ok(())
}

pub fn create(path: &Path, script: &str) -> Result<()> {
    let file = path.join("scripts").join(format!("{}.rs", script));

    if !file.exists() {
        fs::write(&file, include_str!("../../assets/bin.rs"))?;
    }
    add_manifest_bin(path, &file, script)?;

    Ok(())
}

pub fn remove_manifest_bin(project_path: &Path, script: &str) -> Result<()> {
    let manifest_path = project_path.join("Cargo.toml");

    let content = fs::read_to_string(&manifest_path)?;
    let mut document = content.parse::<Document>()?;

    if let Some(bins) = document.get_mut("bin") {
        let arr = bins
            .as_array_of_tables_mut()
            .ok_or(anyhow!("Error type of bin"))?;

        let mut pos = 0;

        for (index, bin) in arr.iter_mut().enumerate() {
            if let Some(_) = bin.get(script) {
                pos = index;
            }
        }

        arr.remove(pos);
    }

    fs::write(manifest_path, document.to_string())?;
    Ok(())
}

pub fn remove(path: &Path, script: &str) -> Result<()> {
    remove_manifest_bin(path, script)?;
    Ok(())
}

pub fn sync(path: &Path) -> Result<()> {
    Ok(())
}
