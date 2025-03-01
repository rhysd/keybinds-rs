keybinds-rs
===========
[![crate][crate-badge]][crates-io]
[![docs][doc-badge]][api-doc]
[![CI][ci-badge]][ci]

**THIS CRATE IS WORK IN PROGRESS YET. The first beta release is planned as 0.1.0. Until then, this
library can be buggy and have arbitrary breaking changes.**

[keybinds-rs][crates-io] is a small crate to parse/generate/dispatch key bindings (keyboard shortcuts) written in Safe Rust.

- Provide the syntax to easily define key bindings in a configuration file like `Ctrl+a`. ([document](./doc/binding_syntax.md))
- Support key sequences like `Ctrl+x Ctrl+s` for complicated key bindings like Vim style.
- Provide the core API independent from any platforms and frameworks with minimal dependencies (only one crate). ([example](./examples/minimal.rs))
- Support several platforms and frameworks as optional features.
  - [crossterm][] ([example](./examples/crossterm.rs))
  - [termwiz][] ([example](./examples/termwiz.rs))
  - [winit][] ([example](./examples/winit.rs))
  - [iced][] ([example](./examples/iced.rs))
- Support parsing/generating the key bindings configuration using [serde][] optionally.
  - [Deserialization example](./examples/deserialize.rs)
  - [Serialization example](./examples/serialize.rs)
- Support structure-aware fuzzing using [arbitrary][] optionally. ([example](./examples/arbitrary.rs))

[API Documentation][api-doc]

## Installation

```sh
cargo add keybinds
```

## Usage

The following code demonstrates the usage by parsing and dispatching key bindings for moving the cursor inside terminal
using the `serde` and `crossterm` optional features. The code can be run as the [example](./examples/crossterm.rs).
See the [API documentation][api-doc] for more details.

```rust
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, execute};
use keybinds::{KeybindDispatcher, Keybinds};
use serde::Deserialize;
use std::io;

// Actions dispatched by key bindings
#[derive(Deserialize)]
enum Action {
    Exit,
    Up,
    Down,
    Left,
    Right,
    Top,
    Bottom,
    Home,
    End,
}

// Configuration of your app
#[derive(Deserialize)]
struct Config {
    keyboard: Keybinds<Action>,
}

const CONFIG_FILE: &str = r#"
[keyboard]
"Esc" = "Exit"

# Standard bindings
"Up" = "Up"
"Down" = "Down"
"Left" = "Left"
"Right" = "Right"
"PageUp" = "Top"
"PageDown" = "Bottom"
"Home" = "Home"
"End" = "End"

# Emacs-like bindings
"Ctrl+p" = "Up"
"Ctrl+n" = "Down"
"Ctrl+b" = "Left"
"Ctrl+f" = "Right"
"Alt+<" = "Top"
"Alt+>" = "Bottom"
"Ctrl+a" = "Home"
"Ctrl+e" = "End"

# Vim-like bindings
"k" = "Up"
"j" = "Down"
"h" = "Left"
"l" = "Right"
"g g" = "Top"
"G" = "Bottom"
"^" = "Home"
"$" = "End"
"#;

fn main() -> io::Result<()> {
    // Parse the configuration from the file content
    let config: Config = toml::from_str(CONFIG_FILE).unwrap();

    // Create the key binding dispatcher to handle key input events
    let mut dispatcher = KeybindDispatcher::new(config.keyboard);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    while let Ok(event) = event::read() {
        // If the event triggered some action, handle it using `match`
        if let Some(action) = dispatcher.dispatch(&event) {
            match action {
                Action::Exit => break,
                Action::Up => execute!(stdout, cursor::MoveUp(1))?,
                Action::Down => execute!(stdout, cursor::MoveDown(1))?,
                Action::Left => execute!(stdout, cursor::MoveLeft(1))?,
                Action::Right => execute!(stdout, cursor::MoveRight(1))?,
                Action::Top => execute!(stdout, cursor::MoveUp(9999))?,
                Action::Bottom => execute!(stdout, cursor::MoveDown(9999))?,
                Action::Home => execute!(stdout, cursor::MoveLeft(9999))?,
                Action::End => execute!(stdout, cursor::MoveRight(9999))?,
            }
        }
    }
    disable_raw_mode()
}
```

## Examples

For more usage, please see the [examples](./examples). They can be run locally by `cargo run` inside this repository.
Some examples require some features enabled. For instance, to run `termwiz` example:

```sh
cargo run --example termwiz --features=termwiz
```

## License

This crate is licensed under [the MIT license](./LICENSE.txt).

[crate-badge]: https://img.shields.io/crates/v/keybinds
[crates-io]: https://crates.io/crates/keybinds
[doc-badge]: https://docs.rs/keybinds/badge.svg
[api-doc]: https://docs.rs/keybinds/latest/keybinds/
[ci-badge]: https://github.com/rhysd/keybinds-rs/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/rhysd/keybinds-rs/actions/workflows/ci.yml
[serde]: https://serde.rs/
[crossterm]: https://crates.io/crates/crossterm
[winit]: https://crates.io/crates/winit
[iced]: https://crates.io/crates/iced
[termwiz]: https://crates.io/crates/termwiz
[arbitrary]: https://crates.io/crates/arbitrary
[toml]: https://crates.io/crates/toml
