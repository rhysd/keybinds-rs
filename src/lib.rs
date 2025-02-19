#![forbid(unsafe_code)]
#![warn(clippy::dbg_macro, clippy::print_stdout)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg_hide))]
#![cfg_attr(docsrs, doc(cfg_hide(doc, docsrs)))]
#![doc = include_str!("../README.md")]

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

pub use error::{Error, Result};
pub use key::{Key, KeyInput, KeySeq, Match, Mods};
pub use keybind::{Found, Keybind, KeybindDispatcher, Keybinds, DEFAULT_TIMEOUT};
