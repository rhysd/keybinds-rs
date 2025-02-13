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
            KeyCode::Char(c) => Self::Char(c.to_ascii_lowercase()),
            KeyCode::Null => Self::Unidentified,
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
        if from.contains(KeyModifiers::SHIFT) {
            to |= Mods::SHIFT;
        }
        if from.contains(KeyModifiers::ALT) {
            to |= Mods::ALT;
        }
        #[cfg(not(target_os = "macos"))]
        if from.contains(KeyModifiers::SUPER) {
            to |= Mods::WIN;
        }
        #[cfg(target_os = "macos")]
        if from.contains(KeyModifiers::SUPER) {
            to |= Mods::CMD;
        }
        if from.contains(KeyModifiers::META) {
            to |= Mods::CMD;
        }
        to
    }
}

impl From<KeyEvent> for KeyInput {
    fn from(event: KeyEvent) -> Self {
        let key = event.code.into();
        let mods = event.modifiers.into();
        Self { key, mods }
    }
}

impl From<Event> for KeyInput {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(event) => event.into(),
            _ => Key::Unidentified.into(),
        }
    }
}
