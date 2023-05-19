use std::collections::HashMap;

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
    pub source_location: SourceLocation,
    #[serde(rename = "secondarySourceLocations")]
    pub secondary_source_locations: Vec<SecondarySourceLocations>,
    #[serde(rename = "type")]
    pub ty: ErrorType,
    pub component: String,
    pub severity: Severity,
    #[serde(rename = "errorCode")]
    pub error_code: String,
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
pub struct Contract {
    pub abi: Vec<ABIInfo>,
    pub metadata: String,
    pub ir: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output {
    pub errors: Vec<OutputError>,
    pub sources: HashMap<String, Source>,
    pub contracts: HashMap<String, HashMap<String, Contract>>,
}

#[cfg(test)]
mod test {
    use tokio::runtime::Runtime;

    use crate::Output;

    #[test]
    fn test() {
        let config = include_str!("output.json");

        let runtime = Runtime::new().unwrap();
        runtime.block_on(async move {
            let obj: Output = serde_json::from_str(&config).unwrap();
            print!("{:?}", obj);
        });
    }
}
