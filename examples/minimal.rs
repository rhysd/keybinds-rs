use keybinds::{Key, KeyInput, Keybinds};
use std::io::{self, Read};

// Actions dispatched by key bindings
#[derive(Debug)]
enum Action {
    SayHello,
    ExitApp,
}

fn main() -> io::Result<()> {
    // Create a key bindings dispatcher to dispatch actions for upcoming key inputs
    let mut keybinds = Keybinds::default();

    // Register key bindings to dispatch the actions
    keybinds.bind("h e l l o", Action::SayHello).unwrap();
    keybinds.bind("Esc", Action::ExitApp).unwrap();

    println!("Type inputs and send it by hitting Enter key. Send Esc to exit");
    for b in io::stdin().lock().bytes() {
        // Convert your key input into `KeyInput` struct
        let input = match b? {
            b'\x1b' => KeyInput::from(Key::Esc),
            b => KeyInput::from(b as char),
        };
        println!("Key input: {input:?}");

        // Try to dispatch action by `dispatch` method
        if let Some(action) = keybinds.dispatch(input) {
            println!("Dispatched action: {action:?}");
            match action {
                Action::SayHello => println!("Hello!"),
                Action::ExitApp => break,
            }
        }
    }

    Ok(())
}
