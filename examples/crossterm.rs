use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, execute};
use keybinds::{Keybinds, KeybindsOld};
use serde::Deserialize;
use std::io;

// Actions dispatched by key bindings
#[derive(Deserialize)]
enum Action {
    Exit,
    Up,
    Down,
    Left,
    Right,
    Top,
    Bottom,
    Home,
    End,
}

// Configuration of your app
#[derive(Deserialize)]
struct Config {
    keyboard: KeybindsOld<Action>,
}

const CONFIG_FILE: &str = r#"
[keyboard]
"Esc" = "Exit"

# Standard bindings
"Up" = "Up"
"Down" = "Down"
"Left" = "Left"
"Right" = "Right"
"PageUp" = "Top"
"PageDown" = "Bottom"
"Home" = "Home"
"End" = "End"

# Emacs-like bindings
"Ctrl+p" = "Up"
"Ctrl+n" = "Down"
"Ctrl+b" = "Left"
"Ctrl+f" = "Right"
"Alt+<" = "Top"
"Alt+>" = "Bottom"
"Ctrl+a" = "Home"
"Ctrl+e" = "End"

# Vim-like bindings
"k" = "Up"
"j" = "Down"
"h" = "Left"
"l" = "Right"
"g g" = "Top"
"G" = "Bottom"
"^" = "Home"
"$" = "End"
"#;

fn main() -> io::Result<()> {
    // Parse the configuration from the file content
    let config: Config = toml::from_str(CONFIG_FILE).unwrap();

    // Create the key binding dispatcher to handle key input events
    let mut dispatcher = Keybinds::new(config.keyboard);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    while let Ok(event) = event::read() {
        // If the event triggered some action, handle it using `match`
        if let Some(action) = dispatcher.dispatch(&event) {
            match action {
                Action::Exit => break,
                Action::Up => execute!(stdout, cursor::MoveUp(1))?,
                Action::Down => execute!(stdout, cursor::MoveDown(1))?,
                Action::Left => execute!(stdout, cursor::MoveLeft(1))?,
                Action::Right => execute!(stdout, cursor::MoveRight(1))?,
                Action::Top => execute!(stdout, cursor::MoveUp(9999))?,
                Action::Bottom => execute!(stdout, cursor::MoveDown(9999))?,
                Action::Home => execute!(stdout, cursor::MoveLeft(9999))?,
                Action::End => execute!(stdout, cursor::MoveRight(9999))?,
            }
        }
    }
    disable_raw_mode()
}
