use alloc::{collections::BTreeMap, string::String, vec::Vec};

use serde::{Deserialize, Serialize};

use crate::ProfileType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SnapperFile {
    pub solidity: Solidity,
    #[serde(default)]
    pub library: BTreeMap<String, BTreeMap<String, String>>,
    pub networks: BTreeMap<String, Network>,
}

impl SnapperFile {
    pub fn get_solidity_profile(&self, profile: &ProfileType) -> &Profile {
        match profile {
            ProfileType::Debug => &self.solidity.profiles.debug,
            ProfileType::Release => &self.solidity.profiles.release,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profiles {
    pub debug: Profile,
    pub release: Profile,
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
                cse: false,
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
                cse: true,
            },
        };

        Self { debug, release }
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
    #[default]
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
    Paris,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Solidity {
    pub version: String,
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
    pub cse: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Network {
    pub url: String,
    pub accounts: Vec<String>,
}
