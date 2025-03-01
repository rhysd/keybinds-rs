use crate::Key;
use std::error;
use std::fmt;

/// The error type for keybinds crate.
#[derive(PartialEq, Debug)]
pub enum Error {
    /// Error raised when parsing an unknown key like `"Fooo"`.
    UnknownKey(Box<str>),
    /// Error raised when parsing an unknown modifier key like `"Fooo+x"`.
    UnknownModifier(Box<str>),
    /// Error raised when parsing an empty key like `""`.
    EmptyKey,
    /// Error raised when parsing an empty modifier key like `"+x"`.
    EmptyModifier,
    /// Error raised when parsing an empty key sequence like `""`.
    EmptyKeySequence,
    /// Error raised when `Shift` modifier key is not allowed with the key. `Shift` modifier is only available with
    /// named keys so key inputs such as `Shift+x` are not allowed. Please read the top level document of this crate
    /// for more details.
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
                write!(f, "Shift modifier is not available with key \"{key}\". It is only available with named keys")
            }
        }
    }
}

impl error::Error for Error {}

/// The result type for keybinds crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;
