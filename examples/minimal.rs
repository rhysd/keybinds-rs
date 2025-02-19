use keybinds::{Key, KeyInput, KeybindDispatcher};
use std::io::{self, Read};

// Actions dispatched by key bindings
#[derive(Debug)]
enum Action {
    SayHello,
    ExitApp,
}

fn main() -> io::Result<()> {
    // Create a dispatcher to dispatch actions for upcoming key inputs
    let mut dispatcher = KeybindDispatcher::default();

    // Register key bindings to dispatch the actions
    dispatcher.bind("h e l l o", Action::SayHello).unwrap();
    dispatcher.bind("Esc", Action::ExitApp).unwrap();

    println!("Type inputs and send it by hitting Enter key. Send Esc to exit");
    for b in io::stdin().bytes() {
        // Convert your key input into `KeyInput` struct
        let input = match b? {
            b'\x1b' => KeyInput::from(Key::Esc),
            b => KeyInput::from(b as char),
        };
        println!("Key input: {input:?}");

        // Try to dispatch action by `dispatch` method
        if let Some(action) = dispatcher.dispatch(input) {
            println!("Dispatched action: {action:?}");
            match action {
                Action::SayHello => println!("Hello!"),
                Action::ExitApp => break,
            }
        }
    }

    Ok(())
}
