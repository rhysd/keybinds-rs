//! Support for [`winit`] crate.
//!
//! This module provides:
//!
//! - the conversion from winit's key and modifier types to [`Key`] and [`Mods`]
//! - [`WinitEventConverter`] struct to track the modifier state and converts key events to [`KeyInput`]
//!
//! ```no_run
//! use keybinds::winit::WinitEventConverter;
//! use keybinds::Keybinds;
//! use winit::application::ApplicationHandler;
//! use winit::event::{Event, WindowEvent};
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
//!     keybinds: Keybinds<Action>,
//!     converter: WinitEventConverter,
//! }
//!
//! impl Default for App {
//!     fn default() -> Self {
//!         // Create a key bindings dispatcher to dispatch actions for upcoming key inputs
//!         let mut keybinds = Keybinds::default();
//!
//!         // Key bindings to dispatch the actions
//!         keybinds.bind("h i", Action::SayHi).unwrap();
//!         keybinds.bind("Mod+q", Action::Exit).unwrap();
//!
//!         Self {
//!             window: None,
//!             keybinds,
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
//!         if let Some(action) = self.keybinds.dispatch(input) {
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
                NamedKey::Help => Self::Help,
                NamedKey::ZoomIn => Self::ZoomIn,
                NamedKey::ZoomOut => Self::ZoomOut,
                NamedKey::ZoomToggle => Self::ZoomToggle,
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
                NamedKey::F1 => Self::F1,
                NamedKey::F2 => Self::F2,
                NamedKey::F3 => Self::F3,
                NamedKey::F4 => Self::F4,
                NamedKey::F5 => Self::F5,
                NamedKey::F6 => Self::F6,
                NamedKey::F7 => Self::F7,
                NamedKey::F8 => Self::F8,
                NamedKey::F9 => Self::F9,
                NamedKey::F10 => Self::F10,
                NamedKey::F11 => Self::F11,
                NamedKey::F12 => Self::F12,
                NamedKey::F13 => Self::F13,
                NamedKey::F14 => Self::F14,
                NamedKey::F15 => Self::F15,
                NamedKey::F16 => Self::F16,
                NamedKey::F17 => Self::F17,
                NamedKey::F18 => Self::F18,
                NamedKey::F19 => Self::F19,
                NamedKey::F20 => Self::F20,
                NamedKey::F21 => Self::F21,
                NamedKey::F22 => Self::F22,
                NamedKey::F23 => Self::F23,
                NamedKey::F24 => Self::F24,
                NamedKey::F25 => Self::F25,
                NamedKey::F26 => Self::F26,
                NamedKey::F27 => Self::F27,
                NamedKey::F28 => Self::F28,
                NamedKey::F29 => Self::F29,
                NamedKey::F30 => Self::F30,
                NamedKey::F31 => Self::F31,
                NamedKey::F32 => Self::F32,
                NamedKey::F33 => Self::F33,
                NamedKey::F34 => Self::F34,
                NamedKey::F35 => Self::F35,
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
        if state.contains(ModifiersState::SHIFT) {
            mods |= Mods::SHIFT;
        }
        mods
    }
}

impl From<ModifiersState> for Mods {
    fn from(state: ModifiersState) -> Self {
        Self::from(&state)
    }
}

impl From<&Modifiers> for Mods {
    fn from(mods: &Modifiers) -> Self {
        Self::from(mods.state())
    }
}

/// Trait to handle various kinds of winit's event values in a uniform way.
///
/// The types that implements this trait can be passed to [`WinitEventConverter::convert`] method call.
pub trait WinitEvent {
    /// Convert the event into [`KeyInput`] instance with the [`WinitEventConverter`] instance. The converter manages
    /// the current modifiers state.
    fn to_key_input(&self, conv: &mut WinitEventConverter) -> KeyInput;
}

impl WinitEvent for KeyEvent {
    fn to_key_input(&self, conv: &mut WinitEventConverter) -> KeyInput {
        KeyInput::new(Key::from(&self.logical_key), conv.mods)
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

/// Event converter to convert winit's events into [`KeyInput`] with managing the current modifiers state.
///
/// The following winit's event types can be converted.
/// - [`winit::event::Event`]
/// - [`winit::event::WindowEvent`]
/// - [`winit::event::KeyEvent`]
///
/// ```
/// use winit::event::{Event, WindowEvent};
/// use winit::window::WindowId;
/// use winit::keyboard::ModifiersState;
/// use keybinds::{Key, Mods, KeyInput};
/// use keybinds::winit::{WinitEventConverter, WinitEvent};
///
/// let mut converter = WinitEventConverter::default();
///
/// # struct Dummy(winit::keyboard::Key);
/// # impl WinitEvent for Dummy {
/// #     fn to_key_input(&self, conv: &mut WinitEventConverter) -> KeyInput {
/// #         KeyInput::new(keybinds::Key::from(&self.0), conv.mods())
/// #     }
/// # }
/// // Receive 'x' key press event
/// // let event = ...
/// # let event = Dummy(winit::keyboard::Key::Character("x".into()));
/// assert_eq!(converter.convert(&event), KeyInput::new('x', Mods::NONE));
///
/// // Receive a modifiers state change event
/// let event = Event::<()>::WindowEvent {
///     window_id: WindowId::dummy(),
///     event: WindowEvent::ModifiersChanged(ModifiersState::CONTROL.into()),
/// };
/// assert_eq!(converter.convert(&event), KeyInput::new(Key::Ignored, Mods::NONE));
///
/// // Receive 'x' key press event again. The current modifier state is reflected
/// // to the converted `KeyInput` instance
/// // let event = ...
/// # let event = Dummy(winit::keyboard::Key::Character("x".into()));
/// assert_eq!(converter.convert(&event), KeyInput::new('x', Mods::CTRL));
/// ```
#[derive(Default)]
pub struct WinitEventConverter {
    mods: Mods,
}

impl WinitEventConverter {
    /// Returns the current modifiers state.
    ///
    /// ```
    /// use winit::event::{Event, WindowEvent};
    /// use winit::window::WindowId;
    /// use winit::keyboard::ModifiersState;
    /// use keybinds::Mods;
    /// use keybinds::winit::WinitEventConverter;
    ///
    /// let mut converter = WinitEventConverter::default();
    ///
    /// // Initially no modifiers are being pressed.
    /// assert_eq!(converter.mods(), Mods::NONE);
    ///
    /// // Receive a modifiers state changed event here.
    /// let event = Event::<()>::WindowEvent {
    ///     window_id: WindowId::dummy(),
    ///     event: WindowEvent::ModifiersChanged(ModifiersState::CONTROL.into()),
    /// };
    /// converter.convert(&event);
    ///
    /// // The CTRL modifier is being pressed.
    /// assert_eq!(converter.mods(), Mods::CTRL);
    /// ```
    pub fn mods(&self) -> Mods {
        self.mods
    }

    /// Update the current modifiers state. This method needs to be called only when you pass winit's `KeyEvent` to the
    /// `convert` method. Otherwise, when you pass `Event` or `WindowEvent`, this method is implicitly called while
    /// converting them into [`KeyInput`].
    ///
    /// ```
    /// use winit::keyboard::ModifiersState;
    /// use keybinds::Mods;
    /// use keybinds::winit::WinitEventConverter;
    ///
    /// let mut converter = WinitEventConverter::default();
    ///
    /// assert_eq!(converter.mods(), Mods::NONE);
    ///
    /// let state = ModifiersState::CONTROL.into();
    /// converter.on_modifiers_changed(&state);
    ///
    /// assert_eq!(converter.mods(), Mods::CTRL);
    /// ```
    pub fn on_modifiers_changed(&mut self, mods: &Modifiers) {
        self.mods = mods.into();
    }

    /// Convert winit's events into [`KeyInput`] instances with managing the current modifiers state. See the document
    /// for [`WinitEventConverter`] for an example.
    pub fn convert<E: WinitEvent>(&mut self, event: &E) -> KeyInput {
        event.to_key_input(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::keyboard::NativeKey;
    use NamedKey::*;
    use WinitKey::*;

    #[test]
    fn convert_key() {
        assert_eq!(Key::from(Named(Space)), Key::Char(' '));
        assert_eq!(Key::from(Named(ArrowUp)), Key::Up);
        assert_eq!(Key::from(Named(F1)), Key::F1);
        assert_eq!(Key::from(Named(Control)), Key::Ignored);
        assert_eq!(Key::from(Named(TVInput)), Key::Unidentified);
        assert_eq!(Key::from(Character("a".into())), Key::Char('a'));
        assert_eq!(Key::from(Character("A".into())), Key::Char('A'));
        assert_eq!(Key::from(Character("foo".into())), Key::Unidentified);
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
            Mods::from(ModifiersState::CONTROL | ModifiersState::ALT | ModifiersState::SHIFT),
            Mods::CTRL | Mods::ALT | Mods::SHIFT,
        );
        assert_eq!(Mods::from(ModifiersState::SUPER), Mods::SUPER);
    }

    // Unformatunately `WinitEventConverter::convert` is not testable because winit does not provide
    // to create an instance of `KeyEvent`. It provides no constructor and contains some pvivate fields.
    // Instead, we use `winit::keyboard::Key` directly.
    impl WinitEvent for WinitKey {
        fn to_key_input(&self, conv: &mut WinitEventConverter) -> KeyInput {
            let key: Key = self.into();
            KeyInput::new(key, conv.mods)
        }
    }

    #[test]
    fn converter_convert_keys() {
        let mut conv = WinitEventConverter::default();

        assert_eq!(conv.mods(), Mods::NONE);

        assert_eq!(conv.convert(&Named(Space)), KeyInput::new(' ', Mods::NONE));
        assert_eq!(
            conv.convert(&Character("(".into())),
            KeyInput::new('(', Mods::NONE),
        );
        assert_eq!(
            conv.convert(&Character("A".into())),
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

        conv.on_modifiers_changed(&ModifiersState::CONTROL.into());

        assert_eq!(
            conv.convert(&Character("x".into())),
            KeyInput::new('x', Mods::CTRL),
        );
    }
}
