use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapperFile {
    pub solidity: Solidity,
    pub library: HashMap<String, HashMap<String, String>>,
    pub networks: HashMap<String, Network>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profiles {
    pub debug: Profile,
    pub release: Profile,
    pub test: Profile,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub debug: bool,
    pub optimizer: Optimizer,
}

impl Default for Profiles {
    fn default() -> Self {
        let debug = Profile {
            debug: true,
            optimizer: Optimizer {
                enable: false,
                runs: 0,
                yul: false,
                yul_stack: false,
                inliner: false,
                deduplicate: false,
                constant: false,
                remove_jumpdest: false,
            },
        };

        let release = Profile {
            debug: false,
            optimizer: Optimizer {
                enable: true,
                runs: 300,
                yul: true,
                yul_stack: true,
                inliner: true,
                deduplicate: true,
                constant: true,
                remove_jumpdest: true,
            },
        };

        let test = debug.clone();

        Self {
            debug,
            release,
            test,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Solidity {
    pub version: Vec<String>,
    #[serde(default)]
    pub via_ir: bool,
    #[serde(default)]
    pub evm_version: EvmVersion,
    #[serde(default)]
    pub profiles: Profiles,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    pub url: String,
    pub accounts: Vec<String>,
}
