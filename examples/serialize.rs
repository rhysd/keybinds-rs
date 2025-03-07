use keybinds::{KeySeq, Keybind, KeybindsOld};
use serde::Serialize;
use std::io::stdin;

// Actions to be dispatched by key bindings
#[derive(Serialize)]
enum Action {
    DoSomething,
}

// Configuration of your app
#[derive(Serialize)]
struct Config {
    bindings: KeybindsOld<Action>,
}

fn main() {
    println!("Input your favorite key bindings like Ctrl+a per line.");
    println!("Input an empty line to finish.");

    let mut binds = vec![];
    for line in stdin().lines().map(Result::unwrap) {
        if line.is_empty() {
            break;
        }
        let seq: KeySeq = line.parse().unwrap();
        binds.push(Keybind::new(seq, Action::DoSomething));
    }

    let config = Config {
        // `Keybinds` is a map from key bindings to the actions
        bindings: KeybindsOld::from(binds),
    };

    // Generate configuration file content using serde
    let generated = toml::to_string_pretty(&config).unwrap();
    println!("{generated}");
}
