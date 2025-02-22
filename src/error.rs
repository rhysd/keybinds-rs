use crate::Key;
use std::error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Error {
    UnknownKey(Box<str>),
    UnknownModifier(Box<str>),
    EmptyKey,
    EmptyModifier,
    EmptyKeySequence,
    ShiftUnavailable(Key),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownKey(key) => write!(f, "Unknown key {key:?} in key sequence"),
            Self::UnknownModifier(key) => {
                write!(f, "Unknown modifier key {key:?} in key sequence")
            }
            Self::EmptyKey => write!(f, "Key must not be empty"),
            Self::EmptyModifier => write!(f, "Modifier key must not be empty"),
            Self::EmptyKeySequence => write!(f, "Key sequence must not be empty"),
            Self::ShiftUnavailable(key) => {
                write!(f, "Shift modifier is not available with key {key:?}. It is only available with named keys")
            }
        }
    }
}

impl error::Error for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
