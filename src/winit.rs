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
                NamedKey::Alt
                | NamedKey::Control
                | NamedKey::Shift
                | NamedKey::Super
                | NamedKey::Hyper
                | NamedKey::Meta
                | NamedKey::Symbol => Self::Ignored,
                _ => Self::Unidentified,
            },
            WinitKey::Character(s) => {
                let mut chars = s.chars();
                if let (Some(c), None) = (chars.next(), chars.next()) {
                    Self::Char(c.to_ascii_lowercase())
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

pub trait KeyInputConvertible {
    fn convert_key_input(&self, conv: &mut KeyEventConverter) -> KeyInput;
}

impl KeyInputConvertible for WindowEvent {
    fn convert_key_input(&self, conv: &mut KeyEventConverter) -> KeyInput {
        match self {
            WindowEvent::ModifiersChanged(mods) => {
                conv.on_modifiers_changed(mods);
                Key::Ignored.into()
            }
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                KeyInput {
                    key: Key::from(&event.logical_key),
                    mods: conv.mods,
                }
            }
            _ => Key::Ignored.into(),
        }
    }
}

impl<T> KeyInputConvertible for Event<T> {
    fn convert_key_input(&self, conv: &mut KeyEventConverter) -> KeyInput {
        if let Event::WindowEvent { event, .. } = self {
            event.convert_key_input(conv)
        } else {
            Key::Ignored.into()
        }
    }
}

#[derive(Default)]
pub struct KeyEventConverter {
    mods: Mods,
}

impl KeyEventConverter {
    pub fn mods(&self) -> Mods {
        self.mods
    }

    pub fn on_modifiers_changed(&mut self, mods: &Modifiers) {
        self.mods = mods.state().into();
    }

    pub fn convert<C: KeyInputConvertible>(&mut self, event: &C) -> KeyInput {
        event.convert_key_input(self)
    }
}
