//! Type collections for solc json api output

use std::collections::HashMap;

use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: String,
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecondarySourceLocations {
    #[serde(flatten)]
    location: SourceLocation,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorType {
    JSONError,
    IOError,
    ParserError,
    DocstringParsingError,
    SyntaxError,
    DeclarationError,
    TypeError,
    UnimplementedFeatureError,
    InternalCompilerError,
    Exception,
    CompilerError,
    FatalError,
    YulException,
    Warning,
    Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputError {
    #[serde(rename = "sourceLocation")]
    pub source_location: Option<SourceLocation>,
    #[serde(rename = "secondarySourceLocations")]
    #[serde(default)]
    pub secondary_source_locations: Vec<SecondarySourceLocations>,
    #[serde(rename = "type")]
    pub ty: ErrorType,
    pub component: String,
    pub severity: Severity,
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,
    pub message: String,
    #[serde(rename = "formattedMessage")]
    pub formatted_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub id: u32,
    pub ast: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ABIInfo {
    #[serde(rename = "event")]
    Event {
        name: String,
        inputs: Vec<Value>,
        #[serde(default)]
        anonymous: bool,
    },
    #[serde(rename = "error")]
    Error { name: String, inputs: Vec<Value> },
    #[serde(rename = "function")]
    Function {
        name: String,
        inputs: Vec<Value>,
        outputs: Vec<Value>,
        #[serde(rename = "stateMutability")]
        #[serde(default)]
        state_mutability: StateMutability,
    },
    #[serde(rename = "constructor")]
    Constructor {
        inputs: Vec<Value>,
        #[serde(rename = "stateMutability")]
        #[serde(default)]
        state_mutability: StateMutability,
    },
    #[serde(rename = "receive")]
    Receive {
        #[serde(rename = "stateMutability")]
        #[serde(default)]
        state_mutability: StateMutability,
    },
    #[serde(rename = "fallback")]
    Fallback {
        #[serde(rename = "stateMutability")]
        #[serde(default)]
        state_mutability: StateMutability,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(default)]
    pub components: Vec<Component>,
    #[serde(default)]
    pub anonymous: bool,
    #[serde(default)]
    pub indexed: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum StateMutability {
    #[serde(rename = "pure")]
    Pure,
    #[serde(rename = "view")]
    View,
    #[serde(rename = "nonpayable")]
    #[default]
    Nonpayable,
    #[serde(rename = "payable")]
    Payable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Storage {
    #[serde(rename = "astId")]
    pub ast_id: u64,
    // TODO: Parse when deserialize
    pub contract: String,
    pub label: String,
    pub offset: u64,
    pub slot: U256,
    #[serde(rename = "type")]
    pub ty: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "encoding")]
pub enum StorageType {
    #[serde(rename = "inplace")]
    Inplace {
        label: String,
        #[serde(rename = "numberOfBytes")]
        number_of_bytes: U256,
        #[serde(default)]
        members: Vec<Storage>,
    },
    #[serde(rename = "mapping")]
    Mapping {
        label: String,
        #[serde(rename = "numberOfBytes")]
        number_of_bytes: U256,
        key: String,
        value: String,
    },
    #[serde(rename = "dynamic_array")]
    DynamicArray {
        label: String,
        #[serde(rename = "numberOfBytes")]
        number_of_bytes: U256,
        base: String,
    },
    #[serde(rename = "bytes")]
    Bytes {
        label: String,
        #[serde(rename = "numberOfBytes")]
        number_of_bytes: U256,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StorageLayout {
    pub storage: Vec<Storage>,
    pub types: HashMap<String, StorageType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedSources {}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkReferencePos {
    pub start: u64,
    pub length: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bytecode {
    #[serde(serialize_with = "hex::serialize")]
    #[serde(deserialize_with = "hex::deserialize")]
    pub object: Vec<u8>,
    pub opcodes: String,
    #[serde(rename = "sourceMap")]
    pub source_map: String,
    #[serde(rename = "generatedSources")]
    pub generated_sources: Vec<GeneratedSources>,
    #[serde(rename = "linkReferences")]
    pub link_references: HashMap<String, HashMap<String, Vec<LinkReferencePos>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeployedBytecode {
    #[serde(rename = "immutableReferences")]
    pub immutable_references: HashMap<String, Vec<LinkReferencePos>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Selector {
    #[serde(serialize_with = "hex::serialize")]
    #[serde(deserialize_with = "hex::deserialize")]
    value: [u8; 4],
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GasUsed {
    Value(U256),
    Infinite(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GasEstimatesCreation {
    #[serde(rename = "codeDepositCost")]
    pub code_deposit_cost: GasUsed,
    #[serde(rename = "executionCost")]
    pub execution_cost: GasUsed,
    #[serde(rename = "totalCost")]
    pub total_cost: GasUsed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GasEstimates {
    pub creation: GasEstimatesCreation,
    pub external: HashMap<String, GasUsed>,
    pub internal: HashMap<String, GasUsed>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EWasm {
    pub wast: String,
    pub wasm: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Evm {
    pub assembly: String,
    pub bytecode: Bytecode,
    #[serde(rename = "deployedBytecode")]
    pub deployed_bytecode: DeployedBytecode,
    #[serde(rename = "methodIdentifiers")]
    pub method_identifiers: HashMap<String, Selector>,
    #[serde(rename = "gasEstimates")]
    pub gas_estimates: GasEstimates,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    pub abi: Vec<ABIInfo>,
    pub metadata: String,
    pub ir: String,
    #[serde(rename = "storageLayout")]
    pub storage_layout: StorageLayout,
    pub evm: Evm,
    pub ewasm: EWasm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompilerOutput {
    pub errors: Vec<OutputError>,
    #[serde(default)]
    pub sources: HashMap<String, Source>,
    #[serde(default)]
    pub contracts: HashMap<String, HashMap<String, Contract>>,
}

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;

    use crate::CompilerOutput;

    #[test]
    fn test() {
        let config = include_str!("output.json");

        let runtime = Runtime::new().unwrap();
        runtime.block_on(async move {
            let _obj: CompilerOutput = serde_json::from_str(&config).unwrap();
            // print!("{:#?}", obj);
        });
    }
}
