//! # Overview
//!
//! [keybinds-rs][crates-io] is a small crate to parse/generate/dispatch key bindings (keyboard shortcuts) written
//! in Safe Rust.
//!
//! - Provide the syntax to easily define key bindings in a configuration file like `Ctrl+a`
//! - Support key sequences like `Ctrl+x Ctrl+s` for complicated key bindings like Vim style
//! - Core API independent from any platforms and frameworks with minimal dependencies (only two crates)
//! - Support several platforms and frameworks as optional features
//!   - [crossterm][]
//!   - [termwiz][]
//!   - [winit][]
//!   - [iced][]
//! - Support parsing/generating a key bindings configuration using [serde][] optionally
//! - Support structure-aware fuzzing using [arbitrary][] optionally.
//!
//! # Installation
//!
//! ```sh
//! cargo add keybinds
//! ```
//!
//! # Minimal example
//!
//! ```
//! use keybinds::{Keybinds, KeyInput, Key, Mods};
//!
//! // Actions dispatched by key bindings
//! #[derive(PartialEq, Eq, Debug)]
//! enum Action {
//!     SayHello,
//!     OpenFile,
//!     ExitApp,
//! }
//!
//! // Create a key bindings dispatcher to dispatch actions for upcoming key inputs
//! let mut keybinds = Keybinds::default();
//!
//! // Register key bindings to dispatch the actions
//!
//! // Key sequence "h" → "e" → "l" → "l" → "o"
//! keybinds.bind("h e l l o", Action::SayHello).unwrap();
//! // Key combination "Ctrl + Alt + Enter"
//! keybinds.bind("Ctrl+Alt+Enter", Action::OpenFile).unwrap();
//! // Sequence of key combinations
//! keybinds.bind("Ctrl+x Ctrl+c", Action::ExitApp).unwrap();
//!
//! // Dispatch `SayHello` action
//! assert_eq!(keybinds.dispatch('h'), None);
//! assert_eq!(keybinds.dispatch('e'), None);
//! assert_eq!(keybinds.dispatch('l'), None);
//! assert_eq!(keybinds.dispatch('l'), None);
//! assert_eq!(keybinds.dispatch('o'), Some(&Action::SayHello));
//!
//! // Dispatch `OpenFile` action
//! let action = keybinds.dispatch(KeyInput::new(Key::Enter, Mods::CTRL | Mods::ALT));
//! assert_eq!(action, Some(&Action::OpenFile));
//!
//! // Dispatch `ExitApp` action
//! assert_eq!(keybinds.dispatch(KeyInput::new('x', Mods::CTRL)), None);
//! assert_eq!(keybinds.dispatch(KeyInput::new('c', Mods::CTRL)), Some(&Action::ExitApp));
//! ```
//!
//! # More examples
//!
//! For more usage, please see [the examples][examples]. They can be run locally by `cargo run` inside this repository.
//! Some examples require some features enabled. For instance, to run the `crossterm` example:
//!
//! ```sh
//! cargo run --example crossterm --features=crossterm,serde
//! ```
//!
//! # Features
//!
//! The list of crate features can be found in `[features]` section of [Cargo.toml][metadata]. Please read the comments
//! on each features which explains about it.
//!
//! # Minimal supported Rust version (MSRV)
//!
//! See `rust-version` field of [Cargo.toml][metadata] for the minimal supported Rust version. Note that enabling
//! optional features may require some higher Rust versions due to the optional dependencies introduced by them.
//!
//! [crates-io]: https://crates.io/crates/keybinds
//! [serde]: https://serde.rs/
//! [crossterm]: https://crates.io/crates/crossterm
//! [winit]: https://crates.io/crates/winit
//! [iced]: https://crates.io/crates/iced
//! [termwiz]: https://crates.io/crates/termwiz
//! [arbitrary]: https://crates.io/crates/arbitrary
//! [examples]: https://github.com/rhysd/keybinds-rs/tree/main/examples
//! [metadata]: https://github.com/rhysd/keybinds-rs/blob/main/Cargo.toml
//!
#![doc = include_str!("../doc/versioning.md")]
#![doc = include_str!("../doc/binding_syntax.md")]
//! [!Caution]: #modifiers
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

#[cfg(feature = "iced")]
pub mod iced;

#[cfg(feature = "arbitrary")]
pub mod arbitrary;

pub use error::{Error, Result};
pub use key::{Key, KeyInput, KeySeq, Match, Mods};
pub use keybind::{Keybind, Keybinds, DEFAULT_TIMEOUT};
