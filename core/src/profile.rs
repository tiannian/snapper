use core::str::FromStr;

use crate::{Error, Result};

/// Profile
#[derive(Debug, Clone)]
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
