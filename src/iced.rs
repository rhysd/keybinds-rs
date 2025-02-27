//! Support for [`iced`] crate.
//!
//! This module provides the conversions from iced's event or key types to [`Key`], [`Mods`], and
//! [`KeyInput`].
//!
//! Put [`KeybindDispatcher`][crate::KeybindDispatcher] as a part of state of your application and
//! dispatch the action in the `update` method. Key events can be subscribed as [`iced::Subscription`].
//!
//! ```no_run
//! use keybinds::KeybindDispatcher;
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
//!     keybinds: KeybindDispatcher<Action>,
//! }
//!
//! impl Default for App {
//!     fn default() -> Self {
//!         let mut keybinds = KeybindDispatcher::default();
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
                Named::F1 => Self::F(1),
                Named::F2 => Self::F(2),
                Named::F3 => Self::F(3),
                Named::F4 => Self::F(4),
                Named::F5 => Self::F(5),
                Named::F6 => Self::F(6),
                Named::F7 => Self::F(7),
                Named::F8 => Self::F(8),
                Named::F9 => Self::F(9),
                Named::F10 => Self::F(10),
                Named::F11 => Self::F(11),
                Named::F12 => Self::F(12),
                Named::F13 => Self::F(13),
                Named::F14 => Self::F(14),
                Named::F15 => Self::F(15),
                Named::F16 => Self::F(16),
                Named::F17 => Self::F(17),
                Named::F18 => Self::F(18),
                Named::F19 => Self::F(19),
                Named::F20 => Self::F(20),
                Named::F21 => Self::F(21),
                Named::F22 => Self::F(22),
                Named::F23 => Self::F(23),
                Named::F24 => Self::F(24),
                Named::F25 => Self::F(25),
                Named::F26 => Self::F(26),
                Named::F27 => Self::F(27),
                Named::F28 => Self::F(28),
                Named::F29 => Self::F(29),
                Named::F30 => Self::F(30),
                Named::F31 => Self::F(31),
                Named::F32 => Self::F(32),
                Named::F33 => Self::F(33),
                Named::F34 => Self::F(34),
                Named::F35 => Self::F(35),
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
