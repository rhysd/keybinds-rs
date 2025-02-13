use crossterm::event::{read, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use keybinds::{KeyBind, KeyBindMatcher, KeyBinds, KeyInput};
use std::io;

// Actions triggered by key bindings
#[derive(PartialEq, Eq, Debug)]
enum Action {
    SayHi,
    MoveLeft,
    Paste,
    ExitApp,
}

fn main() -> io::Result<()> {
    // Key bindings to trigger the actions
    let keybinds = KeyBinds::new(vec![
        KeyBind::multiple("h i".parse().unwrap(), Action::SayHi),
        KeyBind::single("Left".parse().unwrap(), Action::MoveLeft),
        KeyBind::single("Ctrl+p".parse().unwrap(), Action::Paste),
        KeyBind::multiple("Ctrl+x Ctrl+c".parse().unwrap(), Action::ExitApp),
    ]);

    // Create a matcher to trigger actions for upcoming key inputs
    let mut matcher = KeyBindMatcher::new(keybinds);

    println!("Type Ctrl+X → Ctrl+C to exit");
    enable_raw_mode()?;

    while let Ok(event) = read() {
        if let Event::Key(event) = event {
            // Can convert crossterm's `KeyEvent` into `KeyInput`
            println!("Key input `{:?}`\r", KeyInput::from(event));

            // `KeyBindMatcher::trigger` accepts crossterm's `KeyEvent`
            if let Some(action) = matcher.trigger(event) {
                println!("Triggered action `{action:?}`\r");
                if action == &Action::ExitApp {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
