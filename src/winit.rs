//! Support for [`winit`] crate.
//!
//! This module provides:
//!
//! - the conversion from winit's key and modifier types to [`Key`] and [`Mods`]
//! - [`WinitEventConverter`] struct to track the modifier state and converts key events to [`KeyInput`]
//!
//! ```no_run
//! use keybinds::winit::WinitEventConverter;
//! use keybinds::KeybindDispatcher;
//! use winit::application::ApplicationHandler;
//! use winit::event::WindowEvent;
//! use winit::event_loop::{ActiveEventLoop, EventLoop};
//! use winit::window::{Theme, Window, WindowId};
//!
//! // Actions dispatched by key bindings
//! #[derive(Debug)]
//! enum Action {
//!     SayHi,
//!     Exit,
//! }
//!
//! struct App {
//!     window: Option<Window>,
//!     dispatcher: KeybindDispatcher<Action>,
//!     converter: WinitEventConverter,
//! }
//!
//! impl Default for App {
//!     fn default() -> Self {
//!         let mut dispatcher = KeybindDispatcher::default();
//!
//!         // Key bindings to dispatch the actions
//!         dispatcher.bind("h i", Action::SayHi).unwrap();
//!         dispatcher.bind("Mod+q", Action::Exit).unwrap();
//!
//!         Self {
//!             window: None,
//!             dispatcher,
//!             converter: WinitEventConverter::default(),
//!         }
//!     }
//! }
//!
//! impl ApplicationHandler for App {
//!     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//!         let window = event_loop.create_window(Window::default_attributes()).unwrap();
//!         self.window = Some(window);
//!     }
//!
//!     fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
//!         // Convert the window event into key input
//!         let input = self.converter.convert(&event);
//!
//!         // Check if the converted key input dispatches some action
//!         if let Some(action) = self.dispatcher.dispatch(input) {
//!             match action {
//!                 Action::SayHi => println!("Hi!"),
//!                 Action::Exit => event_loop.exit(),
//!             }
//!         }
//!
//!         if let WindowEvent::CloseRequested = event {
//!             event_loop.exit();
//!         }
//!     }
//! }
//!
//! let event_loop = EventLoop::new().unwrap();
//! event_loop.run_app(&mut App::default()).unwrap();
//! ```
use crate::{Key, KeyInput, Mods};
use winit::event::{ElementState, Event, KeyEvent, Modifiers, WindowEvent};
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

pub trait WinitEvent {
    fn to_key_input(&self, conv: &mut WinitEventConverter) -> KeyInput;
}

impl WinitEvent for KeyEvent {
    fn to_key_input(&self, conv: &mut WinitEventConverter) -> KeyInput {
        KeyInput {
            key: Key::from(&self.logical_key),
            mods: conv.mods,
        }
    }
}

impl WinitEvent for WindowEvent {
    fn to_key_input(&self, conv: &mut WinitEventConverter) -> KeyInput {
        match self {
            WindowEvent::ModifiersChanged(mods) => {
                conv.on_modifiers_changed(mods);
                Key::Ignored.into()
            }
            WindowEvent::KeyboardInput { event, .. } if event.state == ElementState::Pressed => {
                event.to_key_input(conv)
            }
            _ => Key::Ignored.into(),
        }
    }
}

impl<T> WinitEvent for Event<T> {
    fn to_key_input(&self, conv: &mut WinitEventConverter) -> KeyInput {
        if let Event::WindowEvent { event, .. } = self {
            event.to_key_input(conv)
        } else {
            Key::Ignored.into()
        }
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

    pub fn convert<E: WinitEvent>(&mut self, event: &E) -> KeyInput {
        event.to_key_input(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::keyboard::{NativeKey, SmolStr};
    use NamedKey::*;
    use WinitKey::*;

    #[test]
    fn convert_key() {
        assert_eq!(Key::from(Named(Space)), Key::Char(' '));
        assert_eq!(Key::from(Named(ArrowUp)), Key::Up);
        assert_eq!(Key::from(Named(F1)), Key::F(1));
        assert_eq!(Key::from(Named(Control)), Key::Ignored);
        assert_eq!(Key::from(Named(TVInput)), Key::Unidentified);
        assert_eq!(Key::from(Character(SmolStr::new("a"))), Key::Char('a'));
        assert_eq!(Key::from(Character(SmolStr::new("A"))), Key::Char('A'));
        assert_eq!(Key::from(Character(SmolStr::new("foo"))), Key::Unidentified);
        assert_eq!(
            Key::from(Unidentified(NativeKey::Unidentified)),
            Key::Unidentified,
        );
        assert_eq!(Key::from(Dead(None)), Key::Unidentified);
    }

    #[test]
    fn convert_modifiers_state() {
        assert_eq!(Mods::from(ModifiersState::CONTROL), Mods::CTRL);
        assert_eq!(
            Mods::from(ModifiersState::CONTROL | ModifiersState::ALT),
            Mods::CTRL | Mods::ALT,
        );
        assert_eq!(Mods::from(ModifiersState::SUPER), Mods::SUPER);
    }

    // Unformatunately `WinitEventConverter::convert` is not testable because winit does not provide
    // to create an instance of `KeyEvent`. It provides no constructor and contains some pvivate fields.
    // Instead, we use `winit::keyboard::Key` directly.
    impl WinitEvent for WinitKey {
        fn to_key_input(&self, conv: &mut WinitEventConverter) -> KeyInput {
            KeyInput {
                key: self.into(),
                mods: conv.mods,
            }
        }
    }

    #[test]
    fn converter_convert_keys() {
        let mut conv = WinitEventConverter::default();

        // We cannot test `on_modifiers_changed` because `winit::event::Modifiers` does not provide
        // a constructor.
        assert_eq!(conv.mods(), Mods::NONE);

        assert_eq!(conv.convert(&Named(Space)), KeyInput::new(' ', Mods::NONE));
        assert_eq!(
            conv.convert(&Character(SmolStr::new("("))),
            KeyInput::new('(', Mods::NONE),
        );
        assert_eq!(
            conv.convert(&Character(SmolStr::new("A"))),
            KeyInput::new('A', Mods::NONE),
        );
        assert_eq!(
            conv.convert(&Named(TVInputHDMI1)),
            KeyInput::new(Key::Unidentified, Mods::NONE),
        );
        assert_eq!(
            conv.convert(&Named(Control)),
            KeyInput::new(Key::Ignored, Mods::NONE),
        );
    }
}
