use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Can't find target and platform")]
    NoTargetPlatform,

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    TokioIoError(#[from] tokio::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
