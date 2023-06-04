use core::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UnknownProfileType,
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
