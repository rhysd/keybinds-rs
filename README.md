keybinds-rs
===========
[![crates.io][crate-badge]][crates-io]
[![CI][ci-badge]][ci]

**THIS CRATE IS WORK IN PROGRESS YET. The first beta release is planned as 0.1.0. Until then, this
library can be buggy and have arbitrary breaking changes.**

[keybinds-rs][crates-io] is a small Rust crate to define/parse/dispatch key bindings (keyboard shortcuts).

- Provide the syntax to easily define key bindings in a configuration file like `Ctrl+a`. ([document](./doc/binding_syntax.md))
- Support key sequences like `Ctrl+x Ctrl+s` for complicated key bindings like Vim style.
- Provide the core API independent from any platforms and frameworks with minimal dependencies (only one crate). ([example](./examples/minimal.rs))
- Support to parse/generate the key bindings configuration using [serde][] optionally. ([example](./examples/serde.rs))
- Support several platforms and frameworks as optional features.
  - [crossterm][] ([example](./examples/crossterm.rs))
  - [termwiz][] ([example](./examples/termwiz.rs))
  - [winit][] ([example](./examples/winit.rs))

[API Documentation][api-doc]

## Installation

```sh
cargo add keybinds
```

## Minimal usage

This crate is platform-agnostic. Create `KeybindDispatcher` instance and define key bindings by `bind` method.
Pass each key input to the `dispatch` method call. It returns a dispatched action. See the [API documentation][api-doc]
for more details.

```rust
use keybinds::{KeybindDispatcher, KeyInput, Key, Mods};

// Actions dispatched by key bindings
#[derive(PartialEq, Eq, Debug)]
enum Action {
    SayHello,
    OpenFile,
    ExitApp,
}

// Create a dispatcher to dispatch actions for upcoming key inputs
let mut dispatcher = KeybindDispatcher::default();

// Register key bindings to dispatch the actions

// Key sequence "h" → "e" → "l" → "l" → "o"
dispatcher.bind("h e l l o", Action::SayHello).unwrap();
// Key combination "Ctrl + Alt + Enter"
dispatcher.bind("Ctrl+Alt+Enter", Action::OpenFile).unwrap();
// Sequence of key combinations
dispatcher.bind("Ctrl+x Ctrl+c", Action::ExitApp).unwrap();

// Dispatch `SayHello` action
assert_eq!(dispatcher.dispatch(KeyInput::from('h')), None);
assert_eq!(dispatcher.dispatch(KeyInput::from('e')), None);
assert_eq!(dispatcher.dispatch(KeyInput::from('l')), None);
assert_eq!(dispatcher.dispatch(KeyInput::from('l')), None);
assert_eq!(dispatcher.dispatch(KeyInput::from('o')), Some(&Action::SayHello));

// Dispatch `OpenFile` action
let action = dispatcher.dispatch(KeyInput::new(Key::Enter, Mods::CTRL | Mods::ALT));
assert_eq!(action, Some(&Action::OpenFile));

// Dispatch `ExitApp` action
assert_eq!(dispatcher.dispatch(KeyInput::new('x', Mods::CTRL)), None);
assert_eq!(dispatcher.dispatch(KeyInput::new('c', Mods::CTRL)), Some(&Action::ExitApp));
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
[ci-badge]: https://github.com/rhysd/keybinds-rs/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/rhysd/keybinds-rs/actions/workflows/ci.yml
[crates-io]: https://crates.io/crates/keybinds
[serde]: https://serde.rs/
[crossterm]: https://crates.io/crates/crossterm
[winit]: https://crates.io/crates/winit
[termwiz]: https://crates.io/crates/termwiz
[api-doc]: https://docs.rs/keybinds/latest/keybinds/
[toml]: https://crates.io/crates/toml
