use crate::{Key, KeyInput, Mods};
use termwiz::input::{InputEvent, KeyCode, KeyEvent, Modifiers};

impl From<KeyCode> for Key {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Char(c) => Self::Char(c),
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
