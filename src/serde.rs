//! Support for [`serde`] crate.
//!
//! This module provides [`Deserialize`] and [`Serialize`] traits support for [`Keybinds`] and some other types
//! following the [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
//! This is useful for parsing key bindings from a configuration file.
//!
//! ```
//! use serde::{Serialize, Deserialize};
//! use keybinds::{Keybinds, Key, Mods, KeyInput};
//!
//! // Actions dispatched by key bindings
//! #[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
//! enum Action {
//!     OpenFile,
//!     ExitApp,
//! }
//!
//! // Configuration file format of your application
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     // `Keybinds` implements serde's `Deserialize`
//!     keyboard: Keybinds<Action>,
//! }
//!
//! // Configuration file content
//! let configuration =
//! r#"[keyboard]
//! "Ctrl+Alt+Enter" = "OpenFile"
//! "Ctrl+x Ctrl+c" = "ExitApp"
//! "#;
//!
//! // Parse the configuration as TOML input
//! let parsed: Config = toml::from_str(configuration).unwrap();
//!
//! // Generate the configuration as TOML
//! let generated = toml::to_string_pretty(&parsed).unwrap();
//!
//! assert_eq!(&generated, configuration);
//! ```
use crate::{KeyInput, KeySeq, Keybind, Keybinds};
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde::ser::{Error as _, Serialize, SerializeMap, Serializer};
use std::fmt;

impl<'de> Deserialize<'de> for KeyInput {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct V;

        impl Visitor<'_> for V {
            type Value = KeyInput;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("key sequence for a key bind")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
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

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
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
                Ok(Keybinds::new(binds))
            }
        }

        deserializer.deserialize_str(V(PhantomData::<A>))
    }
}

impl Serialize for KeyInput {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl Serialize for KeySeq {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.as_slice().is_empty() {
            return Err(S::Error::custom("Key sequence must not be empty"));
        }
        serializer.collect_str(self)
    }
}

impl<A: Serialize> Serialize for Keybinds<A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.as_slice().len()))?;
        for keybind in self.as_slice().iter() {
            map.serialize_entry(&keybind.seq, &keybind.action)?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Key, KeyInput, Mods};
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
    enum A {
        Action1,
        Action2,
        Action3,
        Action4,
        Action5,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Config {
        bindings: Keybinds<A>,
    }

    #[test]
    fn deserialize_ok() {
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
            Keybind::new(['g', 'g'], A::Action2),
            Keybind::new(KeyInput::new('o', Mods::CTRL), A::Action3),
            Keybind::new(
                [
                    KeyInput::new('S', Mods::CTRL),
                    KeyInput::new('G', Mods::ALT | Mods::CTRL),
                ],
                A::Action4,
            ),
        ];
        assert_eq!(actual.as_slice(), &expected);
    }

    #[test]
    fn deserialize_empty_table() {
        let _: Keybinds<A> = toml::from_str("").unwrap();
    }

    #[test]
    fn deserialize_error() {
        let tests = [
            "hello",
            "42",
            "[[foo]]",
            r#""x" = 12"#,
            r#"x = []"#,
            r#""x" = "Action123456""#,
            r#""" = "Action1""#,
            r#""     " = "Action1""#,
            r#""Foooo" = "Action1""#,
            r#""Foooo+x" = "Action1""#,
            r#""Ctrl+Fooooo" = "Action1""#,
            r#""Shift+a" = "Action1""#, // Error because it violates invariant
        ];

        for input in tests {
            if let Ok(k) = toml::from_str::<Keybinds<A>>(input) {
                panic!("parse was successful: {k:?} (input={input:?}");
            }
        }
    }

    #[test]
    fn deserialize_mod_key_bind() {
        let input = r#""Mod+x" = "Action1""#;
        let actual: Keybinds<A> = toml::from_str(input).unwrap();
        let expected = [Keybind::new(KeyInput::new('x', Mods::MOD), A::Action1)];
        assert_eq!(actual.as_slice(), &expected);
    }

    #[test]
    fn serialize_ok() {
        let binds = vec![
            Keybind::new('a', A::Action1),
            Keybind::new(Key::Up, A::Action2),
            Keybind::new(KeyInput::new(Key::Enter, Mods::CTRL), A::Action3),
            Keybind::new(KeySeq::from(['H', 'e', 'l', 'l', 'o']), A::Action4),
            Keybind::new(
                KeySeq::from([
                    KeyInput::new('X', Mods::ALT | Mods::CTRL),
                    KeyInput::new(Key::Up, Mods::SHIFT),
                ]),
                A::Action5,
            ),
        ];
        let config = Config {
            bindings: Keybinds::new(binds),
        };
        let actual = toml::to_string_pretty(&config).unwrap();
        let expected = r#"[bindings]
a = "Action1"
Up = "Action2"
"Ctrl+Enter" = "Action3"
"H e l l o" = "Action4"
"Ctrl+Alt+X Shift+Up" = "Action5"
"#;

        assert_eq!(&actual, expected);
    }

    #[test]
    fn serialize_error() {
        let _ = toml::to_string_pretty(&KeySeq::default()).unwrap_err();
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct TestInput {
        key: KeyInput,
    }
    impl TestInput {
        fn new(k: impl Into<Key>, m: Mods) -> Self {
            Self {
                key: KeyInput::new(k.into(), m),
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct TestSeq {
        seq: KeySeq,
    }

    #[test]
    fn deserialize_key_input_ok() {
        for (input, expected) in [
            (r#"key = "a""#, KeyInput::from('a')),
            (r#"key = "Enter""#, KeyInput::from(Key::Enter)),
            (r#"key = "Ctrl+a""#, KeyInput::new('a', Mods::CTRL)),
            (r#"key = "Shift+Up""#, KeyInput::new(Key::Up, Mods::SHIFT)),
        ] {
            assert_eq!(
                toml::from_str::<TestInput>(input).unwrap().key,
                expected,
                "input={input:?}",
            );
        }
    }

    #[test]
    fn deserialize_key_input_error() {
        for input in [
            r#"key = 42"#,
            r#"key = """#,
            r#"key = "Fooo""#,
            r#"key = "Foo+a""#,
        ] {
            if let Ok(parsed) = toml::from_str::<TestInput>(input) {
                panic!(
                    "Input {:?} was parsed from invalid input {:?}",
                    parsed.key, input
                );
            }
        }
    }

    #[test]
    fn deserialize_key_seq_ok() {
        for (input, expected) in [
            (r#"seq = "a""#, KeySeq::from(['a'])),
            (r#"seq = "a b c""#, KeySeq::from(['a', 'b', 'c'])),
            (r#"seq = "Enter""#, KeySeq::from([Key::Enter])),
            (
                r#"seq = "Up Right Down Left""#,
                KeySeq::from([Key::Up, Key::Right, Key::Down, Key::Left]),
            ),
        ] {
            assert_eq!(
                toml::from_str::<TestSeq>(input).unwrap().seq,
                expected,
                "input={input:?}",
            );
        }
    }

    #[test]
    fn deserialize_key_seq_error() {
        for input in [
            r#"seq = 42"#,
            r#"seq = """#,
            r#"seq = "Fooo""#,
            r#"seq = "Foo+a""#,
        ] {
            if let Ok(parsed) = toml::from_str::<TestSeq>(input) {
                panic!(
                    "Key sequence {:?} was parsed from invalid input {:?}",
                    parsed.seq, input
                );
            }
        }
    }

    #[test]
    fn serialize_key_input_ok() {
        for (input, expected) in [
            (TestInput::new('a', Mods::NONE), r#"key = "a""#),
            (TestInput::new(Key::Enter, Mods::NONE), r#"key = "Enter""#),
            (TestInput::new('a', Mods::CTRL), r#"key = "Ctrl+a""#),
            (TestInput::new(Key::Tab, Mods::ALT), r#"key = "Alt+Tab""#),
        ] {
            assert_eq!(
                toml::to_string(&input).unwrap().trim(),
                expected,
                "input={input:?}",
            );
        }
    }
}
