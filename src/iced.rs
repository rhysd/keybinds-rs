//! Support for [`iced`] crate.
//!
//! This module provides the conversions from iced's event or key types to [`Key`], [`Mods`], and
//! [`KeyInput`].
//!
//! Put [`Keybinds`][crate::Keybinds] as a part of state of your application and
//! dispatch the action in the `update` method. Key events can be subscribed as [`iced::Subscription`].
//!
//! ```no_run
//! use keybinds::Keybinds;
//! use iced::event::listen_with;
//! use iced::{keyboard, Event, Element, Subscription, Task};
//!
//! // Actions dispatched by the key bindings
//! enum Action {
//!     SayHello,
//!     Exit,
//! }
//!
//! #[derive(Debug)]
//! enum Message {
//!     KeyEvent(keyboard::Event)
//! }
//!
//! struct App {
//!     keybinds: Keybinds<Action>,
//! }
//!
//! impl Default for App {
//!     fn default() -> Self {
//!         let mut keybinds = Keybinds::default();
//!
//!         // Define the key bindings
//!         keybinds.bind("H e l l o", Action::SayHello).unwrap();
//!         keybinds.bind("Mod+q", Action::Exit).unwrap();
//!
//!         Self { keybinds }
//!     }
//! }
//!
//! impl App {
//!     fn update(&mut self, message: Message) -> Task<Message> {
//!         match message {
//!             Message::KeyEvent(event) => {
//!                 // Dispatch an action from the key event and handle it
//!                 if let Some(action) = self.keybinds.dispatch(event) {
//!                     match action {
//!                         Action::SayHello => println!("Hello!"),
//!                         Action::Exit => return iced::exit(),
//!                     }
//!                 }
//!             }
//!         }
//!         Task::none()
//!     }
//!
//!     fn view(&self) -> Element<Message> {
//!         todo!("TODO: Build UI of your application")
//!     }
//!
//!     fn subscription(&self) -> Subscription<Message> {
//!         // Subscribe events and send keyboard events as message
//!         listen_with(|event, _, _| match event {
//!             Event::Keyboard(event) => Some(Message::KeyEvent(event)),
//!             _ => None,
//!         })
//!     }
//! }
//!
//! iced::run("My App", App::update, App::view).unwrap();
//! ```
use crate::{Key, KeyInput, Mods};
use iced::keyboard::key::Named;
use iced::keyboard::{Event as KeyEvent, Key as IcedKey, Modifiers};
use iced::Event;

impl From<&IcedKey> for Key {
    fn from(key: &IcedKey) -> Self {
        match key {
            IcedKey::Character(s) => {
                let mut chars = s.chars();
                if let (Some(c), None) = (chars.next(), chars.next()) {
                    Self::Char(c)
                } else {
                    Self::Unidentified
                }
            }
            IcedKey::Named(named) => match named {
                Named::Space => Self::Char(' '),
                Named::ArrowUp => Self::Up,
                Named::ArrowRight => Self::Right,
                Named::ArrowDown => Self::Down,
                Named::ArrowLeft => Self::Left,
                Named::Enter => Self::Enter,
                Named::Backspace => Self::Backspace,
                Named::Delete => Self::Delete,
                Named::Home => Self::Home,
                Named::End => Self::End,
                Named::PageUp => Self::PageUp,
                Named::PageDown => Self::PageDown,
                Named::Escape => Self::Esc,
                Named::Tab => Self::Tab,
                Named::Insert => Self::Insert,
                Named::Copy => Self::Copy,
                Named::Cut => Self::Cut,
                Named::Paste => Self::Paste,
                Named::Clear => Self::Clear,
                Named::Undo => Self::Undo,
                Named::Redo => Self::Redo,
                Named::Help => Self::Help,
                Named::ZoomIn => Self::ZoomIn,
                Named::ZoomOut => Self::ZoomOut,
                Named::ZoomToggle => Self::ZoomToggle,
                Named::ScrollLock => Self::ScrollLock,
                Named::NumLock => Self::NumLock,
                Named::PrintScreen => Self::PrintScreen,
                Named::ContextMenu => Self::Menu,
                Named::MediaPlay => Self::Play,
                Named::MediaPause => Self::Pause,
                Named::MediaPlayPause => Self::PlayPause,
                Named::MediaStop => Self::Stop,
                Named::MediaRewind => Self::Rewind,
                Named::MediaTrackNext => Self::NextTrack,
                Named::MediaTrackPrevious => Self::PrevTrack,
                Named::AudioVolumeUp => Self::VolumeUp,
                Named::AudioVolumeDown => Self::VolumeDown,
                Named::AudioVolumeMute => Self::Mute,
                Named::F1 => Self::F1,
                Named::F2 => Self::F2,
                Named::F3 => Self::F3,
                Named::F4 => Self::F4,
                Named::F5 => Self::F5,
                Named::F6 => Self::F6,
                Named::F7 => Self::F7,
                Named::F8 => Self::F8,
                Named::F9 => Self::F9,
                Named::F10 => Self::F10,
                Named::F11 => Self::F11,
                Named::F12 => Self::F12,
                Named::F13 => Self::F13,
                Named::F14 => Self::F14,
                Named::F15 => Self::F15,
                Named::F16 => Self::F16,
                Named::F17 => Self::F17,
                Named::F18 => Self::F18,
                Named::F19 => Self::F19,
                Named::F20 => Self::F20,
                Named::F21 => Self::F21,
                Named::F22 => Self::F22,
                Named::F23 => Self::F23,
                Named::F24 => Self::F24,
                Named::F25 => Self::F25,
                Named::F26 => Self::F26,
                Named::F27 => Self::F27,
                Named::F28 => Self::F28,
                Named::F29 => Self::F29,
                Named::F30 => Self::F30,
                Named::F31 => Self::F31,
                Named::F32 => Self::F32,
                Named::F33 => Self::F33,
                Named::F34 => Self::F34,
                Named::F35 => Self::F35,
                Named::Alt
                | Named::Control
                | Named::Shift
                | Named::Super
                | Named::Hyper
                | Named::Meta
                | Named::Symbol => Self::Ignored,
                _ => Self::Unidentified,
            },
            IcedKey::Unidentified => Self::Unidentified,
        }
    }
}

impl From<IcedKey> for Key {
    fn from(key: IcedKey) -> Self {
        Self::from(&key)
    }
}

impl From<&Modifiers> for Mods {
    /// Convert iced's [`Modifiers`] into keybinds' [`Mods`].
    ///
    /// ```
    /// use keybinds::Mods;
    /// use iced::keyboard::Modifiers;
    ///
    /// assert_eq!(Mods::from(Modifiers::CTRL), Mods::CTRL);
    /// assert_eq!(Mods::from(Modifiers::ALT), Mods::ALT);
    /// assert_eq!(Mods::from(Modifiers::LOGO), Mods::SUPER);
    /// assert_eq!(Mods::from(Modifiers::SHIFT), Mods::SHIFT);
    /// ```
    fn from(from: &Modifiers) -> Self {
        let mut to = Mods::NONE;
        if from.contains(Modifiers::CTRL) {
            to |= Mods::CTRL;
        }
        if from.contains(Modifiers::ALT) {
            to |= Mods::ALT;
        }
        if from.contains(Modifiers::LOGO) {
            to |= Mods::SUPER;
        }
        if from.contains(Modifiers::SHIFT) {
            to |= Mods::SHIFT;
        }
        to
    }
}

impl From<Modifiers> for Mods {
    fn from(mods: Modifiers) -> Self {
        Self::from(&mods)
    }
}

impl From<&KeyEvent> for KeyInput {
    /// Convert iced's key events to [`KeyInput`]. Events except for key presses are converted into `Key::Ignored` with
    /// no modifiers. Note that <kbd>Shift</kbd> modifier is removed when the pressed key is unnamed following the
    /// [syntax](https://github.com/rhysd/keybinds-rs/blob/main/doc/binding_syntax.md).
    ///
    /// ```
    /// use keybinds::{KeyInput, Mods};
    /// use iced::keyboard::{Event, Modifiers, Key};
    ///
    /// // Key event for Ctrl+Shift+X
    /// let event = Event::KeyPressed {
    ///     key: Key::Character("x".into()),
    ///     modified_key: Key::Character("X".into()),
    ///     modifiers: Modifiers::CTRL | Modifiers::SHIFT,
    ///     // ...
    /// #   location: iced::keyboard::Location::Standard,
    /// #   text: None,
    /// #   physical_key: iced::keyboard::key::Physical::Code(iced::keyboard::key::Code::KeyX),
    /// };
    /// // `Mods::SHIFT` is removed because 'X' is already modified by Shift key
    /// assert_eq!(KeyInput::from(event), KeyInput::new('X', Mods::CTRL));
    ///
    /// // Events other than key presses are ignored
    /// let event = Event::KeyReleased {
    ///     // ...
    /// #   key: Key::Character("x".into()),
    /// #   modifiers: Modifiers::CTRL | Modifiers::SHIFT,
    /// #   location: iced::keyboard::Location::Standard,
    /// };
    /// assert_eq!(KeyInput::from(event), KeyInput::from(keybinds::Key::Ignored));
    /// ```
    fn from(event: &KeyEvent) -> Self {
        match event {
            KeyEvent::KeyPressed {
                modified_key,
                modifiers,
                ..
            } => Self::new(modified_key, modifiers),
            _ => Key::Ignored.into(),
        }
    }
}

impl From<KeyEvent> for KeyInput {
    fn from(event: KeyEvent) -> Self {
        Self::from(&event)
    }
}

impl From<&Event> for KeyInput {
    /// Convert iced's events to [`KeyInput`]. Events unrelated to key presses are converted into `Key::Ignored` with
    /// no modifiers.
    fn from(event: &Event) -> Self {
        match event {
            Event::Keyboard(event) => event.into(),
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
    use iced::keyboard::key::{Code, Physical};
    use iced::keyboard::Location;
    use iced::window::Event as WindowEvent;

    #[test]
    fn key_to_key() {
        assert_eq!(Key::from(IcedKey::Character("+".into())), Key::Char('+'));
        assert_eq!(Key::from(IcedKey::Named(Named::Space)), Key::Char(' '));
        assert_eq!(Key::from(IcedKey::Named(Named::ArrowUp)), Key::Up);
        assert_eq!(Key::from(IcedKey::Named(Named::Control)), Key::Ignored);
        assert_eq!(Key::from(IcedKey::Unidentified), Key::Unidentified);
        assert_eq!(Key::from(IcedKey::Named(Named::Compose)), Key::Unidentified);
    }

    #[test]
    fn modifiers_to_mods() {
        assert_eq!(Mods::from(Modifiers::empty()), Mods::NONE);
        assert_eq!(Mods::from(Modifiers::CTRL), Mods::CTRL);
        assert_eq!(
            Mods::from(Modifiers::CTRL | Modifiers::ALT),
            Mods::CTRL | Mods::ALT,
        );
        assert_eq!(Mods::from(Modifiers::LOGO), Mods::SUPER);
        assert_eq!(Mods::from(Modifiers::SHIFT), Mods::SHIFT);
    }

    #[test]
    fn key_event_to_input() {
        assert_eq!(
            KeyInput::from(KeyEvent::KeyPressed {
                key: IcedKey::Character("x".into()),
                modified_key: IcedKey::Character("X".into()),
                physical_key: Physical::Code(Code::KeyX),
                location: Location::Standard,
                modifiers: Modifiers::SHIFT,
                text: Some("X".into()),
            }),
            KeyInput::from(Key::Char('X')),
        );
        assert_eq!(
            KeyInput::from(KeyEvent::KeyReleased {
                key: IcedKey::Character("x".into()),
                location: Location::Standard,
                modifiers: Modifiers::CTRL,
            }),
            KeyInput::from(Key::Ignored),
        );
    }

    #[test]
    fn event_to_input() {
        assert_eq!(
            KeyInput::from(Event::Keyboard(KeyEvent::KeyPressed {
                key: IcedKey::Character("x".into()),
                modified_key: IcedKey::Character("X".into()),
                physical_key: Physical::Code(Code::KeyX),
                location: Location::Standard,
                modifiers: Modifiers::SHIFT,
                text: Some("X".into()),
            })),
            KeyInput::from(Key::Char('X')),
        );
        assert_eq!(
            KeyInput::from(Event::Window(WindowEvent::Closed)),
            KeyInput::from(Key::Ignored),
        );
    }
}
