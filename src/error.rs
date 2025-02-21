use std::error;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Error {
    UnknownKey(Box<str>),
    UnknownModifier(Box<str>),
    EmptyKey,
    EmptyModifier,
    EmptyKeySequence,
    ShiftUnavailable,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownKey(key) => write!(f, "Unknown key {key:?} in key sequence"),
            Self::UnknownModifier(key) => {
                write!(f, "Unknown modifier key {key:?} in key sequence")
            }
            Self::EmptyKey => write!(f, "Key value is empty"),
            Self::EmptyModifier => write!(f, "Modifier key value is empty"),
            Self::EmptyKeySequence => write!(f, "Key sequence is empty"),
            Self::ShiftUnavailable => write!(f, "Shift modifier is only available with named keys"),
        }
    }
}

impl error::Error for Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;
