use super::{KeyBind, KeyBinds, KeySeq};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;

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
