//! Support for [`crossterm`] crate.
//!
//! This module provides the conversions from crossterm's event types to [`Key`], [`Mods`],
//! and [`KeyInput`].
//!
//! ```no_run
//! use crossterm::event::{read, Event};
//! use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
//! use keybinds::{KeyInput, KeybindDispatcher};
//! use std::io;
//!
//! #[derive(PartialEq, Eq, Debug)]
//! enum Action {
//!     SayHi,
//!     Exit,
//! }
//!
//! let mut dispatcher = KeybindDispatcher::default();
//! dispatcher.bind("h i", Action::SayHi).unwrap();
//! dispatcher.bind("Ctrl+x Ctrl+c", Action::Exit).unwrap();
//!
//! enable_raw_mode().unwrap();
//!
//! while let Ok(event) = read() {
//!     if let Event::Key(event) = event {
//!         // Convert crossterm's `KeyEvent` into `KeyInput`
//!         println!("Key input `{:?}`\r", KeyInput::from(event));
//!
//!         // `KeybindDispatcher::dispatch` accepts crossterm's `KeyEvent`
//!         if let Some(action) = dispatcher.dispatch(event) {
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
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MediaKeyCode};

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
            KeyCode::F(i) => Self::F(i),
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
        if from.contains(KeyModifiers::ALT) {
            to |= Mods::ALT;
        }
        if from.contains(KeyModifiers::SUPER) {
            to |= Mods::SUPER;
        }
        if from.contains(KeyModifiers::META) {
            to |= Mods::CMD;
        }
        to
    }
}

impl From<&KeyEvent> for KeyInput {
    fn from(event: &KeyEvent) -> Self {
        let key: Key = event.code.into();
        let mods = event.modifiers.into();
        Self::new(key, mods)
    }
}

impl From<KeyEvent> for KeyInput {
    fn from(event: KeyEvent) -> Self {
        Self::from(&event)
    }
}

impl From<&Event> for KeyInput {
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
    use crossterm::event::{KeyEventKind, KeyEventState, ModifierKeyCode};

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
        assert_eq!(Key::from(KeyCode::F(12)), Key::F(12));
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
            Mods::CTRL | Mods::ALT | Mods::CMD,
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
