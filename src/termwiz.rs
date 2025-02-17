use crate::{Key, KeyInput, Mods};
use termwiz::input::{InputEvent, KeyCode, KeyEvent, Modifiers};

impl From<KeyCode> for Key {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Char(c) => Self::Char(c.to_ascii_lowercase()),
            KeyCode::Hyper
            | KeyCode::Super
            | KeyCode::Meta
            | KeyCode::Control
            | KeyCode::LeftControl
            | KeyCode::RightControl
            | KeyCode::Shift
            | KeyCode::LeftShift
            | KeyCode::RightShift
            | KeyCode::Alt
            | KeyCode::LeftAlt
            | KeyCode::RightAlt
            | KeyCode::LeftWindows
            | KeyCode::RightWindows => Self::Ignored,
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Tab => Self::Tab,
            KeyCode::Clear => Self::Clear,
            KeyCode::Enter => Self::Enter,
            KeyCode::Escape => Self::Esc,
            KeyCode::Menu => Self::Menu,
            KeyCode::LeftMenu => Self::Menu,
            KeyCode::RightMenu => Self::Menu,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::End => Self::End,
            KeyCode::Home => Self::Home,
            KeyCode::LeftArrow => Self::Left,
            KeyCode::RightArrow => Self::Right,
            KeyCode::UpArrow => Self::Up,
            KeyCode::DownArrow => Self::Down,
            KeyCode::PrintScreen => Self::PrintScreen,
            KeyCode::Insert => Self::Insert,
            KeyCode::Delete => Self::Delete,
            KeyCode::Function(i) => Self::F(i),
            KeyCode::NumLock => Self::NumLock,
            KeyCode::ScrollLock => Self::ScrollLock,
            KeyCode::Copy => Self::Copy,
            KeyCode::Cut => Self::Cut,
            KeyCode::Paste => Self::Paste,
            KeyCode::VolumeMute => Self::Mute,
            KeyCode::VolumeDown => Self::VolumeDown,
            KeyCode::VolumeUp => Self::VolumeUp,
            KeyCode::MediaNextTrack => Self::NextTrack,
            KeyCode::MediaPrevTrack => Self::PrevTrack,
            KeyCode::MediaStop => Self::Stop,
            KeyCode::MediaPlayPause => Self::PlayPause,
            _ => Self::Unidentified,
        }
    }
}

impl From<Modifiers> for Mods {
    fn from(mods: Modifiers) -> Self {
        let mut ret = Mods::NONE;
        if mods.contains(Modifiers::SHIFT)
            || mods.contains(Modifiers::RIGHT_SHIFT)
            || mods.contains(Modifiers::LEFT_SHIFT)
        {
            ret |= Mods::SHIFT;
        }
        if mods.contains(Modifiers::CTRL)
            || mods.contains(Modifiers::RIGHT_CTRL)
            || mods.contains(Modifiers::LEFT_CTRL)
        {
            ret |= Mods::CTRL;
        }
        if mods.contains(Modifiers::ALT)
            || mods.contains(Modifiers::RIGHT_ALT)
            || mods.contains(Modifiers::LEFT_ALT)
        {
            ret |= Mods::ALT;
        }
        if mods.contains(Modifiers::SUPER) {
            ret |= Mods::SUPER;
        }
        ret
    }
}

impl From<&KeyEvent> for KeyInput {
    fn from(event: &KeyEvent) -> Self {
        Self {
            key: event.key.into(),
            mods: event.modifiers.into(),
        }
    }
}

impl From<KeyEvent> for KeyInput {
    fn from(event: KeyEvent) -> Self {
        Self::from(&event)
    }
}

impl From<&InputEvent> for KeyInput {
    fn from(event: &InputEvent) -> Self {
        match event {
            InputEvent::Key(event) => event.into(),
            _ => Key::Ignored.into(),
        }
    }
}

impl From<InputEvent> for KeyInput {
    fn from(event: InputEvent) -> Self {
        Self::from(&event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_keycode() {
        assert_eq!(Key::from(KeyCode::Char('a')), Key::Char('a'));
        assert_eq!(Key::from(KeyCode::Char('A')), Key::Char('a'));
        assert_eq!(Key::from(KeyCode::UpArrow), Key::Up);
        assert_eq!(Key::from(KeyCode::Control), Key::Ignored);
        assert_eq!(Key::from(KeyCode::Sleep), Key::Unidentified);
    }

    #[test]
    fn convert_mods() {
        assert_eq!(Mods::from(Modifiers::SHIFT), Mods::SHIFT);
        assert_eq!(Mods::from(Modifiers::LEFT_SHIFT), Mods::SHIFT);
        assert_eq!(
            Mods::from(Modifiers::CTRL | Modifiers::SHIFT | Modifiers::ALT),
            Mods::CTRL | Mods::SHIFT | Mods::ALT,
        );
        assert_eq!(
            Mods::from(Modifiers::LEFT_CTRL | Modifiers::LEFT_SHIFT | Modifiers::LEFT_ALT),
            Mods::CTRL | Mods::SHIFT | Mods::ALT,
        );
        assert_eq!(Mods::from(Modifiers::SUPER), Mods::SUPER);
    }

    #[test]
    fn convert_key_event() {
        let actual = KeyInput::from(KeyEvent {
            key: KeyCode::Char('A'),
            modifiers: Modifiers::CTRL | Modifiers::SHIFT,
        });
        let expected = KeyInput::new('a', Mods::CTRL | Mods::SHIFT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn convert_input_event() {
        let input = KeyInput::from(InputEvent::Key(KeyEvent {
            key: KeyCode::Char('A'),
            modifiers: Modifiers::CTRL | Modifiers::SHIFT,
        }));
        assert_eq!(input, KeyInput::new('a', Mods::CTRL | Mods::SHIFT));

        let input = KeyInput::from(InputEvent::Resized { cols: 80, rows: 24 });
        assert_eq!(input, KeyInput::from(Key::Ignored));
    }
}
