keybinds-rs
===========
[![crates.io][crate-badge]][crates-io]
[![CI][ci-badge]][ci]

**THIS CRATE IS WORK IN PROGRESS YET. The first beta release is planned as 0.1.0. Until then, this
library can be buggy and have arbitrary breaking changes.**

[keybinds-rs][crates-io] is a small Rust crate to define/parse/dispatch key bindings.

- Provide a syntax to easily define key bindings in a configuration file like `Ctrl+a`
- Support key sequences like `Ctrl+x Ctrl+s`
- Support to parse/generate the key bindings configuration using [serde][] optionally ([example](./examples/serde))
- Core API independent from any platforms and frameworks with minimal dependencies
- Support several platforms and frameworks as optional features
  - [crossterm][] ([example](./examples/crossterm.rs))
  - [termwiz][] ([example](./examples/termwiz.rs))
  - [winit][] ([example](./examples/winit.rs))

[Documentation][api-doc]

## Installation

```sh
cargo add keybinds
```

## Basic usage

This crate is platform-agnostic. Create `KeybindDispatcher` instance and define key bindings by `bind` method.
Pass each key input to the `dispatch` method call. It returns a dispatched action. Key sequence and key combination
can be parsed using `FromStr` trait. See the [API documentation][api-doc] for more details.

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

## Syntax for key sequence and combination

Keys are joint with `+` as a key combination like `Ctrl+a`. The last key must be a normal key and others must be modifier
keys.

Normal keys are a single character (e.g. `a`, `X`, `あ`) or a special key name (e.g. `Up`, `Enter`, `Tab`). Note that
the characters are case-sensitive. `A` means typing "A" and "Shift" keys on US keyboard.

These are **logical** keys which are inputs as the result of key typing. In comparison, physical keys are actual keys on
your keyboard. For example, typing the physical keys "Shift" and "9" produces the logical key `(` with US keyboard, and
it also produces the logical key `)` with JP keyboard.

The following modifier keys are available:

- `Ctrl`: "Ctrl" key
- `Cmd`: "Command" key
- `Mod`: "Command" key on macOS, "Ctrl" key on other platforms
- `Super`: "Windows" key on platforms other than macOS, Command key on macOS
- `Alt`: "Alt" key
- `Option`: An alias to "Alt" key

Here are some examples of key combinations:

- `a`
- `X`
- `!`
- `Enter`
- `Ctrl+X`
- `Mod+x`
- `Ctrl+Alt+Left`

Key combinations are joint with whitespaces as a key sequence. When key combinations are input in the order, they
trigger the action.

Here are some examples of key sequences:

- `h e l l o`
- `Ctrl+x Ctrl+c`

## [serde][] support

See the document for `serde` module.
The serde support requires the `serde` feature enabled.

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
