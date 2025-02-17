use crossterm::event::{read, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use keybinds::{KeyBindMatcher, KeyInput};
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
    // Create a matcher to trigger actions for upcoming key inputs
    let mut matcher = KeyBindMatcher::default();

    // Key bindings to trigger the actions
    matcher.bind("h i", Action::SayHi).unwrap();
    matcher.bind("Left", Action::MoveLeft).unwrap();
    matcher.bind("Ctrl+p", Action::Paste).unwrap();
    matcher.bind("Ctrl+x Ctrl+c", Action::ExitApp).unwrap();

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
