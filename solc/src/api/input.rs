use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum InputLanguage {
    Solidity,
    Yul,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFile {
    pub keccak256: String,
    #[serde(default)]
    pub urls: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StopAfter {
    #[serde(rename = "parsing")]
    Parsing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YulDetails {
    #[serde(rename = "stackAllocation")]
    pub stack_allocation: bool,
    #[serde(rename = "optimizerSteps")]
    pub optimizer_steps: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptimizerDetails {
    pub peephole: bool,
    pub inliner: bool,
    #[serde(rename = "jumpdestRemover")]
    pub jumpdest_remover: bool,
    #[serde(rename = "orderLiterals")]
    pub order_literals: bool,
    pub deduplicate: bool,
    pub cse: bool,
    #[serde(rename = "constantOptimizer")]
    pub constant_optimizer: bool,
    pub yul: bool,
    #[serde(rename = "yulDetails")]
    pub yul_details: YulDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Optimizer {
    pub enabled: bool,
    pub runs: u32,
    pub details: OptimizerDetails,
}

#[derive(Debug, Serialize, Deserialize)]
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
    Paris,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RevertStrings {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "strip")]
    Strip,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "verboseDebug")]
    VerboseDebug,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DebugInfo {
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "snippet")]
    Snippet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsDebug {
    #[serde(rename = "revertStrings")]
    pub revert_strings: RevertStrings,
    #[serde(rename = "debugInfo")]
    pub debug_info: Vec<DebugInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BytecodeHash {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "ipfs")]
    Ipfs,
    #[serde(rename = "bzzr1")]
    Bzzr1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(rename = "appendCBOR")]
    pub append_cbor: bool,
    #[serde(rename = "useLiteralContent")]
    pub use_literal_content: bool,
    #[serde(rename = "bytecodeHash")]
    pub bytecode_hash: BytecodeHash,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Engine {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "bmc")]
    Bmc,
    #[serde(rename = "chc")]
    Chc,
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtCalls {
    #[serde(rename = "trusted")]
    Trusted,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Invariant {
    #[serde(rename = "contract")]
    Contract,
    #[serde(rename = "reentrancy")]
    Reentrancy,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Solver {
    #[serde(rename = "cvc4")]
    Cvc4,
    #[serde(rename = "smtlib2")]
    Smtlib2,
    #[serde(rename = "z3")]
    Z3,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Target {
    #[serde(rename = "constantCondition")]
    ConstantCondition,
    #[serde(rename = "underflow")]
    Underflow,
    #[serde(rename = "overflow")]
    Overflow,
    #[serde(rename = "divByZero")]
    DivByZero,
    #[serde(rename = "balance")]
    Balance,
    #[serde(rename = "assert")]
    Assert,
    #[serde(rename = "popEmptyArray")]
    PopEmptyArray,
    #[serde(rename = "outOfBounds")]
    OutOfBounds,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelChecker {
    pub contracts: HashMap<String, Vec<String>>,
    #[serde(rename = "divModNoSlacks")]
    pub div_mod_no_slacks: bool,
    pub engine: Engine,
    #[serde(rename = "extCalls")]
    pub ext_calls: ExtCalls,
    pub invariants: Vec<Invariant>,
    #[serde(rename = "showProved")]
    pub show_proved: bool,
    #[serde(rename = "showUnproved")]
    pub show_unproved: bool,
    #[serde(rename = "showUnsupported")]
    pub show_unsupported: bool,
    pub solvers: Vec<Solver>,
    pub targets: Vec<Target>,
    pub timeout: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "stopAfter")]
    pub stop_after: StopAfter,
    pub remappings: Vec<String>,
    pub optimizer: Optimizer,
    #[serde(rename = "evmVersion")]
    pub evm_version: EvmVersion,
    #[serde(rename = "viaIR")]
    pub via_ir: bool,
    pub debug: SettingsDebug,
    pub metadata: Metadata,
    pub libraries: HashMap<String, HashMap<String, String>>,
    #[serde(rename = "outputSelection")]
    pub output_selection: HashMap<String, HashMap<String, Vec<String>>>,
    #[serde(rename = "modelChecker")]
    pub model_checker: ModelChecker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub language: InputLanguage,
    pub sources: HashMap<String, SourceFile>,
    pub settings: Settings,
}

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;

    use crate::Input;

    #[test]
    fn test() {
        let config = include_str!("input.json");

        let runtime = Runtime::new().unwrap();
        runtime.block_on(async move {
            let _input: Input = serde_json::from_str(&config).unwrap();
        });
    }
}
