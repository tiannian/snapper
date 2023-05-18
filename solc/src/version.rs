use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Target {
    #[serde(rename = "amd64")]
    Amd64,
    #[serde(rename = "aarch64")]
    AArch64,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Platform {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "macos")]
    MacOS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artifact {
    pub platform: Platform,
    pub target: Target,
    pub urls: Vec<String>,
    pub keccak256: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub target: Vec<Artifact>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerVersion {
    pub builds: HashMap<String, Version>,
}
