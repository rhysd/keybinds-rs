use keybinds::{KeyInput, KeybindDispatcher};
use termwiz::caps::Capabilities;
use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::surface::{Change, Position};
use termwiz::terminal::buffered::BufferedTerminal;
use termwiz::terminal::{new_terminal, Terminal};
use termwiz::Error;

// Actions dispatched by key bindings
#[derive(PartialEq, Eq, Debug)]
enum Action {
    SayHi,
    MoveLeft,
    Paste,
    ExitApp,
}

fn main() -> Result<(), Error> {
    // Create an action dispatcher to dispatch actions for upcoming key inputs
    let mut dispatcher = KeybindDispatcher::default();

    // Key bindings to dispatch the actions
    dispatcher.bind("h i", Action::SayHi).unwrap();
    dispatcher.bind("Left", Action::MoveLeft).unwrap();
    dispatcher.bind("Ctrl+p", Action::Paste).unwrap();
    dispatcher.bind("Ctrl+x Ctrl+c", Action::ExitApp).unwrap();

    let caps = Capabilities::new_from_env()?;
    let terminal = new_terminal(caps)?;

    let mut buf = BufferedTerminal::new(terminal)?;
    buf.add_change("Type Ctrl+x Ctrl+c to exit");
    buf.add_change(Change::CursorPosition {
        x: Position::Absolute(0),
        y: Position::Absolute(2),
    });
    buf.flush()?;
    buf.terminal().set_raw_mode()?;

    loop {
        let Some(input) = buf.terminal().poll_input(None)? else {
            continue;
        };

        // Dispatch action by directly passing `InputEvent` to `dispatch` method.
        let action = dispatcher.dispatch(&input);

        buf.add_change(Change::CursorPosition {
            x: Position::Absolute(0),
            y: Position::Absolute(0),
        });
        buf.add_change(Change::ClearToEndOfLine(ColorAttribute::Default));
        if let Some(action) = action {
            buf.add_change(Change::Attribute(AttributeChange::Foreground(
                AnsiColor::Red.into(),
            )));
            buf.add_change(format!("Action: {action:?}"));
        }

        buf.add_change(Change::CursorPosition {
            x: Position::Absolute(0),
            y: Position::Absolute(2),
        });
        buf.add_change(Change::ClearToEndOfLine(ColorAttribute::Default));
        buf.add_change(format!("KeyInput: {:?}", KeyInput::from(input)));

        buf.flush()?;

        if action == Some(&Action::ExitApp) {
            return Ok(());
        }
    }
}
