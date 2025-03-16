keybinds-rs
===========
[![crate][crate-badge]][crates-io]
[![docs][doc-badge]][api-doc]
[![CI][ci-badge]][ci]

[keybinds-rs][crates-io] is a small platform&framework-agnostic crate to parse/generate/dispatch key bindings (keyboard
shortcuts) written in Safe Rust. You can easily introduce customizable key bindings to your application using this
library.

- Provide the [syntax](./doc/binding_syntax.md) to easily define key bindings in a configuration file like `Ctrl+a`.
- Support key sequences like `Ctrl+x Ctrl+s` for complicated key bindings like Vim style. ([example](./examples/vim.rs))
- Provide the core API independent from any platforms and frameworks with minimal (only two crates) dependencies. ([example](./examples/minimal.rs))
- Support several platforms and frameworks as optional features.
  - [crossterm][] ([example](./examples/crossterm.rs))
  - [termwiz][] ([example](./examples/termwiz.rs))
  - [winit][] ([example](./examples/winit.rs))
  - [iced][] ([example](./examples/iced.rs))
- Support [parsing](./examples/deserialize.rs)/[generating](./examples/serialize.rs) a key bindings configuration
  using [serde][] optionally.
- Support structure-aware fuzzing using [arbitrary][] optionally. ([example](./examples/arbitrary.rs))

[API Documentation][api-doc]

## Installation

```sh
cargo add keybinds
```

## Usage

The following code demonstrates the usage by parsing key bindings configuration from TOML input and dispatching actions
to move the cursor inside terminal with the `serde` and `crossterm` optional features. This code can be run as the
[example](./examples/crossterm.rs). See the [API documentation][api-doc] for more details.

```rust
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, execute};
use keybinds::Keybinds;
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

// Configuration of your application
#[derive(Deserialize)]
struct Config {
    keyboard: Keybinds<Action>,
}

const CONFIG_FILE_CONTENT: &str = r#"
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
    let config: Config = toml::from_str(CONFIG_FILE_CONTENT).unwrap();

    // `Keybinds` instance is a key bindings dispatcher that receives key inputs and
    // dispatches the corresponding actions.
    let mut keybinds = config.keyboard;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    while let Ok(event) = event::read() {
        // If the event triggered some action, handle it using `match`
        if let Some(action) = keybinds.dispatch(&event) {
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

For more usage, please see the working [examples](./examples). They can be run locally by `cargo run` inside this
repository. Some examples require some features enabled. For instance, to run the above `crossterm` example:

```sh
cargo run --example crossterm --features=crossterm,serde
```

## Features

The list of crate features can be found in `[features]` section of [Cargo.toml](./Cargo.toml). Please read the comments
on each features which explains about it.

## Minimal supported Rust version (MSRV)

See `rust-version` field of [Cargo.toml](./Cargo.toml) for the minimal supported Rust version. Note that enabling
optional features may require some higher Rust versions due to the optional dependencies introduced by them.

## Versioning

See the [document](./doc/versioning.md).

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
