use core::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Type of Profile
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileType {
    Debug,
    Release,
}

impl FromStr for ProfileType {
    type Err = Error;

    fn from_str(ty: &str) -> Result<Self> {
        match ty {
            "release" => Ok(Self::Release),
            "debug" => Ok(Self::Debug),
            _ => Err(Error::UnknownProfileType),
        }
    }
}
