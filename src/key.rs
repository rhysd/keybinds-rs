use crate::Error;
use bitflags::bitflags;
use std::slice;
use std::str::FromStr;

#[non_exhaustive]
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
        let s = s.trim();
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
            "plus" | "Plus" | "PLUS" => Ok(Self::Char('+')),
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
            "" => Err(Error::EmptyKey),
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
    pub const MOD: Self = Self::CTRL;
    #[cfg(target_os = "macos")]
    pub const MOD: Self = Self::CMD;
    #[cfg(not(target_os = "macos"))]
    pub const SUPER: Self = Self::WIN;
    #[cfg(target_os = "macos")]
    pub const SUPER: Self = Self::CMD;
}

impl FromStr for Mods {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Control" | "control" | "CONTROL" | "Ctrl" | "ctrl" | "CTRL" => Ok(Self::CTRL),
            "Command" | "command" | "COMMAND" | "Cmd" | "cmd" | "CMD" => Ok(Self::CMD),
            "Mod" | "mod" | "MOD" => Ok(Self::MOD),
            "Alt" | "alt" | "ALT" | "Option" | "option" | "OPTION" => Ok(Self::ALT),
            "Super" | "super" | "SUPER" => Ok(Self::SUPER),
            "" => Err(Error::EmptyModifier),
            _ => Err(Error::UnknownModifier(s.into())),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeyInput {
    pub key: Key,
    pub mods: Mods,
}

impl KeyInput {
    pub fn new(key: impl Into<Key>, mods: Mods) -> Self {
        KeyInput {
            key: key.into(),
            mods,
        }
    }

    pub(crate) fn is_ignored(&self) -> bool {
        self.key == Key::Ignored
    }
}

impl FromStr for KeyInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim().split('+');
        let mut cur = s.next().unwrap(); // Iterator by `.split()` is never empty
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

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Match {
    Matched,
    Prefix,
    Unmatch,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum KeySeq {
    Multiple(Vec<KeyInput>),
    Single(KeyInput),
}

impl KeySeq {
    pub fn matches(&self, inputs: &[KeyInput]) -> Match {
        let mut ls = self.as_slice().iter();
        let mut rs = inputs.iter();
        loop {
            match (ls.next(), rs.next()) {
                (Some(l), Some(r)) if l != r => return Match::Unmatch,
                (Some(_), Some(_)) => continue,
                (Some(_), None) => return Match::Prefix,
                (None, Some(_)) => return Match::Unmatch,
                (None, None) => return Match::Matched,
            }
        }
    }

    fn push(self, input: KeyInput) -> Self {
        match self {
            Self::Multiple(v) if v.is_empty() => Self::Single(input),
            Self::Multiple(mut v) => {
                v.push(input);
                Self::Multiple(v)
            }
            Self::Single(k) => Self::Multiple(vec![k, input]),
        }
    }

    pub fn as_slice(&self) -> &[KeyInput] {
        match self {
            Self::Multiple(v) => v.as_slice(),
            Self::Single(k) => slice::from_ref(k),
        }
    }
}

impl FromStr for KeySeq {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut keys = s.trim().split_whitespace();
        if let Some(key) = keys.next() {
            let mut seq = Self::Single(key.parse()?);
            for key in keys {
                seq = seq.push(key.parse()?);
            }
            Ok(seq)
        } else {
            Err(Error::EmptyKeySequence)
        }
    }
}

impl<K: Into<KeyInput>> From<K> for KeySeq {
    fn from(key: K) -> Self {
        Self::Single(key.into())
    }
}

impl From<Vec<KeyInput>> for KeySeq {
    fn from(mut v: Vec<KeyInput>) -> Self {
        if v.len() == 1 {
            Self::Single(v.pop().unwrap())
        } else {
            Self::Multiple(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key_input_ok() {
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
            ("  x  ", KeyInput::new('x', Mods::NONE)),
            ("Ctrl+Plus", KeyInput::new('+', Mods::CTRL)),
        ];

        for (input, expected) in tests {
            let actual: KeyInput = input.parse().unwrap();
            assert_eq!(actual, expected, "input={input:?}");
        }
    }

    #[test]
    fn parse_key_input_error() {
        let tests = [
            ("", Error::EmptyKey),
            (" ", Error::EmptyKey),
            ("+", Error::EmptyModifier),
            ("+a", Error::EmptyModifier),
            ("Ctrl+", Error::EmptyKey),
            ("Hoge+", Error::UnknownModifier("Hoge".into())),
            ("Fooooo", Error::UnknownKey("Fooooo".into())),
        ];

        for (input, expected) in tests {
            assert_eq!(input.parse::<KeyInput>(), Err(expected), "input={input:?}");
        }
    }

    #[test]
    fn parse_key_seq_ok() {
        let tests = [
            ("x", KeySeq::from('x')),
            ("Enter", KeySeq::from(Key::Enter)),
            ("Ctrl+x", KeySeq::from(KeyInput::new('x', Mods::CTRL))),
            (
                "a b c",
                KeySeq::from(vec!['a'.into(), 'b'.into(), 'c'.into()]),
            ),
            (
                "Up Down Enter",
                KeySeq::from(vec![Key::Up.into(), Key::Down.into(), Key::Enter.into()]),
            ),
            (
                "Ctrl+Alt+a Super+b Mod+c",
                KeySeq::from(vec![
                    KeyInput::new('a', Mods::ALT | Mods::CTRL),
                    KeyInput::new('b', Mods::SUPER),
                    KeyInput::new('c', Mods::MOD),
                ]),
            ),
        ];

        for (seq, expected) in tests {
            assert_eq!(seq.parse::<KeySeq>(), Ok(expected), "seq={seq:?}");
        }
    }

    #[test]
    fn parse_key_seq_error() {
        let tests = [
            ("", Error::EmptyKeySequence),
            (" ", Error::EmptyKeySequence),
            ("+", Error::EmptyModifier),
            ("+a", Error::EmptyModifier),
            ("Ctrl+", Error::EmptyKey),
            ("Hoge+", Error::UnknownModifier("Hoge".into())),
            ("Fooooo", Error::UnknownKey("Fooooo".into())),
            ("a b Fooooo", Error::UnknownKey("Fooooo".into())),
            (" Fooooo ", Error::UnknownKey("Fooooo".into())),
        ];

        for (seq, expected) in tests {
            assert_eq!(seq.parse::<KeySeq>(), Err(expected), "seq={seq:?}");
        }
    }
}
