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

    #[error("Unknown Profile Type")]
    UnknownProfileType,

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    TokioIoError(#[from] tokio::io::Error),

    #[error(transparent)]
    EnvError(#[from] std::env::VarError),

    #[error(transparent)]
    TomlDeError(#[from] toml::de::Error),
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
