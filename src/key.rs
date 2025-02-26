use crate::Error;
use bitflags::bitflags;
use std::fmt;
use std::slice;
use std::str::FromStr;

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
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

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Char(' ') => f.write_str("Space"),
            Self::Char('+') => f.write_str("Plus"),
            Self::Char(c) => write!(f, "{c}"),
            Self::Up => f.write_str("Up"),
            Self::Right => f.write_str("Right"),
            Self::Down => f.write_str("Down"),
            Self::Left => f.write_str("Left"),
            Self::Enter => f.write_str("Enter"),
            Self::Backspace => f.write_str("Backspace"),
            Self::Delete => f.write_str("Delete"),
            Self::Home => f.write_str("Home"),
            Self::End => f.write_str("End"),
            Self::PageUp => f.write_str("PageUp"),
            Self::PageDown => f.write_str("PageDown"),
            Self::Esc => f.write_str("Esc"),
            Self::Tab => f.write_str("Tab"),
            Self::Backtab => f.write_str("Backtab"),
            Self::Insert => f.write_str("Insert"),
            Self::Copy => f.write_str("Copy"),
            Self::Cut => f.write_str("Cut"),
            Self::Paste => f.write_str("Paste"),
            Self::Clear => f.write_str("Clear"),
            Self::Undo => f.write_str("Undo"),
            Self::Redo => f.write_str("Redo"),
            Self::ZoomIn => f.write_str("ZoomIn"),
            Self::ZoomOut => f.write_str("ZoomOut"),
            Self::ScrollLock => f.write_str("ScrollLock"),
            Self::NumLock => f.write_str("NumLock"),
            Self::FnLock => f.write_str("FnLock"),
            Self::PrintScreen => f.write_str("PrintScreen"),
            Self::Menu => f.write_str("Menu"),
            Self::Play => f.write_str("Play"),
            Self::Pause => f.write_str("Pause"),
            Self::PlayPause => f.write_str("PlayPause"),
            Self::Stop => f.write_str("Stop"),
            Self::Rewind => f.write_str("Rewind"),
            Self::NextTrack => f.write_str("NextTrack"),
            Self::PrevTrack => f.write_str("PrevTrack"),
            Self::VolumeUp => f.write_str("VolumeUp"),
            Self::VolumeDown => f.write_str("VolumeDown"),
            Self::Mute => f.write_str("Mute"),
            Self::F(i) => write!(f, "F{i}"),
            Self::Unidentified => f.write_str("Unidentified"),
            Self::Ignored => f.write_str("Ignored"),
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

impl fmt::Display for Mods {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for (value, name) in [
            (Mods::CTRL, "Ctrl"),
            (Mods::CMD, "Cmd"),
            (Mods::ALT, "Alt"),
            (Mods::WIN, "Win"),
            (Mods::SHIFT, "Shift"),
        ] {
            if self.contains(value) {
                if first {
                    first = false;
                } else {
                    f.write_str("+")?;
                }
                f.write_str(name)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct KeyInput {
    key: Key,
    mods: Mods,
}

impl KeyInput {
    pub fn new<K, M>(key: K, mods: M) -> Self
    where
        K: Into<Key>,
        M: Into<Mods>,
    {
        let key = key.into();
        let mut mods = mods.into();
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

impl fmt::Display for KeyInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mods != Mods::NONE {
            write!(f, "{}+", self.mods)?;
        }
        write!(f, "{}", self.key)
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Match {
    Matched,
    Prefix,
    Unmatch,
}

#[derive(Clone, Eq, Debug)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
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

impl fmt::Display for KeySeq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut inputs = self.as_slice().iter();
        if let Some(first) = inputs.next() {
            write!(f, "{}", first)?;
            for input in inputs {
                write!(f, " {}", input)?;
            }
        };
        Ok(())
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
    fn create_key_with_shift() {
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

    #[test]
    fn display() {
        let tests = [
            (KeySeq::from(vec![]), ""),
            (KeySeq::from('a'), "a"),
            (KeySeq::from('A'), "A"),
            (KeySeq::from(Key::Up), "Up"),
            (KeySeq::from(Key::F(11)), "F11"),
            (KeySeq::from(' '), "Space"),
            (KeySeq::from('+'), "Plus"),
            (KeySeq::from(KeyInput::new('a', Mods::CTRL)), "Ctrl+a"),
            (
                KeySeq::from(KeyInput::new(
                    'a',
                    Mods::CTRL | Mods::CMD | Mods::ALT | Mods::WIN,
                )),
                "Ctrl+Cmd+Alt+Win+a",
            ),
            #[cfg(not(target_os = "macos"))]
            (KeySeq::from(KeyInput::new('a', Mods::MOD)), "Ctrl+a"),
            #[cfg(target_os = "macos")]
            (KeySeq::from(KeyInput::new('a', Mods::MOD)), "Cmd+a"),
            #[cfg(not(target_os = "macos"))]
            (KeySeq::from(KeyInput::new('a', Mods::SUPER)), "Win+a"),
            #[cfg(target_os = "macos")]
            (KeySeq::from(KeyInput::new('a', Mods::SUPER)), "Cmd+a"),
            (
                KeySeq::from(KeyInput::new(Key::Enter, Mods::SHIFT)),
                "Shift+Enter",
            ),
            (
                KeySeq::from(KeyInput::new(Key::Char(' '), Mods::SHIFT)),
                "Shift+Space",
            ),
            (
                KeySeq::from(KeyInput::new(Key::Char('+'), Mods::SHIFT)),
                "Shift+Plus",
            ),
            (KeySeq::from(vec!['a'.into(), 'b'.into()]), "a b"),
            (
                KeySeq::from(vec![
                    'a'.into(),
                    'b'.into(),
                    'c'.into(),
                    'd'.into(),
                    'e'.into(),
                ]),
                "a b c d e",
            ),
            (
                KeySeq::from(vec![Key::Left.into(), Key::Right.into()]),
                "Left Right",
            ),
            (
                KeySeq::from(vec![
                    KeyInput::new(Key::Left, Mods::SHIFT),
                    KeyInput::new('X', Mods::ALT | Mods::CTRL),
                ]),
                "Shift+Left Ctrl+Alt+X",
            ),
        ];

        for (seq, expected) in tests {
            let actual = format!("{seq}");
            assert_eq!(&actual, expected, "seq={seq:?}");
        }
    }
}
