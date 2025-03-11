//! Support for [`crossterm`] crate.
//!
//! This module provides the conversions from crossterm's event types to [`Key`], [`Mods`],
//! and [`KeyInput`].
//!
//! ```no_run
//! use crossterm::event::{read, Event};
//! use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
//! use keybinds::{KeyInput, Keybinds};
//! use std::io;
//!
//! #[derive(PartialEq, Eq, Debug)]
//! enum Action {
//!     SayHi,
//!     Exit,
//! }
//!
//! let mut keybinds = Keybinds::default();
//! keybinds.bind("h i", Action::SayHi).unwrap();
//! keybinds.bind("Ctrl+x Ctrl+c", Action::Exit).unwrap();
//!
//! enable_raw_mode().unwrap();
//!
//! while let Ok(event) = read() {
//!     if let Event::Key(event) = event {
//!         // Convert crossterm's `KeyEvent` into `KeyInput`
//!         println!("Key input `{:?}`\r", KeyInput::from(event));
//!
//!         // `Keybinds::dispatch` accepts crossterm's `KeyEvent`
//!         if let Some(action) = keybinds.dispatch(event) {
//!             match action {
//!                 Action::SayHi => println!("Hi!"),
//!                 Action::Exit => break,
//!             }
//!         }
//!     }
//! }
//!
//! disable_raw_mode().unwrap();
//! ```
use crate::{Key, KeyInput, Mods};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MediaKeyCode};

impl From<KeyCode> for Key {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Enter => Self::Enter,
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Home => Self::Home,
            KeyCode::End => Self::End,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::Tab => Self::Tab,
            KeyCode::BackTab => Self::Backtab,
            KeyCode::Delete => Self::Delete,
            KeyCode::Insert => Self::Insert,
            KeyCode::F(1) => Self::F1,
            KeyCode::F(2) => Self::F2,
            KeyCode::F(3) => Self::F3,
            KeyCode::F(4) => Self::F4,
            KeyCode::F(5) => Self::F5,
            KeyCode::F(6) => Self::F6,
            KeyCode::F(7) => Self::F7,
            KeyCode::F(8) => Self::F8,
            KeyCode::F(9) => Self::F9,
            KeyCode::F(10) => Self::F10,
            KeyCode::F(11) => Self::F11,
            KeyCode::F(12) => Self::F12,
            KeyCode::F(13) => Self::F13,
            KeyCode::F(14) => Self::F14,
            KeyCode::F(15) => Self::F15,
            KeyCode::F(16) => Self::F16,
            KeyCode::F(17) => Self::F17,
            KeyCode::F(18) => Self::F18,
            KeyCode::F(19) => Self::F19,
            KeyCode::F(20) => Self::F20,
            KeyCode::F(21) => Self::F21,
            KeyCode::F(22) => Self::F22,
            KeyCode::F(23) => Self::F23,
            KeyCode::F(24) => Self::F24,
            KeyCode::F(25) => Self::F25,
            KeyCode::F(26) => Self::F26,
            KeyCode::F(27) => Self::F27,
            KeyCode::F(28) => Self::F28,
            KeyCode::F(29) => Self::F29,
            KeyCode::F(30) => Self::F30,
            KeyCode::F(31) => Self::F31,
            KeyCode::F(32) => Self::F32,
            KeyCode::F(33) => Self::F33,
            KeyCode::F(34) => Self::F34,
            KeyCode::F(35) => Self::F35,
            KeyCode::Char(c) => Self::Char(c),
            KeyCode::Esc => Self::Esc,
            KeyCode::ScrollLock => Self::ScrollLock,
            KeyCode::NumLock => Self::NumLock,
            KeyCode::PrintScreen => Self::PrintScreen,
            KeyCode::Menu => Self::Menu,
            KeyCode::Media(MediaKeyCode::Play) => Self::Play,
            KeyCode::Media(MediaKeyCode::Pause) => Self::Pause,
            KeyCode::Media(MediaKeyCode::PlayPause) => Self::PlayPause,
            KeyCode::Media(MediaKeyCode::Stop) => Self::Stop,
            KeyCode::Media(MediaKeyCode::Rewind) => Self::Rewind,
            KeyCode::Media(MediaKeyCode::TrackNext) => Self::NextTrack,
            KeyCode::Media(MediaKeyCode::TrackPrevious) => Self::PrevTrack,
            KeyCode::Media(MediaKeyCode::LowerVolume) => Self::VolumeDown,
            KeyCode::Media(MediaKeyCode::RaiseVolume) => Self::VolumeUp,
            KeyCode::Media(MediaKeyCode::MuteVolume) => Self::Mute,
            KeyCode::Modifier(_) | KeyCode::Null => Self::Ignored,
            _ => Self::Unidentified,
        }
    }
}

impl From<KeyModifiers> for Mods {
    fn from(from: KeyModifiers) -> Self {
        let mut to = Mods::NONE;
        if from.contains(KeyModifiers::CONTROL) {
            to |= Mods::CTRL;
        }
        if from.intersects(KeyModifiers::ALT | KeyModifiers::META) {
            to |= Mods::ALT;
        }
        if from.contains(KeyModifiers::SUPER) {
            to |= Mods::SUPER;
        }
        if from.contains(KeyModifiers::SHIFT) {
            to |= Mods::SHIFT;
        }
        to
    }
}

impl From<&KeyEvent> for KeyInput {
    /// Convert crossterm's key events to [`KeyInput`]. The key release events are converted into `Key::Ignored` with no
    /// modifiers.
    fn from(event: &KeyEvent) -> Self {
        if event.kind == KeyEventKind::Release {
            return Key::Ignored.into();
        }
        Self::new(event.code, event.modifiers)
    }
}

impl From<KeyEvent> for KeyInput {
    fn from(event: KeyEvent) -> Self {
        Self::from(&event)
    }
}

impl From<&Event> for KeyInput {
    /// Convert crossterm's events to [`KeyInput`]. Events unrelated to key presses are converted into `Key::Ignored`
    /// with no modifiers.
    fn from(event: &Event) -> Self {
        match event {
            Event::Key(event) => event.into(),
            _ => Key::Ignored.into(),
        }
    }
}

impl From<Event> for KeyInput {
    fn from(event: Event) -> Self {
        Self::from(&event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEventState, ModifierKeyCode};

    #[test]
    fn convert_key_code() {
        assert_eq!(Key::from(KeyCode::Backspace), Key::Backspace);
        assert_eq!(Key::from(KeyCode::Char('a')), Key::Char('a'));
        assert_eq!(Key::from(KeyCode::Char('A')), Key::Char('A'));
        assert_eq!(Key::from(KeyCode::KeypadBegin), Key::Unidentified);
        assert_eq!(Key::from(KeyCode::Null), Key::Ignored);
        assert_eq!(
            Key::from(KeyCode::Modifier(ModifierKeyCode::LeftControl)),
            Key::Ignored,
        );
        assert_eq!(Key::from(KeyCode::Media(MediaKeyCode::Play)), Key::Play);
        assert_eq!(Key::from(KeyCode::F(12)), Key::F12);
    }

    #[test]
    fn convert_modifiers() {
        assert_eq!(Mods::from(KeyModifiers::NONE), Mods::NONE);
        assert_eq!(
            Mods::from(
                KeyModifiers::CONTROL
                    | KeyModifiers::SHIFT
                    | KeyModifiers::ALT
                    | KeyModifiers::META
            ),
            Mods::CTRL | Mods::ALT | Mods::SHIFT,
        );
        assert_eq!(Mods::from(KeyModifiers::SUPER), Mods::SUPER);
    }

    #[test]
    fn convert_key_event() {
        assert_eq!(
            KeyInput::from(KeyEvent {
                code: KeyCode::Char('A'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }),
            KeyInput::new('A', Mods::CTRL),
        );
        assert_eq!(
            KeyInput::from(KeyEvent {
                code: KeyCode::Char('A'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
                kind: KeyEventKind::Repeat,
                state: KeyEventState::NONE,
            }),
            KeyInput::new('A', Mods::CTRL),
        );
        assert_eq!(
            KeyInput::from(KeyEvent {
                code: KeyCode::Char('A'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
                kind: KeyEventKind::Release,
                state: KeyEventState::NONE,
            }),
            KeyInput::new(Key::Ignored, Mods::NONE),
        );
    }

    #[test]
    fn convert_event() {
        assert_eq!(
            KeyInput::from(Event::Key(KeyEvent {
                code: KeyCode::Char('A'),
                modifiers: KeyModifiers::CONTROL | KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            })),
            KeyInput::new('A', Mods::CTRL),
        );
        assert_eq!(
            KeyInput::from(Event::FocusGained),
            KeyInput::new(Key::Ignored, Mods::NONE),
        );
    }
}
