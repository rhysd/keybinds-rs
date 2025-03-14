use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use keybinds::{KeyInput, KeySeq, Keybind, Keybinds, Mods};
use ratatui::backend::CrosstermBackend;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use ratatui::Terminal;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

fn convert_key_input(input: KeyInput) -> Input {
    let key = match input.key() {
        keybinds::Key::Char(c) => Key::Char(c),
        keybinds::Key::Copy => Key::Copy,
        keybinds::Key::Cut => Key::Cut,
        keybinds::Key::Delete => Key::Delete,
        keybinds::Key::Enter => Key::Enter,
        keybinds::Key::Up => Key::Up,
        keybinds::Key::Right => Key::Right,
        keybinds::Key::Down => Key::Down,
        keybinds::Key::Left => Key::Left,
        keybinds::Key::Home => Key::Home,
        keybinds::Key::End => Key::End,
        keybinds::Key::PageUp => Key::PageUp,
        keybinds::Key::PageDown => Key::PageDown,
        keybinds::Key::Tab => Key::Tab,
        _ => Key::Null,
    };
    Input {
        key,
        ctrl: input.mods().contains(Mods::CTRL),
        alt: input.mods().contains(Mods::ALT),
        shift: input.mods().contains(Mods::SHIFT),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Yank,
    Change,
    Delete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Forward,
    Back,
    Down,
    Up,
    WordForward,
    WordEnd,
    WordBack,
    Head,
    End,
    DeleteEnd,
    ChangeEnd,
    Paste,
    Undo,
    Redo,
    DeleteChar,
    Insert,
    InsertNext,
    InsertEnd,
    InsertNextLine,
    InsertPrevLine,
    InsertHead,
    Quit,
    ScrollDown,
    ScrollUp,
    ScrollHalfPageDown,
    ScrollHalfPageUp,
    ScrollPageDown,
    ScrollPageUp,
    ScrollTop,
    ScrollBottom,
    Visual,
    VisualLine,
    Normal,
    Operator(Operator),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Normal,
    Insert,
    Visual,
    Operator(Operator),
}

impl Mode {
    fn transition(self, action: Action, textarea: &mut TextArea<'_>) -> Option<Self> {
        match action {
            Action::Back => textarea.move_cursor(CursorMove::Back),
            Action::Down => textarea.move_cursor(CursorMove::Down),
            Action::Up => textarea.move_cursor(CursorMove::Up),
            Action::Forward => textarea.move_cursor(CursorMove::Forward),
            Action::WordForward => textarea.move_cursor(CursorMove::WordForward),
            Action::WordEnd => {
                textarea.move_cursor(CursorMove::WordEnd);
                if let Self::Operator(_) = self {
                    textarea.move_cursor(CursorMove::Forward); // Include the text under the cursor
                }
            }
            Action::WordBack => textarea.move_cursor(CursorMove::WordBack),
            Action::Head => textarea.move_cursor(CursorMove::Head),
            Action::End => textarea.move_cursor(CursorMove::End),
            Action::DeleteEnd => {
                textarea.delete_line_by_end();
                return Some(Self::Normal);
            }
            Action::ChangeEnd => {
                textarea.delete_line_by_end();
                textarea.cancel_selection();
                return Some(Self::Insert);
            }
            Action::Paste => {
                textarea.paste();
                return Some(Self::Normal);
            }
            Action::Undo => {
                textarea.undo();
                return Some(Self::Normal);
            }
            Action::Redo => {
                textarea.redo();
                return Some(Self::Normal);
            }
            Action::DeleteChar => {
                textarea.delete_next_char();
                return Some(Self::Normal);
            }
            Action::Insert => {
                textarea.cancel_selection();
                return Some(Self::Insert);
            }
            Action::InsertNext => {
                textarea.cancel_selection();
                textarea.move_cursor(CursorMove::Forward);
                return Some(Self::Insert);
            }
            Action::InsertEnd => {
                textarea.cancel_selection();
                textarea.move_cursor(CursorMove::End);
                return Some(Self::Insert);
            }
            Action::InsertNextLine => {
                textarea.move_cursor(CursorMove::End);
                textarea.insert_newline();
                return Some(Self::Insert);
            }
            Action::InsertPrevLine => {
                textarea.move_cursor(CursorMove::Head);
                textarea.insert_newline();
                textarea.move_cursor(CursorMove::Up);
                return Some(Self::Insert);
            }
            Action::InsertHead => {
                textarea.cancel_selection();
                textarea.move_cursor(CursorMove::Head);
                return Some(Self::Insert);
            }
            Action::Quit => return None,
            Action::ScrollDown => textarea.scroll((1, 0)),
            Action::ScrollUp => textarea.scroll((-1, 0)),
            Action::ScrollHalfPageDown => textarea.scroll(Scrolling::HalfPageDown),
            Action::ScrollHalfPageUp => textarea.scroll(Scrolling::HalfPageUp),
            Action::ScrollPageDown => textarea.scroll(Scrolling::PageDown),
            Action::ScrollPageUp => textarea.scroll(Scrolling::PageUp),
            Action::ScrollTop => textarea.move_cursor(CursorMove::Top),
            Action::ScrollBottom => textarea.move_cursor(CursorMove::Bottom),
            Action::Visual => {
                textarea.start_selection();
                return Some(Self::Visual);
            }
            Action::VisualLine => {
                textarea.move_cursor(CursorMove::Head);
                textarea.start_selection();
                textarea.move_cursor(CursorMove::End);
                return Some(Self::Visual);
            }
            Action::Normal => {
                textarea.cancel_selection();
                return Some(Self::Normal);
            }
            Action::Operator(op) => {
                match self {
                    Self::Normal => {
                        textarea.start_selection();
                        return Some(Self::Operator(op));
                    }
                    Self::Visual => match op {
                        Operator::Yank => {
                            textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                            textarea.copy();
                            return Some(Self::Normal);
                        }
                        Operator::Change => {
                            textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                            textarea.cut();
                            return Some(Self::Insert);
                        }
                        Operator::Delete => {
                            textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                            textarea.cut();
                            return Some(Self::Normal);
                        }
                    },
                    Self::Operator(o) if op == o => {
                        // Handle yy, dd, cc. (This is not strictly the same behavior as Vim)
                        textarea.move_cursor(CursorMove::Head);
                        textarea.start_selection();
                        let cursor = textarea.cursor();
                        textarea.move_cursor(CursorMove::Down);
                        if cursor == textarea.cursor() {
                            textarea.move_cursor(CursorMove::End); // At the last line, move to end of the line instead
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Self::Operator(op) = self {
            match op {
                Operator::Yank => {
                    textarea.copy();
                    Some(Self::Normal)
                }
                Operator::Delete => {
                    textarea.cut();
                    Some(Self::Normal)
                }
                Operator::Change => {
                    textarea.cut();
                    Some(Self::Insert)
                }
            }
        } else {
            Some(self)
        }
    }

    fn block<'a>(&self) -> Block<'a> {
        let help = match self {
            Self::Normal => "type q to quit, type i to enter insert mode",
            Self::Insert => "type Esc to back to normal mode",
            Self::Visual => "type y to yank, type d to delete, type Esc to back to normal mode",
            Self::Operator(_) => "move cursor to apply operator",
        };
        let title = format!("{} MODE ({})", self, help);
        Block::default().borders(Borders::ALL).title(title)
    }

    fn cursor_style(&self) -> Style {
        let color = match self {
            Self::Normal => Color::Reset,
            Self::Insert => Color::LightBlue,
            Self::Visual => Color::LightYellow,
            Self::Operator(_) => Color::LightGreen,
        };
        Style::default().fg(color).add_modifier(Modifier::REVERSED)
    }
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Insert => write!(f, "INSERT"),
            Self::Visual => write!(f, "VISUAL"),
            Self::Operator(op) => write!(f, "OPERATOR({op:?})"),
        }
    }
}

struct VimKeybinds {
    normal: Keybinds<Action>,
    visual: Keybinds<Action>,
    insert: Keybinds<Action>,
}

impl VimKeybinds {
    fn new() -> keybinds::Result<Self> {
        fn keybinds(map: &[(&str, Action)]) -> keybinds::Result<Keybinds<Action>> {
            map.iter()
                .copied()
                .map(|(k, a)| k.parse().map(|s: KeySeq| Keybind::new(s, a)))
                .collect()
        }

        let normal = keybinds(&[
            ("h", Action::Back),
            ("j", Action::Down),
            ("k", Action::Up),
            ("l", Action::Forward),
            ("w", Action::WordForward),
            ("e", Action::WordEnd),
            ("b", Action::WordBack),
            ("^", Action::Head),
            ("$", Action::End),
            ("D", Action::DeleteEnd),
            ("C", Action::ChangeEnd),
            ("p", Action::Paste),
            ("u", Action::Undo),
            ("Ctrl+r", Action::Redo),
            ("x", Action::DeleteChar),
            ("i", Action::Insert),
            ("a", Action::InsertNext),
            ("A", Action::InsertEnd),
            ("o", Action::InsertNextLine),
            ("O", Action::InsertPrevLine),
            ("q", Action::Quit),
            ("Ctrl+e", Action::ScrollDown),
            ("Ctrl+y", Action::ScrollUp),
            ("Ctrl+d", Action::ScrollHalfPageDown),
            ("Ctrl+u", Action::ScrollHalfPageUp),
            ("Ctrl+f", Action::ScrollPageDown),
            ("Ctrl+b", Action::ScrollPageUp),
            ("g g", Action::ScrollTop),
            ("G", Action::ScrollBottom),
            ("v", Action::Visual),
            ("V", Action::VisualLine),
            ("y", Action::Operator(Operator::Yank)),
            ("d", Action::Operator(Operator::Delete)),
            ("c", Action::Operator(Operator::Change)),
        ])?;

        let visual = keybinds(&[
            ("h", Action::Back),
            ("j", Action::Down),
            ("k", Action::Up),
            ("l", Action::Forward),
            ("w", Action::WordForward),
            ("e", Action::WordEnd),
            ("b", Action::WordBack),
            ("^", Action::Head),
            ("$", Action::End),
            ("D", Action::DeleteEnd),
            ("C", Action::ChangeEnd),
            ("p", Action::Paste),
            ("u", Action::Undo),
            ("Ctrl+r", Action::Redo),
            ("x", Action::DeleteChar),
            ("i", Action::Insert),
            ("a", Action::InsertNext),
            ("I", Action::InsertHead),
            ("A", Action::InsertEnd),
            ("o", Action::InsertNextLine),
            ("O", Action::InsertPrevLine),
            ("q", Action::Quit),
            ("Ctrl+e", Action::ScrollDown),
            ("Ctrl+y", Action::ScrollUp),
            ("Ctrl+d", Action::ScrollHalfPageDown),
            ("Ctrl+u", Action::ScrollHalfPageUp),
            ("Ctrl+f", Action::ScrollPageDown),
            ("Ctrl+b", Action::ScrollPageUp),
            ("g g", Action::ScrollTop),
            ("G", Action::ScrollBottom),
            ("v", Action::Normal),
            ("V", Action::Normal),
            ("y", Action::Operator(Operator::Yank)),
            ("d", Action::Operator(Operator::Delete)),
            ("c", Action::Operator(Operator::Change)),
            ("Esc", Action::Normal),
        ])?;

        let insert = keybinds(&[("Esc", Action::Normal)])?;

        Ok(Self {
            normal,
            visual,
            insert,
        })
    }

    fn dispatch(&mut self, input: KeyInput, mode: Mode) -> Option<Action> {
        let keybinds = match mode {
            Mode::Normal | Mode::Operator(_) => &mut self.normal,
            Mode::Visual => &mut self.visual,
            Mode::Insert => &mut self.insert,
        };
        keybinds.dispatch(input).copied()
    }
}

struct Vim<'a> {
    mode: Mode,
    keybinds: VimKeybinds,
    textarea: TextArea<'a>,
}

impl<'a> Vim<'a> {
    fn new(textarea: TextArea<'a>) -> keybinds::Result<Self> {
        Ok(Self {
            mode: Mode::Normal,
            keybinds: VimKeybinds::new()?,
            textarea,
        })
    }

    fn handle_action(&mut self, action: Action) -> bool {
        let Some(mode) = self.mode.transition(action, &mut self.textarea) else {
            return true;
        };
        if self.mode != mode {
            self.textarea.set_block(mode.block());
            self.textarea.set_cursor_style(mode.cursor_style());
        }
        self.mode = mode;
        false
    }

    fn handle_input(&mut self, input: KeyInput) -> bool {
        if let Some(action) = self.keybinds.dispatch(input, self.mode) {
            return self.handle_action(action);
        }
        self.textarea.input(convert_key_input(input));
        false
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout().lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = if let Some(path) = env::args().nth(1) {
        let file = fs::File::open(path)?;
        io::BufReader::new(file)
            .lines()
            .collect::<io::Result<_>>()?
    } else {
        TextArea::default()
    };

    textarea.set_block(Mode::Normal.block());
    textarea.set_cursor_style(Mode::Normal.cursor_style());
    let mut vim = Vim::new(textarea).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    loop {
        term.draw(|f| f.render_widget(&vim.textarea, f.area()))?;

        if vim.handle_input(crossterm::event::read()?.into()) {
            break;
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;

    println!("Lines: {:?}", vim.textarea.lines());

    Ok(())
}
