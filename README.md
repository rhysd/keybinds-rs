keybinds-rs
===========
[![CI][ci-badge]][ci]

> [!CAUTION]
> This crate is work in progress yet.

[keybinds-rs][crates-io] is a small Rust crate to define/parse/match key bindings.

- Provide a syntax to easily define key bindings in a configuration file like `Ctrl+A`
- Support key sequences like `Ctrl+X Ctrl+S`
- Parse/Generate the key bindings configuration using [serde][]
- Platform-agnostic core API with minimal dependencies
- **TODO:** Support several platforms as optional features

[Documentation][api-doc]

## Installation

```sh
cargo add keybinds
```

## Usage

This crate is platform-agnostic. Define key bindings by `KeyBinds` and build `KeyBindMatcher` instance with it.
Pass each key input to the `trigger` method and it returns a triggered action. Key sequence and key combination
can be parsed using `FromStr` trait. See the [API documentation][api-doc] for more details.

```rust
use keybinds::{KeyBind, KeyBinds, KeyBindMatcher, KeyInput, Key, Mods};

// Actions triggered by key bindings
#[derive(PartialEq, Eq, Debug)]
enum Action {
    SayHello,
    OpenFile,
    ExitApp,
}

// Key bindings to trigger the actions
let keybinds = KeyBinds::new(vec![
    // Key sequence "hello"
    KeyBind::multiple("h e l l o".parse().unwrap(), Action::SayHello),
    // Key combination "Ctrl + Shift + Enter"
    KeyBind::single("Ctrl+Shift+Enter".parse().unwrap(), Action::OpenFile),
    // Sequence of key combinations
    KeyBind::multiple("Ctrl+x Ctrl+c".parse().unwrap(), Action::ExitApp),
]);

let mut matcher = KeyBindMatcher::new(keybinds);

// Trigger `SayHello` action
assert_eq!(matcher.trigger(KeyInput::from('h')), None);
assert_eq!(matcher.trigger(KeyInput::from('e')), None);
assert_eq!(matcher.trigger(KeyInput::from('l')), None);
assert_eq!(matcher.trigger(KeyInput::from('l')), None);
assert_eq!(matcher.trigger(KeyInput::from('o')), Some(&Action::SayHello));

// Trigger `OpenFile` action
let action = matcher.trigger(KeyInput::new(Key::Enter, Mods::CTRL | Mods::SHIFT));
assert_eq!(action, Some(&Action::OpenFile));

// Trigger `ExitApp` action
assert_eq!(matcher.trigger(KeyInput::new('x', Mods::CTRL)), None);
assert_eq!(matcher.trigger(KeyInput::new('c', Mods::CTRL)), Some(&Action::ExitApp));
```

## Syntax for key sequence and combination

Keys are joint with `+` as a key combination like `Ctrl+a`. The last key must be a normal key and others must be modifier
keys.

Normal keys are a single character (e.g. `a`, `X`, `あ`) or a special key name (e.g. `Up`, `Enter`, `Tab`). Note that
upper case characters like `A` are equivalent to the lower case ones like `a`. For representing `A` key, explicitly
specify `Shift` modifier key.

The following modifier keys are available:

- `Ctrl`: Ctrl key
- `Cmd`: Command key
- `Mod`: Command key on macOS, Ctrl key on other platforms
- `Shift`: Shift key
- `Alt`: Alt key
- `Option`: An alias to Alt key

Here are some examples of key combinations:

```ignore
a
Enter
Mod+x
Ctrl+Shift+Left
```

Key combinations are joint with whitespaces as a key sequence. When key combinations are input in the order, they
trigger the action.

Here are some examples of key sequences:

```ignore
h e l l o
Ctrl+x Ctrl+c
```

## [serde][] support

### Parsing key bindings configurations

`KeyBinds` implements serde's `Deserialize` trait. This is an example to parse key bindings as TOML.

```rust
use serde::Deserialize;
use keybinds::{KeyBinds, KeyBindMatcher, Key, Mods, KeyInput};

// Actions triggered by key bindings
#[derive(Deserialize, PartialEq, Eq, Debug)]
enum Action {
    OpenFile,
    ExitApp,
}

// Configuration file format of your application
#[derive(Deserialize)]
pub struct Config {
    pub bindings: KeyBinds<Action>,
}

let configuration = r#"
[bindings]
"Ctrl+Shift+Enter" = "OpenFile"
"Ctrl+x Ctrl+c" = "ExitApp"
"#;

// Parse the TOML input
let config: Config = toml::from_str(configuration).unwrap();

// Use the key bindings parsed from the TOML input
let mut matcher = KeyBindMatcher::new(config.bindings);
let action = matcher.trigger(KeyInput::new(Key::Enter, Mods::CTRL | Mods::SHIFT));
assert_eq!(action, Some(&Action::OpenFile));
```

## License

This crate is licensed under [the MIT license](./LICENSE.txt).

[ci-badge]: https://github.com/rhysd/keybinds-rs/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/rhysd/keybinds-rs/actions/workflows/ci.yml
[crates-io]: https://crates.io/crates/keybinds
[serde]: https://serde.rs/
[api-doc]: https://docs.rs/keybinds/latest/keybinds/
