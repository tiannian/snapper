use thiserror::Error;

/// Error
#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't find target and platform")]
    NoTargetPlatform,

    #[error("Unsupport platform")]
    UnsupportPlatform,

    #[error("Failed To Get Stdio")]
    FailedToGetStdio,

    #[error("Failed To Parse Filename")]
    FailedToParseFileName,

    #[error("Unknown Profile Type")]
    UnknownProfileType,

    #[error("No contract output")]
    NoContractOutput,

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    EnvError(#[from] std::env::VarError),

    #[error(transparent)]
    TomlDeError(#[from] toml::de::Error),

    #[error(transparent)]
    StripPrefixError(#[from] std::path::StripPrefixError),

    #[error("Core error: {0}")]
    CoreError(snapper_core::Error),
}

impl From<snapper_core::Error> for Error {
    fn from(value: snapper_core::Error) -> Self {
        Self::CoreError(value)
    }
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
