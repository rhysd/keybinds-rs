//! # Overview
//!
//! [keybinds-rs][crates-io] is a small Rust crate to define/parse/dispatch key bindings (keyboard shortcuts).
//!
//! - Provide a syntax to easily define key bindings in a configuration file like `Ctrl+a`
//! - Support key sequences like `Ctrl+x Ctrl+s` for complicated key bindings like Vim style
//! - Support to parse/generate the key bindings configuration using [serde][] optionally
//! - Core API independent from any platforms and frameworks with minimal dependencies (only one crate)
//! - Support several platforms and frameworks as optional features
//!   - [crossterm][]
//!   - [termwiz][]
//!   - [winit][]
//! - Support structral fuzzing using [arbitrary][] optionally.
//!
//! # Minimal example
//!
//! ```
//! use keybinds::{KeybindDispatcher, KeyInput, Key, Mods};
//!
//! // Actions dispatched by key bindings
//! #[derive(PartialEq, Eq, Debug)]
//! enum Action {
//!     SayHello,
//!     OpenFile,
//!     ExitApp,
//! }
//!
//! // Create a dispatcher to dispatch actions for upcoming key inputs
//! let mut dispatcher = KeybindDispatcher::default();
//!
//! // Register key bindings to dispatch the actions
//!
//! // Key sequence "h" → "e" → "l" → "l" → "o"
//! dispatcher.bind("h e l l o", Action::SayHello).unwrap();
//! // Key combination "Ctrl + Alt + Enter"
//! dispatcher.bind("Ctrl+Alt+Enter", Action::OpenFile).unwrap();
//! // Sequence of key combinations
//! dispatcher.bind("Ctrl+x Ctrl+c", Action::ExitApp).unwrap();
//!
//! // Dispatch `SayHello` action
//! assert_eq!(dispatcher.dispatch(KeyInput::from('h')), None);
//! assert_eq!(dispatcher.dispatch(KeyInput::from('e')), None);
//! assert_eq!(dispatcher.dispatch(KeyInput::from('l')), None);
//! assert_eq!(dispatcher.dispatch(KeyInput::from('l')), None);
//! assert_eq!(dispatcher.dispatch(KeyInput::from('o')), Some(&Action::SayHello));
//!
//! // Dispatch `OpenFile` action
//! let action = dispatcher.dispatch(KeyInput::new(Key::Enter, Mods::CTRL | Mods::ALT));
//! assert_eq!(action, Some(&Action::OpenFile));
//!
//! // Dispatch `ExitApp` action
//! assert_eq!(dispatcher.dispatch(KeyInput::new('x', Mods::CTRL)), None);
//! assert_eq!(dispatcher.dispatch(KeyInput::new('c', Mods::CTRL)), Some(&Action::ExitApp));
//! ```
//!
//! # More examples
//!
//! For more usage, please see [the examples][examples]. They can be run locally by `cargo run` inside this repository.
//! Some examples require some features enabled. For instance, to run `termwiz` example:
//!
//! ```sh
//! cargo run --example termwiz --features=termwiz
//! ```
//!
//! [crates-io]: https://crates.io/crates/keybinds
//! [serde]: https://serde.rs/
//! [crossterm]: https://crates.io/crates/crossterm
//! [winit]: https://crates.io/crates/winit
//! [termwiz]: https://crates.io/crates/termwiz
//! [arbitrary]: https://crates.io/crates/arbitrary
//! [examples]: https://github.com/rhysd/keybinds-rs/tree/main/examples
//!
#![doc = include_str!("../doc/binding_syntax.md")]
#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(doc, docsrs)))]

mod error;
mod key;
mod keybind;

#[cfg(feature = "crossterm")]
pub mod crossterm;

#[cfg(feature = "serde")]
pub mod serde;

#[cfg(feature = "termwiz")]
pub mod termwiz;

#[cfg(feature = "winit")]
pub mod winit;

#[cfg(feature = "arbitrary")]
pub mod arbitrary;

pub use error::{Error, Result};
pub use key::{Key, KeyInput, KeySeq, Match, Mods};
pub use keybind::{Found, Keybind, KeybindDispatcher, Keybinds, DEFAULT_TIMEOUT};
