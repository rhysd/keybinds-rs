//! Support for [`serde`] crate.
//!
//! This module provides a [`Deserialize`] trait support for [`Keybinds`] to easily parse key bindings
//! from a configuration file.
//!
//! ```
//! use serde::Deserialize;
//! use keybinds::{Keybinds, KeybindDispatcher, Key, Mods, KeyInput};
//!
//! // Actions dispatched by key bindings
//! #[derive(Deserialize, PartialEq, Eq, Debug)]
//! enum Action {
//!     OpenFile,
//!     ExitApp,
//! }
//!
//! // Configuration file format of your application
//! #[derive(Deserialize)]
//! struct Config {
//!     // `Keybinds` implements serde's `Deserialize`
//!     bindings: Keybinds<Action>,
//! }
//!
//! // Configuration file content
//! let configuration = r#"
//! [bindings]
//! "Ctrl+Alt+Enter" = "OpenFile"
//! "Ctrl+x Ctrl+c" = "ExitApp"
//! "#;
//!
//! // Parse the TOML input
//! let config: Config = toml::from_str(configuration).unwrap();
//!
//! // Create a dispatcher from the key bindings
//! let mut dispatcher = KeybindDispatcher::new(config.bindings);
//!
//! // Use the key bindings
//! let action = dispatcher.dispatch(KeyInput::new(Key::Enter, Mods::CTRL | Mods::ALT));
//! assert_eq!(action, Some(&Action::OpenFile));
//! ```
use crate::{KeyInput, KeySeq, Keybind, Keybinds};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

impl<'de> Deserialize<'de> for KeyInput {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl Visitor<'_> for V {
            type Value = KeyInput;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("key sequence for a key bind")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_str(V)
    }
}

impl<'de> Deserialize<'de> for KeySeq {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl Visitor<'_> for V {
            type Value = KeySeq;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("key sequence for a key bind")
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
                v.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_str(V)
    }
}

impl<'de, A: Deserialize<'de>> Deserialize<'de> for Keybinds<A> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::marker::PhantomData;

        struct V<A>(PhantomData<A>);

        impl<'de, A: Deserialize<'de>> Visitor<'de> for V<A> {
            type Value = Keybinds<A>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("key bindings object as pairs of key sequences and actions")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
                let mut binds = vec![];
                while let Some((seq, action)) = access.next_entry::<KeySeq, A>()? {
                    binds.push(Keybind::new(seq, action));
                }
                Ok(Keybinds::from(binds))
            }
        }

        deserializer.deserialize_str(V(PhantomData::<A>))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{KeyInput, Mods};
    use serde::Deserialize;
    use std::ops::Deref;

    #[derive(Clone, Copy, PartialEq, Eq, Deserialize, Debug)]
    enum A {
        Action1,
        Action2,
        Action3,
        Action4,
        Action5,
    }

    #[derive(Deserialize, Debug)]
    struct Config {
        bindings: Keybinds<A>,
    }

    #[test]
    fn deserialize_key_bindings_ok() {
        let input = r#"
        [bindings]
        "j" = "Action1"
        "g g" = "Action2"
        "Ctrl+o" = "Action3"
        "Ctrl+S Alt+Ctrl+G" = "Action4"
        "#;

        let config: Config = toml::from_str(input).unwrap();
        let actual = config.bindings;
        let expected = [
            Keybind::new('j', A::Action1),
            Keybind::new(
                vec![
                    KeyInput::new('g', Mods::NONE),
                    KeyInput::new('g', Mods::NONE),
                ],
                A::Action2,
            ),
            Keybind::new(KeyInput::new('o', Mods::CTRL), A::Action3),
            Keybind::new(
                vec![
                    KeyInput::new('S', Mods::CTRL),
                    KeyInput::new('G', Mods::ALT | Mods::CTRL),
                ],
                A::Action4,
            ),
        ];
        assert_eq!(actual.deref(), &expected);
    }

    #[test]
    fn deserialize_empty_table() {
        let _: Keybinds<A> = toml::from_str("").unwrap();
    }

    #[test]
    fn deserialize_key_bindings_error() {
        let tests = [
            r#""x" = 12"#,
            r#""x" = "Action123456""#,
            r#""" = "Action1""#,
            r#""     " = "Action1""#,
            r#""Foooo" = "Action1""#,
            r#""Foooo+x" = "Action1""#,
            r#""Ctrl+Fooooo" = "Action1""#,
            r#""Shift+a" = "Action1""#, // Error because it violates invariant
        ];

        for input in tests {
            let _ = toml::from_str::<Keybinds<A>>(input)
                .expect_err(&format!("invalid input {input:?}"));
        }
    }

    #[test]
    fn deserialize_mod_key_bind() {
        let input = r#""Mod+x" = "Action1""#;
        let actual: Keybinds<A> = toml::from_str(input).unwrap();
        let expected = [Keybind::new(KeyInput::new('x', Mods::MOD), A::Action1)];
        assert_eq!(actual.deref(), expected);
    }
}
