//! Type collections for solc json api input

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use snapper_core::EvmVersion;

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum InputLanguage {
    #[default]
    Solidity,
    Yul,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keccak256: Option<String>,
    #[serde(default)]
    pub urls: Vec<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum StopAfter {
    #[serde(rename = "parsing")]
    #[default]
    Parsing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YulDetails {
    #[serde(rename = "stackAllocation")]
    pub stack_allocation: bool,
    #[serde(rename = "optimizerSteps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizer_steps: Option<String>,
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
    #[serde(rename = "*")]
    All,
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
pub enum OutputSelection {
    #[serde(rename = "ast")]
    Ast,
    #[serde(rename = "abi")]
    Abi,
    #[serde(rename = "devdoc")]
    DevDoc,
    #[serde(rename = "userdoc")]
    UserDoc,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "ir")]
    IR,
    #[serde(rename = "irOptimized")]
    IROptimzed,
    #[serde(rename = "storageLayout")]
    StorageLayout,
    #[serde(rename = "evm.assembly")]
    EvmAssembly,
    #[serde(rename = "evm.legacyAssembly")]
    EvmLeacyAssembly,
    #[serde(rename = "evm.bytecode")]
    EvmBytecode,
    #[serde(rename = "evm.bytecode.functionDebugData")]
    EvmBytecodeFunctionDebugData,
    #[serde(rename = "evm.bytecode.object")]
    EvmBytecodeObject,
    #[serde(rename = "evm.bytecode.opcodes")]
    EvmBytecodeOpcodes,
    #[serde(rename = "evm.bytecode.sourceMap")]
    EvmBytecodeSourceMap,
    #[serde(rename = "evm.bytecode.linkReferences")]
    EvmBytecodeLinkReferences,
    #[serde(rename = "evm.bytecode.generatedSources")]
    EvmBytecodeGeneratedSources,
    #[serde(rename = "evm.deployedBytecode")]
    EvmDeployedBytecode,
    #[serde(rename = "evm.deployedBytecode.immutableReferences")]
    EvmDeployedBytecodeImmutableReferences,
    #[serde(rename = "evm.methodIdentifiers")]
    EvmMethodIdentifiers,
    #[serde(rename = "evm.gasEstimates")]
    EvmGasEstimates,
    #[serde(rename = "ewasm.wast")]
    EWasmWast,
    #[serde(rename = "ewasm.wasm")]
    EWasmWasm,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    pub libraries: HashMap<String, HashMap<String, String>>,
    #[serde(rename = "outputSelection")]
    pub output_selection: HashMap<String, HashMap<String, Vec<OutputSelection>>>,
    #[serde(rename = "modelChecker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_checker: Option<ModelChecker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerInput {
    pub language: InputLanguage,
    pub sources: HashMap<String, SourceFile>,
    pub settings: Settings,
}

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;

    use crate::CompilerInput;

    #[test]
    fn test() {
        let config = include_str!("input.json");

        let runtime = Runtime::new().unwrap();
        runtime.block_on(async move {
            let _input: CompilerInput = serde_json::from_str(&config).unwrap();

            // println!("{:#?}", _input);
        });
    }
}
