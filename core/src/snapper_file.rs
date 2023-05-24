use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SnapperFile {
    pub solidity: Solidity,
    pub library: HashMap<String, HashMap<String, String>>,
    pub profile: Profiles,
    pub networks: HashMap<String, Network>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profiles {
    pub debug: Profile,
    pub release: Profile,
    pub test: Profile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub debug: bool,
    pub optimizer: Optimizer,
}

impl Default for Profiles {
    fn default() -> Self {
        Self {
            debug,
            release,
            test,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum EvmVersion {
    #[serde(rename = "homestead")]
    Homestead,
    #[serde(rename = "tangerineWhistle")]
    TangerineWhistle,
    #[serde(rename = "spuriousDragon")]
    SpuriousDragon,
    #[serde(rename = "byzantium")]
    Byzantium,
    #[serde(rename = "constantinople")]
    Constantinople,
    #[serde(rename = "petersburg")]
    Petersburg,
    #[serde(rename = "istanbul")]
    Istanbul,
    #[serde(rename = "berlin")]
    Berlin,
    #[serde(rename = "london")]
    London,
    #[serde(rename = "paris")]
    #[default]
    Paris,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Solidity {
    pub version: Vec<String>,
    #[serde(default)]
    pub via_ir: bool,
    pub evm_version: EvmVersion,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Optimizer {
    pub enable: bool,
    pub runs: u32,
    pub yul: bool,
    pub yul_stack: bool,
    pub inliner: bool,
    pub deduplicate: bool,
    pub constant: bool,
    pub remove_jumpdest: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    pub url: String,
    pub accounts: Vec<String>,
}
