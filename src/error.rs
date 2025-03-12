use crate::Key;
use std::error;
use std::fmt;

/// The error type for keybinds crate.
///
/// ```
/// use keybinds::{KeySeq, Error};
///
/// let error = "Foo".parse::<KeySeq>().unwrap_err();
/// assert_eq!(error, Error::UnknownKey("Foo".into()));
///
/// let error = "".parse::<KeySeq>().unwrap_err();
/// assert_eq!(error, Error::EmptyKeySequence);
///
/// let error = "Foo+a".parse::<KeySeq>().unwrap_err();
/// assert_eq!(error, Error::UnknownModifier("Foo".into()));
/// ```
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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
    /// Error raised when `Shift` modifier key is not allowed with the key.
    ///
    /// `Shift` modifier is only available with named keys so key inputs such as `Shift+x` are not allowed. Please read
    /// the top level document of this crate for more details.
    ShiftUnavailable(Key),
}

impl fmt::Display for Error {
    /// Display the error message.
    ///
    /// ```
    /// use keybinds::KeySeq;
    ///
    /// let error = "Foo".parse::<KeySeq>().unwrap_err();
    ///
    /// assert_eq!(format!("{error}"), r#"Unknown key "Foo" in key sequence"#);
    /// ```
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
                write!(f, "Shift modifier is only available with named keys and key \"{key}\" is not a named key")
            }
        }
    }
}

impl error::Error for Error {}

/// The result type for keybinds crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use crate::{Key, KeySeq};

    #[test]
    fn error_message() {
        let error = "Foo".parse::<KeySeq>().unwrap_err();
        assert_eq!(format!("{error}"), r#"Unknown key "Foo" in key sequence"#);
        let error = "Foo+a".parse::<KeySeq>().unwrap_err();
        assert_eq!(
            format!("{error}"),
            r#"Unknown modifier key "Foo" in key sequence"#,
        );
        let error = "".parse::<Key>().unwrap_err();
        assert_eq!(format!("{error}"), r#"Key must not be empty"#);
        let error = "+a".parse::<KeySeq>().unwrap_err();
        assert_eq!(format!("{error}"), r#"Modifier key must not be empty"#);
        let error = "".parse::<KeySeq>().unwrap_err();
        assert_eq!(format!("{error}"), r#"Key sequence must not be empty"#);
        let error = "Shift+a".parse::<KeySeq>().unwrap_err();
        assert_eq!(
            format!("{error}"),
            r#"Shift modifier is only available with named keys and key "a" is not a named key"#,
        );
    }
}
