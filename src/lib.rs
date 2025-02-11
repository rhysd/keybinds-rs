use anyhow::{bail, Error, Result};
use bitflags::bitflags;
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Key {
    Char(char),
    Up,
    Right,
    Down,
    Left,
    Enter,
    Backspace,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Esc,
    Tab,
    F(u8),
}

impl From<char> for Key {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        {
            let mut c = s.chars();
            if let (Some(c), None) = (c.next(), c.next()) {
                return Ok(Self::Char(c.to_ascii_lowercase()));
            }
        }

        if s.starts_with(['f', 'F']) {
            if let Ok(x) = s[1..].parse() {
                return Ok(Self::F(x));
            }
        }

        match s {
            "up" | "Up" => Ok(Self::Up),
            "right" | "Right" => Ok(Self::Right),
            "down" | "Down" => Ok(Self::Down),
            "left" | "Left" => Ok(Self::Left),
            "enter" | "Enter" => Ok(Self::Enter),
            "backspace" | "Backspace" => Ok(Self::Backspace),
            "delete" | "Delete" => Ok(Self::Delete),
            "home" | "Home" => Ok(Self::Home),
            "end" | "End" => Ok(Self::End),
            "pageup" | "PageUp" => Ok(Self::PageUp),
            "pagedown" | "PageDown" => Ok(Self::PageDown),
            "esc" | "Esc" => Ok(Self::Esc),
            "tab" | "Tab" => Ok(Self::Tab),
            _ => bail!("Unexpected key {s:?} in key sequence"),
        }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
    pub struct Mods: u8 {
        const NONE     = 0b00000000;
        const CTRL     = 0b00000001;
        const CMD      = 0b00000010;
        const SHIFT    = 0b00000100;
        const ALT      = 0b00001000;
    }
}

impl Mods {
    #[cfg(not(target_os = "macos"))]
    const MOD: Self = Self::CTRL;
    #[cfg(target_os = "macos")]
    const MOD: Self = Self::CMD;
}

impl FromStr for Mods {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Control" | "Ctrl" | "ctrl" => Ok(Self::CTRL),
            "Command" | "command" | "Cmd" | "cmd" => Ok(Self::CMD),
            "Mod" | "mod" => Ok(Self::MOD),
            "Shift" | "shift" => Ok(Self::SHIFT),
            "Alt" | "alt" | "Option" | "option" => Ok(Self::ALT),
            _ => bail!("Unexpected modifier key {s:?} in key sequence"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeyInput {
    key: Key,
    mods: Mods,
}

impl KeyInput {
    pub fn new(key: impl Into<Key>, mods: Mods) -> Self {
        KeyInput {
            key: key.into(),
            mods,
        }
    }
}

impl FromStr for KeyInput {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut s = s.split('+');
        let Some(mut cur) = s.next() else {
            bail!("Key definition is empty");
        };
        let mut mods = Mods::NONE;
        loop {
            if let Some(next) = s.next() {
                mods |= cur.parse()?;
                cur = next;
            } else {
                let key = cur.parse()?;
                return Ok(Self { key, mods });
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeySeq(Vec<KeyInput>);

impl KeySeq {
    pub fn matches(&self, inputs: &[KeyInput]) -> bool {
        self.0.iter().eq(inputs.iter())
    }
}

impl FromStr for KeySeq {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inputs = s
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        if inputs.is_empty() {
            bail!("Key sequence is empty");
        }
        Ok(Self(inputs))
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeyBind<A> {
    seq: KeySeq,
    action: A,
}

impl<A> KeyBind<A> {
    pub fn new(seq: Vec<KeyInput>, action: A) -> Self {
        Self {
            seq: KeySeq(seq),
            action,
        }
    }

    pub fn single(input: KeyInput, action: A) -> Self {
        Self::new(vec![input], action)
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct KeyBinds<A>(Vec<KeyBind<A>>);

impl<A> KeyBinds<A> {
    pub fn find(&self, seq: &[KeyInput]) -> Option<&KeyBind<A>> {
        self.0.iter().find(|bind| bind.seq.matches(seq))
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

#[derive(Default)]
pub struct KeyBindMatcher<A> {
    binds: KeyBinds<A>,
    current: Vec<KeyInput>,
    last_input: Option<Instant>,
}

impl<A> KeyBindMatcher<A> {
    pub fn new(binds: KeyBinds<A>) -> Self {
        Self {
            binds,
            current: vec![],
            last_input: None,
        }
    }

    pub fn reset(&mut self) {
        self.last_input = None;
        self.current.clear();
    }

    fn handle_timeout(&mut self) {
        let now = Instant::now();
        let timeout = self
            .last_input
            .is_some_and(|t| now.duration_since(t).as_secs() > 0);
        if timeout {
            self.reset();
        } else {
            self.last_input = Some(now);
        }
    }

    pub fn find(&mut self, input: KeyInput) -> Option<&A> {
        self.handle_timeout();
        self.current.push(input);

        let action = self.binds.find(&self.current).map(|b| &b.action)?;

        // `self.reset()` cannot be called here because the borrow checker depends on field splitting.
        self.last_input = None;
        self.current.clear();
        Some(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq, Deserialize, Debug)]
    pub enum A {
        Action1,
        Action2,
        Action3,
        Action4,
        Action5,
    }

    #[test]
    fn parse_key_seq() {
        let tests = [
            ("x", KeyInput::new('x', Mods::NONE)),
            ("A", KeyInput::new('a', Mods::NONE)),
            ("あ", KeyInput::new('あ', Mods::NONE)),
            ("Ctrl+x", KeyInput::new('x', Mods::CTRL)),
            ("Ctrl+Shift+x", KeyInput::new('x', Mods::CTRL | Mods::SHIFT)),
            ("shift+ctrl+x", KeyInput::new('x', Mods::CTRL | Mods::SHIFT)),
            #[cfg(target_os = "macos")]
            ("Mod+x", KeyInput::new('x', Mods::CMD)),
            #[cfg(not(target_os = "macos"))]
            ("Mod+x", KeyInput::new('x', Mods::CTRL)),
            ("F", KeyInput::new('f', Mods::NONE)),
            ("F1", KeyInput::new(Key::F(1), Mods::NONE)),
            ("Ctrl+F1", KeyInput::new(Key::F(1), Mods::CTRL)),
            ("F20", KeyInput::new(Key::F(20), Mods::NONE)),
            ("Up", KeyInput::new(Key::Up, Mods::NONE)),
            (
                "Ctrl+Shift+Enter",
                KeyInput::new(Key::Enter, Mods::CTRL | Mods::SHIFT),
            ),
        ];

        for (input, expected) in tests {
            let actual: KeyInput = input.parse().unwrap();
            assert_eq!(actual, expected, "input={input:?}");
        }
    }

    #[test]
    fn handle_input() {
        let binds = vec![
            KeyBind::single(KeyInput::new('a', Mods::NONE), A::Action1),
            KeyBind::single(KeyInput::new('a', Mods::CTRL | Mods::SHIFT), A::Action2),
            KeyBind::new(
                vec![
                    KeyInput::new('b', Mods::NONE),
                    KeyInput::new('c', Mods::NONE),
                ],
                A::Action3,
            ),
            KeyBind::single(KeyInput::new(Key::Up, Mods::NONE), A::Action4),
        ];

        let mut keybinds = KeyBindMatcher::new(KeyBinds(binds.clone()));

        for bind in binds {
            keybinds.reset();
            let len = bind.seq.0.len();
            for (idx, input) in bind.seq.0.iter().enumerate() {
                let is_last = idx + 1 == len;
                let expected = is_last.then_some(bind.action);
                let actual = keybinds.find(input.clone());
                assert_eq!(actual, expected.as_ref(), "bind={bind:?}");
            }
        }
    }

    #[test]
    fn deserialize_key_binds() {
        let input = r#"
        "j" = "Action1"
        "g g" = "Action2"
        "Ctrl+o" = "Action3"
        "Ctrl+S Alt+Shift+G" = "Action4"
        "#;

        let actual: KeyBinds<A> = toml::from_str(input).unwrap();
        let expected = [
            KeyBind::single(KeyInput::new('j', Mods::NONE), A::Action1),
            KeyBind::new(
                vec![
                    KeyInput::new('g', Mods::NONE),
                    KeyInput::new('g', Mods::NONE),
                ],
                A::Action2,
            ),
            KeyBind::single(KeyInput::new('o', Mods::CTRL), A::Action3),
            KeyBind::new(
                vec![
                    KeyInput::new('s', Mods::CTRL),
                    KeyInput::new('g', Mods::ALT | Mods::SHIFT),
                ],
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
