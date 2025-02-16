use keybinds::{KeyBind, KeyBindMatcher, KeyBinds, KeyInput};
use termwiz::caps::Capabilities;
use termwiz::cell::AttributeChange;
use termwiz::color::{AnsiColor, ColorAttribute};
use termwiz::surface::{Change, Position};
use termwiz::terminal::buffered::BufferedTerminal;
use termwiz::terminal::{new_terminal, Terminal};
use termwiz::Error;

// Actions triggered by key bindings
#[derive(PartialEq, Eq, Debug)]
enum Action {
    SayHi,
    MoveLeft,
    Paste,
    ExitApp,
}

fn main() -> Result<(), Error> {
    // Key bindings to trigger the actions
    let keybinds = KeyBinds::new(vec![
        KeyBind::multiple("h i".parse().unwrap(), Action::SayHi),
        KeyBind::single("Left".parse().unwrap(), Action::MoveLeft),
        KeyBind::single("Ctrl+p".parse().unwrap(), Action::Paste),
        KeyBind::multiple("Ctrl+x Ctrl+c".parse().unwrap(), Action::ExitApp),
    ]);

    // Create a matcher to trigger actions for upcoming key inputs
    let mut matcher = KeyBindMatcher::new(keybinds);

    let caps = Capabilities::new_from_env()?;
    let terminal = new_terminal(caps)?;

    let mut buf = BufferedTerminal::new(terminal)?;
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

        // Trigger action by matching the key input
        let action = matcher.trigger(&input);

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
