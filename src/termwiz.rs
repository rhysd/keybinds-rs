//! Support for [`termwiz`] crate.
//!
//! This module provides the conversions from termwiz's event types to [`Key`], [`Mods`],
//! and [`KeyInput`].
//!
//! ```no_run
//! use keybinds::{KeyInput, Keybinds};
//! use termwiz::caps::Capabilities;
//! use termwiz::terminal::buffered::BufferedTerminal;
//! use termwiz::terminal::{new_terminal, Terminal};
//!
//! // Actions dispatched by key bindings
//! enum Action {
//!     SayHi,
//!     ExitApp,
//! }
//!
//! // Create a key bindings dispatcher to dispatch actions for upcoming key inputs
//! let mut keybinds = Keybinds::default();
//!
//! // Key bindings to dispatch the actions
//! keybinds.bind("h i", Action::SayHi).unwrap();
//! keybinds.bind("Ctrl+x Ctrl+c", Action::ExitApp).unwrap();
//!
//! let caps = Capabilities::new_from_env().unwrap();
//! let terminal = new_terminal(caps).unwrap();
//!
//! let mut buf = BufferedTerminal::new(terminal).unwrap();
//! buf.flush().unwrap();
//! buf.terminal().set_raw_mode().unwrap();
//!
//! loop {
//!     let Some(input) = buf.terminal().poll_input(None).unwrap() else {
//!         continue;
//!     };
//!
//!     // Conversion from `InputEvent` to `KeyInput`
//!     buf.add_change(format!("{:?}\r\n", KeyInput::from(&input)));
//!
//!     // Dispatch action by directly passing `InputEvent` to `dispatch` method.
//!     let action = keybinds.dispatch(&input);
//!
//!     if let Some(action) = action {
//!         match action {
//!             Action::SayHi => {
//!                 buf.add_change("Hi!\r\n");
//!             }
//!             Action::ExitApp => break,
//!         }
//!     }
//!
//!     buf.flush().unwrap();
//! }
//! ```
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
        if mods.contains(Modifiers::SHIFT)
            || mods.contains(Modifiers::LEFT_SHIFT)
            || mods.contains(Modifiers::RIGHT_SHIFT)
        {
            ret |= Mods::SHIFT;
        }
        ret
    }
}

impl From<&KeyEvent> for KeyInput {
    fn from(event: &KeyEvent) -> Self {
        Self::new(event.key, event.modifiers)
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
        assert_eq!(Key::from(KeyCode::Char('A')), Key::Char('A'));
        assert_eq!(Key::from(KeyCode::UpArrow), Key::Up);
        assert_eq!(Key::from(KeyCode::Control), Key::Ignored);
        assert_eq!(Key::from(KeyCode::Sleep), Key::Unidentified);
    }

    #[test]
    fn convert_mods() {
        assert_eq!(Mods::from(Modifiers::CTRL), Mods::CTRL);
        assert_eq!(Mods::from(Modifiers::LEFT_CTRL), Mods::CTRL);
        assert_eq!(
            Mods::from(Modifiers::CTRL | Modifiers::SUPER | Modifiers::ALT | Modifiers::SHIFT),
            Mods::CTRL | Mods::SUPER | Mods::ALT | Mods::SHIFT,
        );
        assert_eq!(
            Mods::from(Modifiers::LEFT_CTRL | Modifiers::LEFT_ALT),
            Mods::CTRL | Mods::ALT,
        );
    }

    #[test]
    fn convert_key_event() {
        let actual = KeyInput::from(KeyEvent {
            key: KeyCode::Char('A'),
            modifiers: Modifiers::CTRL | Modifiers::ALT,
        });
        let expected = KeyInput::new('A', Mods::CTRL | Mods::ALT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn convert_input_event() {
        let input = KeyInput::from(InputEvent::Key(KeyEvent {
            key: KeyCode::Char('A'),
            modifiers: Modifiers::CTRL | Modifiers::ALT,
        }));
        assert_eq!(input, KeyInput::new('A', Mods::CTRL | Mods::ALT));

        let input = KeyInput::from(InputEvent::Resized { cols: 80, rows: 24 });
        assert_eq!(input, KeyInput::from(Key::Ignored));
    }
}
