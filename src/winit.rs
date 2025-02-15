use crate::{Key, KeyInput, Mods};
use winit::event::{ElementState, Event, Modifiers, WindowEvent};
use winit::keyboard::{Key as WinitKey, ModifiersState, NamedKey};

impl From<&WinitKey> for Key {
    fn from(key: &WinitKey) -> Self {
        match key {
            WinitKey::Named(named) => match named {
                NamedKey::Space => Self::Char(' '),
                NamedKey::ArrowUp => Self::Up,
                NamedKey::ArrowRight => Self::Right,
                NamedKey::ArrowDown => Self::Down,
                NamedKey::ArrowLeft => Self::Left,
                NamedKey::Enter => Self::Enter,
                NamedKey::Backspace => Self::Backspace,
                NamedKey::Delete => Self::Delete,
                NamedKey::Home => Self::Home,
                NamedKey::End => Self::End,
                NamedKey::PageUp => Self::PageUp,
                NamedKey::PageDown => Self::PageDown,
                NamedKey::Escape => Self::Esc,
                NamedKey::Tab => Self::Tab,
                NamedKey::Insert => Self::Insert,
                NamedKey::Copy => Self::Copy,
                NamedKey::Cut => Self::Cut,
                NamedKey::Paste => Self::Paste,
                NamedKey::Clear => Self::Clear,
                NamedKey::Undo => Self::Undo,
                NamedKey::Redo => Self::Redo,
                NamedKey::ScrollLock => Self::ScrollLock,
                NamedKey::NumLock => Self::NumLock,
                NamedKey::PrintScreen => Self::PrintScreen,
                NamedKey::ContextMenu => Self::Menu,
                NamedKey::MediaPlay => Self::Play,
                NamedKey::MediaPause => Self::Pause,
                NamedKey::MediaPlayPause => Self::PlayPause,
                NamedKey::MediaStop => Self::Stop,
                NamedKey::MediaRewind => Self::Rewind,
                NamedKey::MediaTrackNext => Self::NextTrack,
                NamedKey::MediaTrackPrevious => Self::PrevTrack,
                NamedKey::AudioVolumeUp => Self::VolumeUp,
                NamedKey::AudioVolumeDown => Self::VolumeDown,
                NamedKey::AudioVolumeMute => Self::Mute,
                NamedKey::F1 => Self::F(1),
                NamedKey::F2 => Self::F(2),
                NamedKey::F3 => Self::F(3),
                NamedKey::F4 => Self::F(4),
                NamedKey::F5 => Self::F(5),
                NamedKey::F6 => Self::F(6),
                NamedKey::F7 => Self::F(7),
                NamedKey::F8 => Self::F(8),
                NamedKey::F9 => Self::F(9),
                NamedKey::F10 => Self::F(10),
                NamedKey::F11 => Self::F(11),
                NamedKey::F12 => Self::F(12),
                NamedKey::F13 => Self::F(13),
                NamedKey::F14 => Self::F(14),
                NamedKey::F15 => Self::F(15),
                NamedKey::F16 => Self::F(16),
                NamedKey::F17 => Self::F(17),
                NamedKey::F18 => Self::F(18),
                NamedKey::F19 => Self::F(19),
                NamedKey::F20 => Self::F(20),
                _ => Self::Unidentified,
            },
            WinitKey::Character(s) => {
                let mut chars = s.chars();
                if let (Some(c), None) = (chars.next(), chars.next()) {
                    Self::Char(c)
                } else {
                    Self::Unidentified
                }
            }
            _ => Self::Unidentified,
        }
    }
}

impl From<WinitKey> for Key {
    fn from(key: WinitKey) -> Self {
        Self::from(&key)
    }
}

impl From<&ModifiersState> for Mods {
    fn from(state: &ModifiersState) -> Self {
        let mut mods = Mods::NONE;
        if state.contains(ModifiersState::CONTROL) {
            mods |= Mods::CTRL;
        }
        if state.contains(ModifiersState::SHIFT) {
            mods |= Mods::SHIFT;
        }
        if state.contains(ModifiersState::ALT) {
            mods |= Mods::ALT;
        }
        if state.contains(ModifiersState::SUPER) {
            mods |= Mods::SUPER;
        }
        mods
    }
}

impl From<ModifiersState> for Mods {
    fn from(state: ModifiersState) -> Self {
        Self::from(&state)
    }
}

#[derive(Default)]
pub struct WinitEventConverter {
    mods: Mods,
}

impl WinitEventConverter {
    pub fn mods(&self) -> Mods {
        self.mods
    }

    pub fn on_modifiers_changed(&mut self, mods: &Modifiers) {
        self.mods = mods.state().into();
    }

    pub fn convert_window_event(&mut self, event: &WindowEvent) -> KeyInput {
        match event {
            WindowEvent::ModifiersChanged(mods) => {
                self.on_modifiers_changed(mods);
                Key::Unidentified.into()
            }
            WindowEvent::KeyboardInput { event, .. } if event.state != ElementState::Pressed => {
                KeyInput {
                    key: Key::from(&event.logical_key),
                    mods: self.mods,
                }
            }
            _ => Key::Unidentified.into(),
        }
    }

    pub fn convert_event<T>(&mut self, event: &Event<T>) -> KeyInput {
        if let Event::WindowEvent { event, .. } = event {
            self.convert_window_event(event)
        } else {
            Key::Unidentified.into()
        }
    }
}
