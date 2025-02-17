keybinds-rs
===========
[![CI][ci-badge]][ci]

**THIS CRATE IS WORK IN PROGRESS YET. The first beta release is planned as 0.1.0. Until then, this
library can be buggy and have arbitrary breaking changes.**

[keybinds-rs][crates-io] is a small Rust crate to define/parse/dispatch key bindings.

- Provide a syntax to easily define key bindings in a configuration file like `Ctrl+A`
- Support key sequences like `Ctrl+X Ctrl+S`
- Support to parse/generate the key bindings configuration using [serde][] optionally
- Core API independent from any platforms and frameworks with minimal dependencies
- Support several platforms and frameworks as optional features
  - [crossterm][] ([example](./examples/crossterm.rs))
  - [termwiz][] ([example](./examples/termwiz.rs))
  - [winit][] ([example](./examples/winit.rs))

[Documentation][api-doc]

## Installation

Use `cargo add` to update your `Cargo.toml`.

```sh
cargo add keybinds
```

If some additional features are needed, use `--features` flag.

```sh
cargo add keybinds --features=serde
```

## Basic usage

This crate is platform-agnostic. Define key bindings by `Keybinds` and build `KeybindDispatcher` instance with it.
Pass each key input to the `trigger` method and it returns a triggered action. Key sequence and key combination
can be parsed using `FromStr` trait. See the [API documentation][api-doc] for more details.

```rust
use keybinds::{KeybindDispatcher, KeyInput, Key, Mods};

// Actions triggered by key bindings
#[derive(PartialEq, Eq, Debug)]
enum Action {
    SayHello,
    OpenFile,
    ExitApp,
}

// Create a dispatcher to trigger actions for upcoming key inputs
let mut dispatcher = KeybindDispatcher::default();

// Register key bindings to trigger the actions

// Key sequence "hello"
dispatcher.bind("h e l l o", Action::SayHello).unwrap();
// Key combination "Ctrl + Shift + Enter"
dispatcher.bind("Ctrl+Shift+Enter", Action::OpenFile).unwrap();
// Sequence of key combinations
dispatcher.bind("Ctrl+x Ctrl+c", Action::ExitApp).unwrap();

// Trigger `SayHello` action
assert_eq!(dispatcher.trigger(KeyInput::from('h')), None);
assert_eq!(dispatcher.trigger(KeyInput::from('e')), None);
assert_eq!(dispatcher.trigger(KeyInput::from('l')), None);
assert_eq!(dispatcher.trigger(KeyInput::from('l')), None);
assert_eq!(dispatcher.trigger(KeyInput::from('o')), Some(&Action::SayHello));

// Trigger `OpenFile` action
let action = dispatcher.trigger(KeyInput::new(Key::Enter, Mods::CTRL | Mods::SHIFT));
assert_eq!(action, Some(&Action::OpenFile));

// Trigger `ExitApp` action
assert_eq!(dispatcher.trigger(KeyInput::new('x', Mods::CTRL)), None);
assert_eq!(dispatcher.trigger(KeyInput::new('c', Mods::CTRL)), Some(&Action::ExitApp));
```

## Syntax for key sequence and combination

Keys are joint with `+` as a key combination like `Ctrl+a`. The last key must be a normal key and others must be modifier
keys.

Normal keys are a single character (e.g. `a`, `X`, `あ`) or a special key name (e.g. `Up`, `Enter`, `Tab`). Note that
upper case characters like `A` are equivalent to the lower case ones like `a`. For representing `A` key, explicitly
specify `Shift` modifier key.

The following modifier keys are available:

- `Ctrl`: "Ctrl" key
- `Cmd`: "Command" key
- `Mod`: "Command" key on macOS, "Ctrl" key on other platforms
- `Super`: "Windows" key on platforms other than macOS, Command key on macOS
- `Shift`: "Shift" key
- `Alt`: "Alt" key
- `Option`: An alias to "Alt" key

Here are some examples of key combinations:

- `a`
- `Enter`
- `Mod+x`
- `Ctrl+Shift+Left`

Key combinations are joint with whitespaces as a key sequence. When key combinations are input in the order, they
trigger the action.

Here are some examples of key sequences:

- `h e l l o`
- `Ctrl+x Ctrl+c`

## [serde][] support

See the document for `serde` module.
The serde support requires the `serde` feature enabled.

### Parsing key bindings configurations

`Keybinds` implements serde's `Deserialize` trait. This is an example to parse key bindings with [toml][] crate.

```rust,ignore
use serde::Deserialize;
use keybinds::{Keybinds, KeybindDispatcher, Key, Mods, KeyInput};

// Actions triggered by key bindings
#[derive(Deserialize, PartialEq, Eq, Debug)]
enum Action {
    OpenFile,
    ExitApp,
}

// Configuration file format of your application
#[derive(Deserialize)]
struct Config {
    // `Keybinds` implements serde's `Deserialize`
    bindings: Keybinds<Action>,
}

let configuration = r#"
[bindings]
"Ctrl+Shift+Enter" = "OpenFile"
"Ctrl+x Ctrl+c" = "ExitApp"
"#;

// Parse the TOML input
let config: Config = toml::from_str(configuration).unwrap();

// Use the key bindings parsed from the TOML input
let mut dispatcher = KeybindDispatcher::new(config.bindings);
let action = dispatcher.trigger(KeyInput::new(Key::Enter, Mods::CTRL | Mods::SHIFT));
assert_eq!(action, Some(&Action::OpenFile));
```

## License

This crate is licensed under [the MIT license](./LICENSE.txt).

[ci-badge]: https://github.com/rhysd/keybinds-rs/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/rhysd/keybinds-rs/actions/workflows/ci.yml
[crates-io]: https://crates.io/crates/keybinds
[serde]: https://serde.rs/
[crossterm]: https://crates.io/crates/crossterm
[winit]: https://crates.io/crates/winit
[termwiz]: https://crates.io/crates/termwiz
[api-doc]: https://docs.rs/keybinds/latest/keybinds/
[toml]: https://crates.io/crates/toml
