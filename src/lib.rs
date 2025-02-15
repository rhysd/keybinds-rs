#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod error;

#[cfg(feature = "crossterm")]
pub mod crossterm;
#[cfg(feature = "serde")]
pub mod serde;
#[cfg(feature = "winit")]
pub mod winit;

pub use error::{Error, Result};

use bitflags::bitflags;
use std::str::FromStr;
use std::time::{Duration, Instant};

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
    Backtab,
    Insert,
    Copy,
    Cut,
    Paste,
    Clear,
    Undo,
    Redo,
    ZoomIn,
    ZoomOut,
    ScrollLock,
    NumLock,
    FnLock,
    PrintScreen,
    Menu,
    Play,
    Pause,
    PlayPause,
    Stop,
    Rewind,
    NextTrack,
    PrevTrack,
    VolumeUp,
    VolumeDown,
    Mute,
    F(u8),
    Unidentified,
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

        // TODO: Use phf for O(1) switch
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
            "backtab" | "Backtab" => Ok(Self::Tab),
            "insert" | "Insert" => Ok(Self::Insert),
            "copy" | "Copy" => Ok(Self::Copy),
            "cut" | "Cut" => Ok(Self::Cut),
            "paste" | "Paste" => Ok(Self::Paste),
            "clear" | "Clear" => Ok(Self::Clear),
            "undo" | "Undo" => Ok(Self::Undo),
            "redo" | "Redo" => Ok(Self::Redo),
            "zoomin" | "ZoomIn" => Ok(Self::ZoomIn),
            "zoomout" | "ZoomOut" => Ok(Self::ZoomOut),
            "scrolllock" | "ScrollLock" => Ok(Self::ScrollLock),
            "fnlock" | "FnLock" => Ok(Self::FnLock),
            "numlock" | "NumLock" => Ok(Self::ScrollLock),
            "printscreen" | "PrintScreen" => Ok(Self::PrintScreen),
            "menu" | "Menu" => Ok(Self::Menu),
            "play" | "Play" => Ok(Self::Play),
            "pause" | "Pause" => Ok(Self::Pause),
            "playpause" | "PlayPause" => Ok(Self::PlayPause),
            "stop" | "Stop" => Ok(Self::Stop),
            "rewind" | "Rewind" => Ok(Self::Rewind),
            "nexttrack" | "NextTrack" => Ok(Self::NextTrack),
            "prevtrack" | "PrevTrack" => Ok(Self::PrevTrack),
            "volumeup" | "VolumeUp" => Ok(Self::VolumeUp),
            "volumedown" | "VolumeDown" => Ok(Self::VolumeDown),
            "mute" | "Mute" => Ok(Self::Mute),
            _ => Err(Error::UnknownKey(s.into())),
        }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Default, Copy, Clone, PartialEq, Eq, Hash, Debug)]
    pub struct Mods: u8 {
        const NONE  = 0b00000000;
        const CTRL  = 0b00000001;
        const CMD   = 0b00000010;
        const SHIFT = 0b00000100;
        const ALT   = 0b00001000;
        const WIN   = 0b00010000;
    }
}

impl Mods {
    #[cfg(not(target_os = "macos"))]
    const MOD: Self = Self::CTRL;
    #[cfg(target_os = "macos")]
    const MOD: Self = Self::CMD;
    #[cfg(not(target_os = "macos"))]
    const SUPER: Self = Self::WIN;
    #[cfg(target_os = "macos")]
    const SUPER: Self = Self::CMD;
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
            "Super" | "super" => Ok(Self::SUPER),
            _ => Err(Error::UnknownModifier(s.into())),
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
            return Err(Error::EmptyKeyInput);
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

impl<K: Into<Key>> From<K> for KeyInput {
    fn from(k: K) -> Self {
        Self::new(k.into(), Mods::NONE)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeySeq(Vec<KeyInput>);

impl KeySeq {
    pub fn new(v: Vec<KeyInput>) -> Self {
        Self(v)
    }

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
            return Err(Error::EmptyKeySequence);
        }
        Ok(Self(inputs))
    }
}

impl From<char> for KeySeq {
    fn from(c: char) -> Self {
        Self::new(vec![c.into()])
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeyBind<A> {
    seq: KeySeq,
    action: A,
}

impl<A> KeyBind<A> {
    pub fn multiple(seq: KeySeq, action: A) -> Self {
        Self { seq, action }
    }

    pub fn single(input: KeyInput, action: A) -> Self {
        Self::multiple(KeySeq::new(vec![input]), action)
    }
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct KeyBinds<A>(Vec<KeyBind<A>>);

impl<A> KeyBinds<A> {
    pub fn new(v: Vec<KeyBind<A>>) -> Self {
        Self(v)
    }

    pub fn find(&self, seq: &[KeyInput]) -> Option<&KeyBind<A>> {
        self.0.iter().find(|bind| bind.seq.matches(seq))
    }
}

#[derive(Default)]
pub struct KeyBindMatcher<A> {
    binds: KeyBinds<A>,
    ongoing: Vec<KeyInput>,
    last_input: Option<Instant>,
    timeout: Duration,
}

impl<A> KeyBindMatcher<A> {
    pub fn new(binds: KeyBinds<A>) -> Self {
        Self {
            binds,
            ongoing: vec![],
            last_input: None,
            timeout: Duration::from_secs(1),
        }
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn reset(&mut self) {
        self.ongoing.clear();
        self.last_input = None;
    }

    fn handle_timeout(&mut self) {
        let now = Instant::now();
        let is_timeout = self
            .last_input
            .is_some_and(|t| now.duration_since(t) > self.timeout);
        if is_timeout {
            self.ongoing.clear();
        }
        self.last_input = Some(now);
    }

    pub fn trigger<I: Into<KeyInput>>(&mut self, input: I) -> Option<&A> {
        self.handle_timeout();
        self.ongoing.push(input.into());

        // TODO: When no keybind is prefix-matched, call `self.reset()`
        let action = self.binds.find(&self.ongoing).map(|b| &b.action)?;

        // `self.reset` cannot be called because the borrow checker needs to split field lifetimes.
        self.ongoing.clear();
        self.last_input = None;
        Some(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum A {
        Action1,
        Action2,
        Action3,
        Action4,
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
            #[cfg(target_os = "macos")]
            ("Super+x", KeyInput::new('x', Mods::CMD)),
            #[cfg(not(target_os = "macos"))]
            ("Super+x", KeyInput::new('x', Mods::WIN)),
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
            KeyBind::multiple(
                KeySeq::new(vec![
                    KeyInput::new('b', Mods::NONE),
                    KeyInput::new('c', Mods::NONE),
                ]),
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
                let actual = keybinds.trigger(input.clone());
                assert_eq!(actual, expected.as_ref(), "bind={bind:?}");
            }
        }
    }
}
