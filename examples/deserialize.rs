use keybinds::{KeybindDispatcher, Keybinds};
use serde::Deserialize;

// Actions triggered by key bindings
#[derive(Deserialize, Debug)]
enum Action {
    OpenFile,
    ExitApp,
}

// Configuration file format of your application
#[derive(Deserialize)]
struct Config {
    // `Keybinds` implements serde's `Deserialize` to deserialize key bindings from a mapping object
    bindings: Keybinds<Action>,
}

fn main() {
    let configuration = r#"
[bindings]
"Ctrl+Alt+Enter" = "OpenFile"
"Ctrl+x Ctrl+c" = "ExitApp"
"#;

    // Parse the TOML input
    let config: Config = toml::from_str(configuration).unwrap();

    // Use the key bindings parsed from the TOML input
    let dispatcher = KeybindDispatcher::new(config.bindings);

    dbg!(dispatcher.keybinds());
}
