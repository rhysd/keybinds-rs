use crate::Error;
use bitflags::bitflags;
use smallvec::{smallvec, SmallVec};
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "arbitrary")]
use arbitrary::Arbitrary;

// Note: We use `Key::F1`...`Key::F35` variants instead of `Key::F(u8)` variant because
//  * it reduces the size of `Key` from 8 bytes to 4 bytes because `u8` value requires a padding. Thanks to this
//    reduction, `KeyInput` fits to 1 word and can implement `Copy`.
//  * it can avoid invalid keys like `Key::F(0)` or `Key::F(999)`.

/// Single logical key on keyboard.
///
/// The 'logical key' is the key after applying modifier keys. For example, `Key::Char('A')` usually means the result
/// of pressing <kbd>Shift</kbd> + <kbd>A</kbd> physical keys.
///
/// This enum is non-exhaustive because more keys may be added in the future.
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
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
    /// Special virtual key for keys which are not identified.
    Unidentified,
    /// Special virtual key for ignoring the key input. This key is completely ignored by a key binding dispatcher.
    Ignored,
}

impl Key {
    /// Returns true when it is a named key such as "Up". As an edge case, the space key and the "+" key are also
    /// treated as named keys following the [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md)
    /// although they are instances of `Key::Char` variant.
    ///
    /// ```
    /// use keybinds::Key;
    ///
    /// assert!(Key::Up.is_named());
    /// assert!(Key::Copy.is_named());
    /// assert!(Key::Insert.is_named());
    /// assert!(Key::Enter.is_named());
    /// assert!(Key::Home.is_named());
    /// assert!(Key::F1.is_named());
    /// assert!(Key::Char(' ').is_named());
    /// assert!(Key::Char('+').is_named());
    /// assert!(!Key::Char('x').is_named());
    /// assert!(!Key::Unidentified.is_named());
    /// ```
    pub fn is_named(self) -> bool {
        match self {
            Self::Char(' ' | '+') => true,
            Self::Char(_) | Self::Ignored | Self::Unidentified => false,
            _ => true,
        }
    }
}

impl From<char> for Key {
    /// Convert the character to the single character key.
    ///
    /// ```
    /// use keybinds::Key;
    ///
    /// let key: Key = 'X'.into();
    /// assert_eq!(key, Key::Char('X'));
    /// ```
    fn from(c: char) -> Self {
        Self::Char(c)
    }
}

impl FromStr for Key {
    type Err = Error;

    /// Parse the key from [`str`] following the [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// ```
    /// use keybinds::Key;
    ///
    /// assert_eq!("x".parse(), Ok(Key::Char('x')));
    /// assert_eq!("Up".parse(), Ok(Key::Up));
    /// assert_eq!("Enter".parse(), Ok(Key::Enter));
    /// assert_eq!("Space".parse(), Ok(Key::Char(' ')));
    /// assert_eq!("Plus".parse(), Ok(Key::Char('+')));
    /// assert_eq!("F1".parse(), Ok(Key::F1));
    ///
    /// assert!("Unknown".parse::<Key>().is_err());
    /// assert!("".parse::<Key>().is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_ascii();
        {
            let mut c = s.chars();
            if let (Some(c), None) = (c.next(), c.next()) {
                return Ok(Self::Char(c));
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
            "numlock" | "NumLock" | "NUMLOCK" => Ok(Self::NumLock),
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
            "f1" | "F1" => Ok(Self::F1),
            "f2" | "F2" => Ok(Self::F2),
            "f3" | "F3" => Ok(Self::F3),
            "f4" | "F4" => Ok(Self::F4),
            "f5" | "F5" => Ok(Self::F5),
            "f6" | "F6" => Ok(Self::F6),
            "f7" | "F7" => Ok(Self::F7),
            "f8" | "F8" => Ok(Self::F8),
            "f9" | "F9" => Ok(Self::F9),
            "f10" | "F10" => Ok(Self::F10),
            "f11" | "F11" => Ok(Self::F11),
            "f12" | "F12" => Ok(Self::F12),
            "f13" | "F13" => Ok(Self::F13),
            "f14" | "F14" => Ok(Self::F14),
            "f15" | "F15" => Ok(Self::F15),
            "f16" | "F16" => Ok(Self::F16),
            "f17" | "F17" => Ok(Self::F17),
            "f18" | "F18" => Ok(Self::F18),
            "f19" | "F19" => Ok(Self::F19),
            "f20" | "F20" => Ok(Self::F20),
            "f21" | "F21" => Ok(Self::F21),
            "f22" | "F22" => Ok(Self::F22),
            "f23" | "F23" => Ok(Self::F23),
            "f24" | "F24" => Ok(Self::F24),
            "f25" | "F25" => Ok(Self::F25),
            "f26" | "F26" => Ok(Self::F26),
            "f27" | "F27" => Ok(Self::F27),
            "f28" | "F28" => Ok(Self::F28),
            "f29" | "F29" => Ok(Self::F29),
            "f30" | "F30" => Ok(Self::F30),
            "f31" | "F31" => Ok(Self::F31),
            "f32" | "F32" => Ok(Self::F32),
            "f33" | "F33" => Ok(Self::F33),
            "f34" | "F34" => Ok(Self::F34),
            "f35" | "F35" => Ok(Self::F35),
            "" => Err(Error::EmptyKey),
            _ => Err(Error::UnknownKey(s.into())),
        }
    }
}

impl fmt::Display for Key {
    /// Generate a string representation of the key following the [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// ```
    /// use keybinds::Key;
    ///
    /// assert_eq!(format!("{}", Key::Char('X')), "X");
    /// assert_eq!(format!("{}", Key::Down), "Down");
    /// assert_eq!(format!("{}", Key::Insert), "Insert");
    /// assert_eq!(format!("{}", Key::F5), "F5");
    /// assert_eq!(format!("{}", Key::Char(' ')), "Space");
    /// assert_eq!(format!("{}", Key::Char('+')), "Plus");
    /// ```
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
            Self::F1 => f.write_str("F1"),
            Self::F2 => f.write_str("F2"),
            Self::F3 => f.write_str("F3"),
            Self::F4 => f.write_str("F4"),
            Self::F5 => f.write_str("F5"),
            Self::F6 => f.write_str("F6"),
            Self::F7 => f.write_str("F7"),
            Self::F8 => f.write_str("F8"),
            Self::F9 => f.write_str("F9"),
            Self::F10 => f.write_str("F10"),
            Self::F11 => f.write_str("F11"),
            Self::F12 => f.write_str("F12"),
            Self::F13 => f.write_str("F13"),
            Self::F14 => f.write_str("F14"),
            Self::F15 => f.write_str("F15"),
            Self::F16 => f.write_str("F16"),
            Self::F17 => f.write_str("F17"),
            Self::F18 => f.write_str("F18"),
            Self::F19 => f.write_str("F19"),
            Self::F20 => f.write_str("F20"),
            Self::F21 => f.write_str("F21"),
            Self::F22 => f.write_str("F22"),
            Self::F23 => f.write_str("F23"),
            Self::F24 => f.write_str("F24"),
            Self::F25 => f.write_str("F25"),
            Self::F26 => f.write_str("F26"),
            Self::F27 => f.write_str("F27"),
            Self::F28 => f.write_str("F28"),
            Self::F29 => f.write_str("F29"),
            Self::F30 => f.write_str("F30"),
            Self::F31 => f.write_str("F31"),
            Self::F32 => f.write_str("F32"),
            Self::F33 => f.write_str("F33"),
            Self::F34 => f.write_str("F34"),
            Self::F35 => f.write_str("F35"),
            Self::Unidentified => f.write_str("Unidentified"),
            Self::Ignored => f.write_str("Ignored"),
        }
    }
}

bitflags! {
    /// Modifier keys such as "Ctrl".
    ///
    /// `NONE` means nothing is pressed. These constants are bitfields so use `|` for representing to press multiple
    /// modifiers at once.
    ///
    /// ```
    /// use keybinds::Mods;
    ///
    /// // No modifiers
    /// let none = Mods::NONE;
    /// // Ctrl + Alt
    /// let ctrl_alt = Mods::CTRL | Mods::ALT;
    ///
    /// assert_ne!(none, ctrl_alt);
    /// ```
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
    /// The "Mod" modifier key. It is equivalent to "Cmd" on macOS and "Ctrl" on other platforms.
    ///
    /// ```
    /// use keybinds::Mods;
    ///
    /// #[cfg(target_os = "macos")]
    /// assert_eq!(Mods::MOD, Mods::CMD);
    /// #[cfg(not(target_os = "macos"))]
    /// assert_eq!(Mods::MOD, Mods::CTRL);
    /// ```
    #[cfg(not(target_os = "macos"))]
    pub const MOD: Self = Self::CTRL;
    /// The "Mod" modifier key. It is equivalent to "Cmd" on macOS and "Ctrl" on other platforms.
    ///
    /// ```
    /// use keybinds::Mods;
    ///
    /// #[cfg(target_os = "macos")]
    /// assert_eq!(Mods::MOD, Mods::CMD);
    /// #[cfg(not(target_os = "macos"))]
    /// assert_eq!(Mods::MOD, Mods::CTRL);
    /// ```
    #[cfg(target_os = "macos")]
    pub const MOD: Self = Self::CMD;
    /// The "Super" modifier key. It is equivalent to "Cmd" on macOS and "Win" on other platforms.
    ///
    /// ```
    /// use keybinds::Mods;
    ///
    /// #[cfg(target_os = "macos")]
    /// assert_eq!(Mods::SUPER, Mods::CMD);
    /// #[cfg(not(target_os = "macos"))]
    /// assert_eq!(Mods::SUPER, Mods::WIN);
    /// ```
    #[cfg(not(target_os = "macos"))]
    pub const SUPER: Self = Self::WIN;
    /// The "Super" modifier key. It is equivalent to "Cmd" on macOS and "Win" on other platforms.
    ///
    /// ```
    /// use keybinds::Mods;
    ///
    /// #[cfg(target_os = "macos")]
    /// assert_eq!(Mods::SUPER, Mods::CMD);
    /// #[cfg(not(target_os = "macos"))]
    /// assert_eq!(Mods::SUPER, Mods::WIN);
    /// ```
    #[cfg(target_os = "macos")]
    pub const SUPER: Self = Self::CMD;
}

impl FromStr for Mods {
    type Err = Error;

    /// Parse the modifier key from [`str`] following the [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// ```
    /// use keybinds::Mods;
    ///
    /// assert_eq!("Ctrl".parse(), Ok(Mods::CTRL));
    /// assert_eq!("Cmd".parse(), Ok(Mods::CMD));
    /// assert_eq!("Alt".parse(), Ok(Mods::ALT));
    /// assert_eq!("Mod".parse(), Ok(Mods::MOD));
    /// assert_eq!("Super".parse(), Ok(Mods::SUPER));
    ///
    /// // Aliases
    /// assert_eq!("Control".parse(), Ok(Mods::CTRL));
    /// assert_eq!("Command".parse(), Ok(Mods::CMD));
    /// assert_eq!("Option".parse(), Ok(Mods::ALT));
    ///
    /// // Error cases
    /// assert!("Fooo".parse::<Mods>().is_err());
    /// assert!("".parse::<Mods>().is_err());
    /// ```
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
    /// Generate a string representation of the modifier key following the [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// When multiple modifiers are pressed at once they are joined with "+". When no modifier key is pressed, it generates an empty string.
    ///
    /// ```
    /// use keybinds::Mods;
    ///
    /// assert_eq!(format!("{}", Mods::CTRL), "Ctrl");
    /// assert_eq!(format!("{}", Mods::CTRL | Mods::CMD | Mods::ALT), "Ctrl+Cmd+Alt");
    /// assert_eq!(format!("{}", Mods::NONE), "");
    /// ```
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

/// Single key input by pressing a key and modifiers.
///
/// This struct is equivalent to a key combination in the [syntax document](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md)
/// such as "Ctrl+x".
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[cfg_attr(feature = "arbitrary", derive(Arbitrary))]
pub struct KeyInput {
    key: Key,
    mods: Mods,
}

impl KeyInput {
    /// Create a new [`KeyInput`] instance with checking the <kbd>Shift</kbd> modifier restriction described in the
    /// [syntax document](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// ```
    /// use keybinds::{KeyInput, Key, Mods};
    ///
    /// let k = KeyInput::new('x', Mods::CTRL);
    /// assert_eq!(k.key(), Key::Char('x'));
    /// assert_eq!(k.mods(), Mods::CTRL);
    ///
    /// let k = KeyInput::new(Key::Enter, Mods::MOD);
    /// assert_eq!(k.key(), Key::Enter);
    /// assert_eq!(k.mods(), Mods::MOD);
    ///
    /// // Shift modifier is removed when it is not used with named keys following the restriction.
    /// let k = KeyInput::new('x', Mods::SHIFT | Mods::CTRL);
    /// assert_eq!(k.key(), Key::Char('x'));
    /// assert_eq!(k.mods(), Mods::CTRL);
    ///
    /// // You need to use the following instead.
    /// let k = KeyInput::new('X', Mods::CTRL);
    /// assert_eq!(k.key(), Key::Char('X'));
    /// assert_eq!(k.mods(), Mods::CTRL);
    /// ```
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

    /// Return the [`Key`] of the input.
    pub fn key(&self) -> Key {
        self.key
    }

    /// Return the [`Mods`] of the input.
    pub fn mods(&self) -> Mods {
        self.mods
    }
}

impl FromStr for KeyInput {
    type Err = Error;

    /// Parse the key input from [`str`] following the [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// ```
    /// use keybinds::{Key, Mods, KeyInput};
    ///
    /// assert_eq!("a".parse(), Ok(KeyInput::new('a', Mods::NONE)));
    /// assert_eq!("Ctrl+x".parse(), Ok(KeyInput::new('x', Mods::CTRL)));
    /// assert_eq!("Alt+Shift+Enter".parse(), Ok(KeyInput::new(Key::Enter, Mods::ALT | Mods::SHIFT)));
    ///
    /// assert!("".parse::<KeyInput>().is_err());
    /// assert!("Foooo".parse::<KeyInput>().is_err());
    /// assert!("Shift+x".parse::<KeyInput>().is_err()); // Violates Shift modifier invariant
    /// ```
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
    /// Convert a single key with no modifiers into [`KeyInput`].
    ///
    /// ```
    /// use keybinds::{KeyInput, Mods};
    ///
    /// assert_eq!(KeyInput::from('x'), KeyInput::new('x', Mods::NONE));
    /// ```
    fn from(k: K) -> Self {
        Self::new(k.into(), Mods::NONE)
    }
}

impl fmt::Display for KeyInput {
    /// Generate a string representation of the key input following the
    /// [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// ```
    /// use keybinds::{Key, Mods, KeyInput};
    ///
    /// assert_eq!(format!("{}", KeyInput::new('x', Mods::CTRL)), "Ctrl+x");
    /// assert_eq!(
    ///     format!("{}", KeyInput::new(Key::Enter, Mods::SHIFT | Mods::ALT)),
    ///     "Alt+Shift+Enter",
    /// );
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mods != Mods::NONE {
            write!(f, "{}+", self.mods)?;
        }
        write!(f, "{}", self.key)
    }
}

/// The result of [`KeySeq::match_to`] to match a key sequence to key inputs.
#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
pub enum Match {
    /// The key inputs completely matched to the key sequence.
    Matched,
    /// The key inupts were a prefix of the key sequence. This means the matching is still ongoing.
    Prefix,
    /// The key inputs did not match to the key sequence.
    Unmatch,
}

/// The key sequence bound to some action. It consists of one or more [`KeyInput`] instances.
///
/// This type represents a key sequence in the [syntax document](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md)
/// such as "Ctrl+x Ctrl+s".
///
/// A key sequence usually consists of a single key input or two key inputs, except for complex key bindings like Vim
/// style. This type is allocated on stack until it has two inputs. When it has more inputs, they are spilled onto the
/// heap.
///
/// ```
/// use keybinds::{KeySeq, KeyInput, Key, Mods};
///
/// let mut seq = KeySeq::from([KeyInput::new('x', Mods::CTRL), 'a'.into()]);
///
/// // Add more elements
/// seq.push('b'.into());
///
/// // Modify the inner slice
/// seq.as_mut_slice()[2] = Key::Enter.into();
///
/// // Access the inner slice
/// assert_eq!(seq.as_slice().len(), 3);
/// ```
///
/// More elements can be added by [`KeySeq::push`], [`KeySeq::insert`], or [`KeySeq::extend`]. There is no API to
/// remove elements for now because no use case has been revealed. However you can do it by converting the key
/// sequence into [`Vec`].
///
/// ```
/// use keybinds::{KeySeq, KeyInput};
///
/// let seq: KeySeq = ['a', 'b', 'c'].into_iter().collect();
///
/// let mut vec: Vec<_> = seq.as_slice().iter().copied().collect();
/// vec.remove(1);
///
/// let seq: KeySeq = vec.into_iter().collect();
///
/// assert_eq!(seq.as_slice(), &[KeyInput::from('a'), KeyInput::from('c')]);
/// ```
///
#[derive(Clone, PartialEq, Eq, Default, Hash, Debug)]
pub struct KeySeq(SmallVec<[KeyInput; 2]>);

impl KeySeq {
    /// Match the given inputs to the key sequence. The result [`Match`] is one of following cases:
    ///
    /// - the key sequence completely matched the input
    /// - the input was a prefix of the key sequence. This means the matching is still ongoing
    /// - the key sequence didn't match the input
    ///
    /// ```
    /// use keybinds::{KeySeq, Match};
    ///
    /// let seq = KeySeq::from(['x', 'y', 'z']);
    ///
    /// let matched = ['x'.into(), 'y'.into(), 'z'.into()];
    /// let ongoing_1 = ['x'.into()];
    /// let ongoing_2 = ['x'.into(), 'y'.into()];
    /// let unmatch_1 = ['x'.into(), 'y'.into(), 'a'.into()];
    /// let unmatch_2 = ['y'.into(), 'z'.into()];
    ///
    /// assert_eq!(seq.match_to(&matched), Match::Matched);
    /// assert_eq!(seq.match_to(&ongoing_1), Match::Prefix);
    /// assert_eq!(seq.match_to(&ongoing_2), Match::Prefix);
    /// assert_eq!(seq.match_to(&unmatch_1), Match::Unmatch);
    /// assert_eq!(seq.match_to(&unmatch_2), Match::Unmatch);
    /// ```
    pub fn match_to(&self, inputs: &[KeyInput]) -> Match {
        let mut ls = self.0.iter();
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

    /// Get the key sequence as a slice.
    ///
    /// ```
    /// use keybinds::{KeySeq, KeyInput};
    ///
    /// let seq: KeySeq = ['a', 'b'].into_iter().collect();
    ///
    /// assert_eq!(seq.as_slice(), &[KeyInput::from('a'), KeyInput::from('b')]);
    /// ```
    pub fn as_slice(&self) -> &[KeyInput] {
        self.0.as_slice()
    }

    /// Mutably borrow the inner slice.
    ///
    /// ```
    /// use keybinds::{KeySeq, KeyInput};
    ///
    /// let mut seq: KeySeq = ['a', 'b'].into_iter().collect();
    ///
    /// seq.as_mut_slice()[1] = 'x'.into();
    ///
    /// assert_eq!(seq.as_slice(), &[KeyInput::from('a'), KeyInput::from('x')]);
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [KeyInput] {
        self.0.as_mut_slice()
    }

    /// Push the input to the end of the key sequence. This method is useful to build a key sequence conditionally.
    ///
    /// ```
    /// use keybinds::{KeySeq, KeyInput, Mods};
    ///
    /// struct Config {
    ///     prefix_key: Option<KeyInput>,
    /// }
    ///
    /// // let config = ...
    /// # let config = Config { prefix_key: Some('t'.into()) };
    ///
    /// let mut seq = KeySeq::default();
    ///
    /// if let Some(key) = config.prefix_key {
    ///     seq.push(key);
    /// }
    ///
    /// seq.extend([
    ///     KeyInput::new('x', Mods::CTRL),
    ///     KeyInput::new('c', Mods::CTRL),
    /// ]);
    ///
    /// let len = seq.as_slice().len();
    /// assert!(len == 2 || len == 3);
    /// ```
    pub fn push(&mut self, input: KeyInput) {
        self.0.push(input);
    }

    /// Insert the input at the index of the key sequence. This method is useful to insert some prefix key after
    /// building the sequence.
    ///
    /// ```
    /// use keybinds::{KeySeq, KeyInput, Mods};
    ///
    /// let mut seq: KeySeq = ['a', 'b'].into_iter().collect();
    /// let prefix = KeyInput::new('x', Mods::CTRL);
    ///
    /// seq.insert(0, prefix);
    ///
    /// assert_eq!(seq.as_slice(), &[prefix, 'a'.into(), 'b'.into()]);
    /// ```
    pub fn insert(&mut self, idx: usize, input: KeyInput) {
        self.0.insert(idx, input);
    }
}

impl FromStr for KeySeq {
    type Err = Error;

    /// Parse a key sequence from [`str`] following the [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// This method expects at least one key in the sequence. When the sequence is invalid such as unknown keys or
    /// empty input, this method returns an error.
    ///
    /// ```
    /// use keybinds::{KeySeq, KeyInput, Key, Mods};
    ///
    /// assert_eq!("x".parse(), Ok(KeySeq::from(['x'])));
    /// assert_eq!(
    ///     "Ctrl+Up Alt+Down".parse(),
    ///     Ok(KeySeq::from([
    ///         KeyInput::new(Key::Up, Mods::CTRL),
    ///         KeyInput::new(Key::Down, Mods::ALT),
    ///     ])),
    /// );
    /// assert_eq!(
    ///     "h e l l o".parse(),
    ///     Ok(KeySeq::from(['h', 'e', 'l', 'l', 'o'])),
    /// );
    ///
    /// // Errors
    /// assert!("".parse::<KeySeq>().is_err());       // Empty key sequence
    /// assert!("x Fooo".parse::<KeySeq>().is_err()); // Unknown named key
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: SmallVec<_> = s
            .split_ascii_whitespace()
            .map(|key| key.parse())
            .collect::<Result<_, _>>()?;
        if vec.is_empty() {
            return Err(Error::EmptyKeySequence);
        }
        Ok(Self(vec))
    }
}

impl<I: Into<KeyInput>> From<I> for KeySeq {
    /// Convert a single key input into a key sequence.
    ///
    /// ```
    /// use keybinds::{KeySeq, Key};
    ///
    /// assert_eq!(KeySeq::from('x'), KeySeq::from(['x']));
    /// assert_eq!(KeySeq::from(Key::Enter), KeySeq::from([Key::Enter]));
    /// ```
    fn from(key: I) -> Self {
        Self(smallvec![key.into()])
    }
}

impl<const N: usize, I: Into<KeyInput>> From<[I; N]> for KeySeq {
    /// Convert an array of key inputs into a key sequence.
    ///
    /// ```
    /// use keybinds::{KeySeq, KeyInput, Key, Mods};
    ///
    /// let seq = KeySeq::from([Key::Enter.into(), KeyInput::new('x', Mods::CTRL)]);
    /// let slice = seq.as_slice();
    /// assert_eq!(slice[0].key(), Key::Enter);
    /// assert_eq!(slice[1].mods(), Mods::CTRL);
    /// ```
    fn from(arr: [I; N]) -> Self {
        Self(arr.into_iter().map(Into::into).collect())
    }
}

impl<I: Into<KeyInput>> FromIterator<I> for KeySeq {
    /// Collect a key sequence from an iterator of key inputs.
    ///
    /// ```
    /// use std::iter::repeat;
    /// use keybinds::{KeySeq, KeyInput, Mods};
    ///
    /// let seq: KeySeq = repeat(KeyInput::new('a', Mods::CTRL)).take(3).collect();
    ///
    /// assert_eq!(format!("{seq}"), "Ctrl+a Ctrl+a Ctrl+a");
    /// ```
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = I>,
    {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

impl fmt::Display for KeySeq {
    /// Generate a string representation of the key sequence following the
    /// [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// Key inputs are joined with single spaces. If the sequence is empty, this method writes nothing.
    ///
    /// ```
    /// use keybinds::{KeySeq, KeyInput, Key, Mods};
    ///
    /// let seq = KeySeq::from([
    ///     KeyInput::new('x', Mods::CTRL),
    ///     KeyInput::new(Key::Enter, Mods::ALT),
    /// ]);
    /// assert_eq!(format!("{seq}"), "Ctrl+x Alt+Enter");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut inputs = self.0.iter();
        if let Some(first) = inputs.next() {
            write!(f, "{}", first)?;
            for input in inputs {
                write!(f, " {}", input)?;
            }
        };
        Ok(())
    }
}

impl<I: Into<KeyInput>> Extend<I> for KeySeq {
    /// Extend the key sequence with the iterator of key inputs. See [`KeySeq::push`] for an example.
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = I>,
    {
        self.0.extend(iter.into_iter().map(Into::into));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key_input_ok() {
        let seq: KeySeq = ['a'].into_iter().collect();
        assert!(!seq.as_slice().is_empty());

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
            ("F1", KeyInput::new(Key::F1, Mods::NONE)),
            ("Ctrl+F1", KeyInput::new(Key::F1, Mods::CTRL)),
            ("F20", KeyInput::new(Key::F20, Mods::NONE)),
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
                KeyInput::new(Key::F7, Mods::SHIFT | Mods::CTRL),
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
            ("a b c", KeySeq::from(['a', 'b', 'c'])),
            (
                "Up Down Enter",
                KeySeq::from([Key::Up, Key::Down, Key::Enter]),
            ),
            (
                "Ctrl+Alt+a Super+b Mod+c",
                KeySeq::from([
                    KeyInput::new('a', Mods::ALT | Mods::CTRL),
                    KeyInput::new('b', Mods::SUPER),
                    KeyInput::new('c', Mods::MOD),
                ]),
            ),
            (
                "　 Ctrl+　",
                KeySeq::from([
                    KeyInput::new('　', Mods::NONE),
                    KeyInput::new('　', Mods::CTRL),
                ]),
            ),
            ("　 　 　", KeySeq::from(['　', '　', '　'])),
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
                KeySeq(smallvec![KeyInput {
                    key: Key::Char('a'),
                    mods: Mods::NONE,
                }]),
            ),
            (
                KeySeq::from(Key::Enter),
                KeySeq(smallvec![KeyInput::from(Key::Enter)]),
            ),
            (
                KeySeq::from([KeyInput::from('x')]),
                KeySeq(smallvec![KeyInput::from('x')]),
            ),
            (
                KeySeq::from(['x', 'y']),
                KeySeq(smallvec![KeyInput::from('x'), KeyInput::from('y')]),
            ),
            (
                KeySeq::from(KeyInput::new(Key::Enter, Mods::CTRL)),
                KeySeq(smallvec![KeyInput {
                    key: Key::Enter,
                    mods: Mods::CTRL,
                }]),
            ),
        ] {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn key_seq_as_slice() {
        let mut seq: KeySeq = ['a', 'b', 'c'].into_iter().collect();
        seq.as_mut_slice()[1] = 'x'.into();
        assert_eq!(seq.as_slice(), &['a'.into(), 'x'.into(), 'c'.into()]);
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
    fn display_keyseq() {
        let tests = [
            (KeySeq::from([] as [KeyInput; 0]), ""),
            (KeySeq::from('a'), "a"),
            (KeySeq::from('A'), "A"),
            (KeySeq::from(Key::Up), "Up"),
            (KeySeq::from(Key::F11), "F11"),
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
            (KeySeq::from(['a', 'b']), "a b"),
            (KeySeq::from(['a', 'b', 'c', 'd', 'e']), "a b c d e"),
            (KeySeq::from([Key::Left, Key::Right]), "Left Right"),
            (
                KeySeq::from([
                    KeyInput::new(Key::Left, Mods::SHIFT),
                    KeyInput::new('X', Mods::ALT | Mods::CTRL),
                ]),
                "Shift+Left Ctrl+Alt+X",
            ),
            (KeySeq::from(['　', '　']), "　 　"),
        ];

        for (seq, expected) in tests {
            let actual = format!("{seq}");
            assert_eq!(&actual, expected, "seq={seq:?}");
        }
    }
}
