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

impl Key {
    pub fn is_named(self) -> bool {
        match self {
            Self::Char(c) if c == ' ' || c == '+' => true,
            Self::Char(_) | Self::Ignored | Self::Unidentified => false,
            _ => true,
        }
    }
}

impl From<char> for Key {
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_ascii();
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
        const SHIFT = 0b00010000;
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
        match s.trim_ascii() {
            "Control" | "control" | "CONTROL" | "Ctrl" | "ctrl" | "CTRL" => Ok(Self::CTRL),
            "Command" | "command" | "COMMAND" | "Cmd" | "cmd" | "CMD" => Ok(Self::CMD),
            "Mod" | "mod" | "MOD" => Ok(Self::MOD),
            "Alt" | "alt" | "ALT" | "Option" | "option" | "OPTION" => Ok(Self::ALT),
            "Super" | "super" | "SUPER" => Ok(Self::SUPER),
            "Shift" | "shift" | "SHIFT" => Ok(Self::SHIFT),
            "" => Err(Error::EmptyModifier),
            _ => Err(Error::UnknownModifier(s.into())),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct KeyInput {
    key: Key,
    mods: Mods,
}

impl KeyInput {
    pub fn new<K: Into<Key>>(key: K, mut mods: Mods) -> Self {
        let key = key.into();
        if !key.is_named() {
            mods.remove(Mods::SHIFT); // Ensure the invariant
        }
        KeyInput { key, mods }
    }

    pub fn key(&self) -> Key {
        self.key
    }

    pub fn mods(&self) -> Mods {
        self.mods
    }
}

impl FromStr for KeyInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.trim_ascii().split('+');
        let mut cur = s.next().unwrap(); // Iterator by `.split()` is never empty
        let mut mods = Mods::NONE;
        loop {
            if let Some(next) = s.next() {
                mods |= cur.parse()?;
                cur = next;
            } else {
                let key: Key = cur.parse()?;
                if mods.contains(Mods::SHIFT) && !key.is_named() {
                    return Err(Error::ShiftUnavailable(key));
                }
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

#[derive(Clone, Eq, Debug)]
pub enum KeySeq {
    Multiple(Vec<KeyInput>),
    Single(KeyInput),
}

// Consider that `KeySeq::Multiple(vec!['a'.into()])` should be equal to `KeySeq::Single('a'.into())`
impl PartialEq for KeySeq {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl KeySeq {
    pub fn match_to(&self, inputs: &[KeyInput]) -> Match {
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
        let mut keys = s.split_ascii_whitespace();
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
            ("Mod+x", KeyInput::new('x', Mods::MOD)),
            ("Super+x", KeyInput::new('x', Mods::SUPER)),
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
            ("Shift+Up", KeyInput::new(Key::Up, Mods::SHIFT)),
            (
                "Ctrl+Shift+F7",
                KeyInput::new(Key::F(7), Mods::SHIFT | Mods::CTRL),
            ),
            ("Shift+Plus", KeyInput::new('+', Mods::SHIFT)),
            ("Shift+Space", KeyInput::new(' ', Mods::SHIFT)),
            ("　", KeyInput::new('　', Mods::NONE)),
            ("Ctrl+　", KeyInput::new('　', Mods::CTRL)),
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
            ("Shift+a", Error::ShiftUnavailable(Key::Char('a'))),
            ("Ctrl+Shift+A", Error::ShiftUnavailable(Key::Char('A'))),
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
            (
                "　 Ctrl+　",
                KeySeq::from(vec![
                    KeyInput::new('　', Mods::NONE),
                    KeyInput::new('　', Mods::CTRL),
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

    #[test]
    fn conversions() {
        for (actual, expected) in [
            (Key::from('a'), Key::Char('a')),
            (Key::from('あ'), Key::Char('あ')),
        ] {
            assert_eq!(actual, expected);
        }

        for (actual, expected) in [
            (
                KeyInput::from('a'),
                KeyInput {
                    key: Key::Char('a'),
                    mods: Mods::NONE,
                },
            ),
            (
                KeyInput::from(Key::Enter),
                KeyInput {
                    key: Key::Enter,
                    mods: Mods::NONE,
                },
            ),
        ] {
            assert_eq!(actual, expected);
        }

        for (actual, expected) in [
            (
                KeySeq::from('a'),
                KeySeq::Single(KeyInput {
                    key: Key::Char('a'),
                    mods: Mods::NONE,
                }),
            ),
            (
                KeySeq::from(Key::Enter),
                KeySeq::Single(KeyInput {
                    key: Key::Enter,
                    mods: Mods::NONE,
                }),
            ),
            (
                KeySeq::from(vec![KeyInput::from('x')]),
                KeySeq::Single(KeyInput {
                    key: Key::Char('x'),
                    mods: Mods::NONE,
                }),
            ),
            (
                KeySeq::from(vec!['x'.into(), 'y'.into()]),
                KeySeq::Multiple(vec![
                    KeyInput {
                        key: Key::Char('x'),
                        mods: Mods::NONE,
                    },
                    KeyInput {
                        key: Key::Char('y'),
                        mods: Mods::NONE,
                    },
                ]),
            ),
            (
                KeySeq::from(KeyInput::new(Key::Enter, Mods::CTRL)),
                KeySeq::Single(KeyInput {
                    key: Key::Enter,
                    mods: Mods::CTRL,
                }),
            ),
        ] {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn keyseq_eq() {
        use KeySeq::*;
        assert_eq!(Multiple(vec!['a'.into()]), Single('a'.into()));
        assert_eq!(Single('a'.into()), Multiple(vec!['a'.into()]));
        assert_ne!(Multiple(vec!['a'.into()]), Single('b'.into()));
        assert_ne!(Single('a'.into()), Multiple(vec!['b'.into()]));
    }

    #[test]
    fn key_is_named() {
        assert!(!Key::Char('a').is_named());
        assert!(!Key::Char('(').is_named());
        assert!(!Key::Char('あ').is_named());
        assert!(!Key::Ignored.is_named());
        assert!(!Key::Unidentified.is_named());
        assert!(Key::Up.is_named());
        assert!(Key::Enter.is_named());
        // Edge cases
        assert!(Key::Char(' ').is_named());
        assert!(Key::Char('+').is_named());
    }

    #[test]
    fn unnamed_key_with_shift() {
        let k = KeyInput::new(Key::Up, Mods::SHIFT);
        assert_eq!(k.key(), Key::Up);
        assert_eq!(k.mods(), Mods::SHIFT);

        let k = KeyInput::new('a', Mods::SHIFT);
        assert_eq!(k.key(), Key::Char('a'));
        assert_eq!(k.mods(), Mods::NONE);

        let k = KeyInput::new('X', Mods::SHIFT | Mods::CTRL);
        assert_eq!(k.key(), Key::Char('X'));
        assert_eq!(k.mods(), Mods::CTRL);
    }
}
