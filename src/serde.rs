use crate::{KeyBind, KeyBinds, KeyInput, KeySeq};
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

impl<'de, A: Deserialize<'de>> Deserialize<'de> for KeyBinds<A> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::marker::PhantomData;

        struct V<A>(PhantomData<A>);

        impl<'de, A: Deserialize<'de>> Visitor<'de> for V<A> {
            type Value = KeyBinds<A>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("key bindings object as pairs of key sequences and actions")
            }

            fn visit_map<M: MapAccess<'de>>(self, mut access: M) -> Result<Self::Value, M::Error> {
                let mut binds = vec![];
                while let Some((seq, action)) = access.next_entry()? {
                    binds.push(KeyBind { seq, action });
                }
                Ok(KeyBinds(binds))
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
        bindings: KeyBinds<A>,
    }

    #[test]
    fn deserialize_key_binds() {
        let input = r#"
        [bindings]
        "j" = "Action1"
        "g g" = "Action2"
        "Ctrl+o" = "Action3"
        "Ctrl+S Alt+Shift+G" = "Action4"
        "#;

        let config: Config = toml::from_str(input).unwrap();
        let actual = config.bindings;
        let expected = [
            KeyBind::single(KeyInput::new('j', Mods::NONE), A::Action1),
            KeyBind::multiple(
                KeySeq::new(vec![
                    KeyInput::new('g', Mods::NONE),
                    KeyInput::new('g', Mods::NONE),
                ]),
                A::Action2,
            ),
            KeyBind::single(KeyInput::new('o', Mods::CTRL), A::Action3),
            KeyBind::multiple(
                KeySeq::new(vec![
                    KeyInput::new('s', Mods::CTRL),
                    KeyInput::new('g', Mods::ALT | Mods::SHIFT),
                ]),
                A::Action4,
            ),
        ];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn deserialize_mod_key_bind() {
        let input = r#""Mod+x" = "Action1""#;
        let actual: KeyBinds<A> = toml::from_str(input).unwrap();
        let expected = [
            #[cfg(target_os = "macos")]
            KeyBind::single(KeyInput::new('x', Mods::CMD), A::Action1),
            #[cfg(not(target_os = "macos"))]
            KeyBind::single(KeyInput::new('x', Mods::CTRL), A::Action1),
        ];
        assert_eq!(actual.0, expected);
    }
}
