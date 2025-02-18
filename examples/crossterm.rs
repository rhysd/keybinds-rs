use crossterm::event::{read, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use keybinds::{KeyInput, KeybindDispatcher};
use std::io;

// Actions dispatched by key bindings
#[derive(PartialEq, Eq, Debug)]
enum Action {
    SayHi,
    MoveLeft,
    Paste,
    ExitApp,
}

fn main() -> io::Result<()> {
    // Create a dispatcher to dispatch actions for upcoming key inputs
    let mut dispatcher = KeybindDispatcher::default();

    // Key bindings to dispatch the actions
    dispatcher.bind("h i", Action::SayHi).unwrap();
    dispatcher.bind("Left", Action::MoveLeft).unwrap();
    dispatcher.bind("Ctrl+p", Action::Paste).unwrap();
    dispatcher.bind("Ctrl+x Ctrl+c", Action::ExitApp).unwrap();

    println!("Type Ctrl+X → Ctrl+C to exit");
    enable_raw_mode()?;

    while let Ok(event) = read() {
        if let Event::Key(event) = event {
            // Can convert crossterm's `KeyEvent` into `KeyInput`
            println!("Key input `{:?}`\r", KeyInput::from(event));

            // `KeybindDispatcher::dispatch` accepts crossterm's `KeyEvent`
            if let Some(action) = dispatcher.dispatch(event) {
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
