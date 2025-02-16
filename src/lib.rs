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
    Ignored,
}

impl From<char> for Key {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

const NAMED_KEYS: phf::Map<&str, Key> = phf::phf_map! {
    "space" => Key::Char(' '),
    "Space" => Key::Char(' '),
    "SPACE" => Key::Char(' '),
    "up" => Key::Up,
    "Up" => Key::Up,
    "UP" => Key::Up,
    "right" => Key::Right,
    "Right" => Key::Right,
    "RIGHT" => Key::Right,
    "down" => Key::Down,
    "Down" => Key::Down,
    "DOWN" => Key::Down,
    "left" => Key::Left,
    "Left" => Key::Left,
    "LEFT" => Key::Left,
    "enter" => Key::Enter,
    "Enter" => Key::Enter,
    "ENTER" => Key::Enter,
    "backspace" => Key::Backspace,
    "Backspace" => Key::Backspace,
    "BACKSPACE" => Key::Backspace,
    "delete" => Key::Delete,
    "Delete" => Key::Delete,
    "DELETE" => Key::Delete,
    "home" => Key::Home,
    "Home" => Key::Home,
    "HOME" => Key::Home,
    "end" => Key::End,
    "End" => Key::End,
    "END" => Key::End,
    "pageup" => Key::PageUp,
    "PageUp" => Key::PageUp,
    "PAGEUP" => Key::PageUp,
    "pagedown" => Key::PageDown,
    "PageDown" => Key::PageDown,
    "PAGEDOWN" => Key::PageDown,
    "esc" => Key::Esc,
    "Esc" => Key::Esc,
    "ESC" => Key::Esc,
    "escape" => Key::Esc,
    "Escape" => Key::Esc,
    "ESCAPE" => Key::Esc,
    "tab" => Key::Tab,
    "Tab" => Key::Tab,
    "TAB" => Key::Tab,
    "backtab" => Key::Backtab,
    "Backtab" => Key::Backtab,
    "BACKTAB" => Key::Backtab,
    "insert" => Key::Insert,
    "Insert" => Key::Insert,
    "INSERT" => Key::Insert,
    "copy" => Key::Copy,
    "Copy" => Key::Copy,
    "COPY" => Key::Copy,
    "cut" => Key::Cut,
    "Cut" => Key::Cut,
    "CUT" => Key::Cut,
    "paste" => Key::Paste,
    "Paste" => Key::Paste,
    "PASTE" => Key::Paste,
    "clear" => Key::Clear,
    "Clear" => Key::Clear,
    "CLEAR" => Key::Clear,
    "undo" => Key::Undo,
    "Undo" => Key::Undo,
    "UNDO" => Key::Undo,
    "redo" => Key::Redo,
    "Redo" => Key::Redo,
    "REDO" => Key::Redo,
    "zoomin" => Key::ZoomIn,
    "ZoomIn" => Key::ZoomIn,
    "ZOOMIN" => Key::ZoomIn,
    "zoomout" => Key::ZoomOut,
    "ZoomOut" => Key::ZoomOut,
    "ZOOMOUT" => Key::ZoomOut,
    "scrolllock" => Key::ScrollLock,
    "ScrollLock" => Key::ScrollLock,
    "SCROLLLOCK" => Key::ScrollLock,
    "fnlock" => Key::FnLock,
    "FnLock" => Key::FnLock,
    "FNLOCK" => Key::FnLock,
    "numlock" => Key::ScrollLock,
    "NumLock" => Key::ScrollLock,
    "NUMLOCK" => Key::ScrollLock,
    "printscreen" => Key::PrintScreen,
    "PrintScreen" => Key::PrintScreen,
    "PRINTSCREEN" => Key::PrintScreen,
    "menu" => Key::Menu,
    "Menu" => Key::Menu,
    "MENU" => Key::Menu,
    "play" => Key::Play,
    "Play" => Key::Play,
    "PLAY" => Key::Play,
    "pause" => Key::Pause,
    "Pause" => Key::Pause,
    "PAUSE" => Key::Pause,
    "playpause" => Key::PlayPause,
    "PlayPause" => Key::PlayPause,
    "PLAYPAUSE" => Key::PlayPause,
    "stop" => Key::Stop,
    "Stop" => Key::Stop,
    "STOP" => Key::Stop,
    "rewind" => Key::Rewind,
    "Rewind" => Key::Rewind,
    "REWIND" => Key::Rewind,
    "nexttrack" => Key::NextTrack,
    "NextTrack" => Key::NextTrack,
    "NEXTTRACK" => Key::NextTrack,
    "prevtrack" => Key::PrevTrack,
    "PrevTrack" => Key::PrevTrack,
    "PREVTRACK" => Key::PrevTrack,
    "volumeup" => Key::VolumeUp,
    "VolumeUp" => Key::VolumeUp,
    "VOLUMEUP" => Key::VolumeUp,
    "volumedown" => Key::VolumeDown,
    "VolumeDown" => Key::VolumeDown,
    "VOLUMEDOWN" => Key::VolumeDown,
    "mute" => Key::Mute,
    "Mute" => Key::Mute,
    "MUTE" => Key::Mute,
};

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

        if let Some(key) = NAMED_KEYS.get(s).copied() {
            Ok(key)
        } else {
            Err(Error::UnknownKey(s.into()))
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

    fn is_ignored(&self) -> bool {
        self.key == Key::Ignored
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

#[derive(PartialEq, Eq, Debug)]
pub enum Match<T> {
    Matched(T),
    Prefix,
    Unmatch,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeySeq(Vec<KeyInput>);

impl KeySeq {
    pub fn new(v: Vec<KeyInput>) -> Self {
        Self(v)
    }

    pub fn matches(&self, inputs: &[KeyInput]) -> Match<()> {
        let mut ls = self.0.iter();
        let mut rs = inputs.iter();
        loop {
            match (ls.next(), rs.next()) {
                (Some(l), Some(r)) if l != r => return Match::Unmatch,
                (Some(_), Some(_)) => continue,
                (Some(_), None) => return Match::Prefix,
                (None, Some(_)) => return Match::Unmatch,
                (None, None) => return Match::Matched(()),
            }
        }
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

    pub fn find(&self, seq: &[KeyInput]) -> Match<&KeyBind<A>> {
        let mut saw_prefix = false;
        for bind in self.0.iter() {
            match bind.seq.matches(seq) {
                Match::Matched(_) => return Match::Matched(bind),
                Match::Prefix => saw_prefix = true,
                Match::Unmatch => continue,
            }
        }
        if saw_prefix {
            Match::Prefix
        } else {
            Match::Unmatch
        }
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
        let input = input.into();
        if input.is_ignored() {
            return None;
        }
        self.handle_timeout();
        self.ongoing.push(input);

        // `self.reset` cannot be called because the borrow checker needs to split field lifetimes.
        match self.binds.find(&self.ongoing) {
            Match::Matched(bind) => {
                self.ongoing.clear();
                self.last_input = None;
                Some(&bind.action)
            }
            Match::Prefix => None, // Matching is still ongoing
            Match::Unmatch => {
                self.ongoing.clear();
                self.last_input = None;
                None
            }
        }
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

    #[test]
    fn discard_ongoing_nothing_matched() {
        let binds = vec![KeyBind::single(KeyInput::new('a', Mods::NONE), A::Action1)];
        let mut keybinds = KeyBindMatcher::new(KeyBinds(binds.clone()));

        assert_eq!(keybinds.trigger(KeyInput::from('x')), None);
        assert_eq!(keybinds.trigger(KeyInput::from('y')), None);
        assert_eq!(keybinds.trigger(KeyInput::from('a')), Some(&A::Action1));
        assert_eq!(keybinds.trigger(KeyInput::from('z')), None);
        assert_eq!(keybinds.trigger(KeyInput::from('a')), Some(&A::Action1));
    }
}
