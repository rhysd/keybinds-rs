use keybinds::Keybinds;
use serde::Deserialize;
use std::time::Duration;

// Actions triggered by key bindings
#[derive(Deserialize, Debug)]
enum Action {
    OpenFile,
    ExitApp,
}

// Keyboard specific configuration
#[derive(Deserialize)]
struct KeyboardConfig {
    // `Keybinds` implements serde's `Deserialize` to deserialize key bindings from a mapping object
    bindings: Keybinds<Action>,
    // Timeout on waiting for the next input while the matching is ongoing.
    timeout: Option<u64>,
}

// Configuration file format of your application
#[derive(Deserialize)]
struct Config {
    keyboard: KeyboardConfig,
}

fn main() {
    let configuration = r#"
[keyboard]
timeout = 500

[keyboard.bindings]
"Ctrl+Alt+Enter" = "OpenFile"
"Ctrl+x Ctrl+c" = "ExitApp"
"#;

    // Parse the TOML input into the `Config` instance
    let config: Config = toml::from_str(configuration).unwrap();
    let mut keybinds = config.keyboard.bindings;

    // Set the matching timeout if needed
    if let Some(ms) = config.keyboard.timeout {
        keybinds.set_timeout(Duration::from_millis(ms));
    }

    dbg!(keybinds.as_slice());
    dbg!(keybinds.timeout());
}
