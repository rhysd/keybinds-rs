use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnknownKey(Box<str>),
    UnknownModifier(Box<str>),
    EmptyKeyInput,
    EmptyKeySequence,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownKey(key) => write!(f, "Unknown key {key:?} in key sequence"),
            Self::UnknownModifier(key) => {
                write!(f, "Unknown modifier key {key:?} in key sequence")
            }
            Self::EmptyKeyInput => write!(f, "Key input definition is empty"),
            Self::EmptyKeySequence => write!(f, "Key sequence is empty"),
        }
    }
}

impl error::Error for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
