use keybinds::{KeySeq, Keybind, Keybinds};
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
    bindings: Keybinds<Action>,
}

fn main() -> keybinds::Result<()> {
    println!("Input your favorite key bindings like Ctrl+a per line.");
    println!("Input an empty line to finish.");

    let bindings: Keybinds<_> = stdin()
        .lines()
        .map(Result::unwrap)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let seq: KeySeq = l.parse()?;
            Ok(Keybind::new(seq, Action::DoSomething))
        })
        .collect::<Result<_, _>>()?;
    let config = Config { bindings };

    // Generate configuration file content using serde
    let generated = toml::to_string_pretty(&config).unwrap();
    println!("{generated}");

    Ok(())
}
