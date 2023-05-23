use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapperFile {
    pub solidity: Solidity,
    pub library: HashMap<String, HashMap<String, String>>,
    pub networks: HashMap<String, Network>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Solidity {
    pub version: Vec<String>,
    pub via_ir: bool,
    pub optimizer: Optimizer,
    pub evm_version: EVMVersion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Optimizer {
    pub enable: bool,
    pub runs: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EVMVersion {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub url: String,
    pub accounts: Vec<String>,
}
