#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(doc, docsrs)))]
#![doc = include_str!("../README.md")]

mod error;

#[cfg(feature = "crossterm")]
pub mod crossterm;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "termwiz")]
pub mod termwiz;

#[cfg(feature = "winit")]
pub mod winit;

pub use error::{Error, Result};

use bitflags::bitflags;
use std::ops::Deref;
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

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        {
            let mut c = s.chars();
            if let (Some(c), None) = (c.next(), c.next()) {
                return Ok(Self::Char(c));
            }
        }

        if s.starts_with(['f', 'F']) {
            if let Ok(x) = s[1..].parse() {
                return Ok(Self::F(x));
            }
        }

        match s {
            "space" | "Space" | "SPACE" => Ok(Self::Char(' ')),
            "up" | "Up" | "UP" => Ok(Self::Up),
            "right" | "Right" | "RIGHT" => Ok(Self::Right),
            "down" | "Down" | "DOWN" => Ok(Self::Down),
            "left" | "Left" | "LEFT" => Ok(Self::Left),
            "enter" | "Enter" | "ENTER" => Ok(Self::Enter),
            "backspace" | "Backspace" | "BACKSPACE" => Ok(Self::Backspace),
            "delete" | "Delete" | "DELETE" => Ok(Self::Delete),
            "home" | "Home" | "HOME" => Ok(Self::Home),
            "end" | "End" | "END" => Ok(Self::End),
            "pageup" | "PageUp" | "PAGEUP" => Ok(Self::PageUp),
            "pagedown" | "PageDown" | "PAGEDOWN" => Ok(Self::PageDown),
            "esc" | "Esc" | "ESC" | "escape" | "Escape" | "ESCAPE" => Ok(Self::Esc),
            "tab" | "Tab" | "TAB" => Ok(Self::Tab),
            "backtab" | "Backtab" | "BACKTAB" => Ok(Self::Backtab),
            "insert" | "Insert" | "INSERT" => Ok(Self::Insert),
            "copy" | "Copy" | "COPY" => Ok(Self::Copy),
            "cut" | "Cut" | "CUT" => Ok(Self::Cut),
            "paste" | "Paste" | "PASTE" => Ok(Self::Paste),
            "clear" | "Clear" | "CLEAR" => Ok(Self::Clear),
            "undo" | "Undo" | "UNDO" => Ok(Self::Undo),
            "redo" | "Redo" | "REDO" => Ok(Self::Redo),
            "zoomin" | "ZoomIn" | "ZOOMIN" => Ok(Self::ZoomIn),
            "zoomout" | "ZoomOut" | "ZOOMOUT" => Ok(Self::ZoomOut),
            "scrolllock" | "ScrollLock" | "SCROLLLOCK" => Ok(Self::ScrollLock),
            "fnlock" | "FnLock" | "FNLOCK" => Ok(Self::FnLock),
            "numlock" | "NumLock" | "NUMLOCK" => Ok(Self::ScrollLock),
            "printscreen" | "PrintScreen" | "PRINTSCREEN" => Ok(Self::PrintScreen),
            "menu" | "Menu" | "MENU" => Ok(Self::Menu),
            "play" | "Play" | "PLAY" => Ok(Self::Play),
            "pause" | "Pause" | "PAUSE" => Ok(Self::Pause),
            "playpause" | "PlayPause" | "PLAYPAUSE" => Ok(Self::PlayPause),
            "stop" | "Stop" | "STOP" => Ok(Self::Stop),
            "rewind" | "Rewind" | "REWIND" => Ok(Self::Rewind),
            "nexttrack" | "NextTrack" | "NEXTTRACK" => Ok(Self::NextTrack),
            "prevtrack" | "PrevTrack" | "PREVTRACK" => Ok(Self::PrevTrack),
            "volumeup" | "VolumeUp" | "VOLUMEUP" => Ok(Self::VolumeUp),
            "volumedown" | "VolumeDown" | "VOLUMEDOWN" => Ok(Self::VolumeDown),
            "mute" | "Mute" | "MUTE" => Ok(Self::Mute),
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
        const ALT   = 0b00000100;
        const WIN   = 0b00001000;
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
            "Control" | "control" | "CONTROL" | "Ctrl" | "ctrl" | "CTRL" => Ok(Self::CTRL),
            "Command" | "command" | "COMMAND" | "Cmd" | "cmd" | "CMD" => Ok(Self::CMD),
            "Mod" | "mod" | "MOD" => Ok(Self::MOD),
            "Alt" | "alt" | "ALT" | "Option" | "option" | "OPTION" => Ok(Self::ALT),
            "Super" | "super" | "SUPER" => Ok(Self::SUPER),
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

// TODO: Make this struct to the following to avoid heap allocation.
// ```
// enum KeySeq {
//     Single(KeyInput),
//     Multiple(Vec<KeyInput>),
// }
// ```
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
pub struct Keybind<A> {
    seq: KeySeq,
    action: A,
}

impl<A> Keybind<A> {
    pub fn multiple(seq: KeySeq, action: A) -> Self {
        Self { seq, action }
    }

    pub fn single(input: KeyInput, action: A) -> Self {
        Self::multiple(KeySeq::new(vec![input]), action)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Keybinds<A>(Vec<Keybind<A>>);

impl<A> Default for Keybinds<A> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<A> Deref for Keybinds<A> {
    type Target = [Keybind<A>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<A> Keybinds<A> {
    pub fn new(v: Vec<Keybind<A>>) -> Self {
        Self(v)
    }

    pub fn find(&self, seq: &[KeyInput]) -> Match<&Keybind<A>> {
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

pub struct KeybindDispatcher<A> {
    binds: Keybinds<A>,
    ongoing: Vec<KeyInput>,
    last_input: Option<Instant>,
    timeout: Duration,
}

impl<A> Default for KeybindDispatcher<A> {
    fn default() -> Self {
        Self::new(Keybinds::default())
    }
}

impl<A> KeybindDispatcher<A> {
    pub fn new(binds: Keybinds<A>) -> Self {
        Self {
            binds,
            ongoing: vec![],
            last_input: None,
            timeout: Duration::from_secs(1),
        }
    }

    pub fn add<K: Into<KeySeq>>(&mut self, key: K, action: A) {
        let seq = key.into();
        self.binds.0.push(Keybind::multiple(seq, action));
    }

    pub fn bind(&mut self, key: &str, action: A) -> Result<()> {
        let seq: KeySeq = key.parse()?;
        self.add(seq, action);
        Ok(())
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn reset(&mut self) {
        self.ongoing.clear();
        self.last_input = None;
    }

    pub fn keybinds(&self) -> &Keybinds<A> {
        &self.binds
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

    pub fn dispatch<I: Into<KeyInput>>(&mut self, input: I) -> Option<&A> {
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

impl<A> FromIterator<Keybind<A>> for KeybindDispatcher<A> {
    fn from_iter<T: IntoIterator<Item = Keybind<A>>>(iter: T) -> Self {
        Self::new(Keybinds(iter.into_iter().collect()))
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
            ("A", KeyInput::new('A', Mods::NONE)),
            ("あ", KeyInput::new('あ', Mods::NONE)),
            ("Ctrl+x", KeyInput::new('x', Mods::CTRL)),
            ("Ctrl+Alt+x", KeyInput::new('x', Mods::CTRL | Mods::ALT)),
            ("alt+ctrl+x", KeyInput::new('x', Mods::CTRL | Mods::ALT)),
            (
                "ALT+SUPER+DOWN",
                KeyInput::new(Key::Down, Mods::ALT | Mods::SUPER),
            ),
            #[cfg(target_os = "macos")]
            ("Mod+x", KeyInput::new('x', Mods::CMD)),
            #[cfg(not(target_os = "macos"))]
            ("Mod+x", KeyInput::new('x', Mods::CTRL)),
            #[cfg(target_os = "macos")]
            ("Super+x", KeyInput::new('x', Mods::CMD)),
            #[cfg(not(target_os = "macos"))]
            ("Super+x", KeyInput::new('x', Mods::WIN)),
            ("F1", KeyInput::new(Key::F(1), Mods::NONE)),
            ("Ctrl+F1", KeyInput::new(Key::F(1), Mods::CTRL)),
            ("F20", KeyInput::new(Key::F(20), Mods::NONE)),
            ("Up", KeyInput::new(Key::Up, Mods::NONE)),
            ("Space", KeyInput::new(' ', Mods::NONE)),
            (
                "Ctrl+Super+Enter",
                KeyInput::new(Key::Enter, Mods::CTRL | Mods::SUPER),
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
            Keybind::single(KeyInput::new('a', Mods::NONE), A::Action1),
            Keybind::single(KeyInput::new('a', Mods::CTRL), A::Action2),
            Keybind::multiple(
                KeySeq::new(vec![
                    KeyInput::new('B', Mods::NONE),
                    KeyInput::new('c', Mods::NONE),
                ]),
                A::Action3,
            ),
            Keybind::single(KeyInput::new(Key::Up, Mods::NONE), A::Action4),
        ];

        let mut keybinds = KeybindDispatcher::new(Keybinds(binds.clone()));

        for bind in binds {
            keybinds.reset();
            let len = bind.seq.0.len();
            for (idx, input) in bind.seq.0.iter().enumerate() {
                let is_last = idx + 1 == len;
                let expected = is_last.then_some(bind.action);
                let actual = keybinds.dispatch(input.clone());
                assert_eq!(actual, expected.as_ref(), "bind={bind:?}");
            }
        }
    }

    #[test]
    fn discard_ongoing_nothing_matched() {
        let binds = vec![Keybind::single(KeyInput::new('a', Mods::NONE), A::Action1)];
        let mut keybinds = KeybindDispatcher::new(Keybinds(binds.clone()));

        assert_eq!(keybinds.dispatch(KeyInput::from('x')), None);
        assert_eq!(keybinds.dispatch(KeyInput::from('y')), None);
        assert_eq!(keybinds.dispatch(KeyInput::from('a')), Some(&A::Action1));
        assert_eq!(keybinds.dispatch(KeyInput::from('z')), None);
        assert_eq!(keybinds.dispatch(KeyInput::from('a')), Some(&A::Action1));
    }

    #[test]
    fn dispatcher_from_iter() {
        let expected = vec![
            Keybind::single('a'.into(), A::Action1),
            Keybind::multiple(
                KeySeq::new(vec![
                    KeyInput::new('b', Mods::CTRL),
                    KeyInput::new('c', Mods::MOD),
                ]),
                A::Action2,
            ),
        ];

        let actual: KeybindDispatcher<_> = expected.clone().into_iter().collect();
        assert_eq!(actual.binds.0, expected);
    }
}
