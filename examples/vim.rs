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
use std::io::{self, BufRead};
use tui_textarea::{CursorMove, Input, Key, Scrolling, TextArea};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Yank,
    Change,
    Delete,
}

impl Operator {
    fn operate(self, textarea: &mut TextArea<'_>) {
        match self {
            Operator::Yank => textarea.copy(),
            Operator::Change | Operator::Delete => {
                textarea.cut();
            }
        }
    }
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

impl Action {
    fn is_operatable(self, mode: Mode) -> bool {
        mode == Mode::Normal
            && matches!(
                self,
                Action::Back
                    | Action::Down
                    | Action::Up
                    | Action::Forward
                    | Action::WordForward
                    | Action::WordEnd
                    | Action::WordBack
                    | Action::Head
                    | Action::End
                    | Action::ScrollDown
                    | Action::ScrollUp
                    | Action::ScrollHalfPageDown
                    | Action::ScrollHalfPageUp
                    | Action::ScrollPageDown
                    | Action::ScrollPageUp
                    | Action::ScrollTop
                    | Action::ScrollBottom
                    | Action::Operator(_)
            )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Normal,
    Insert,
    Visual,
}

impl Mode {
    fn block<'a>(&self) -> Block<'a> {
        let help = match self {
            Self::Normal => "type q to quit, type i to enter insert mode",
            Self::Insert => "type Esc to back to normal mode",
            Self::Visual => "type y to yank, type d to delete, type Esc to back to normal mode",
        };
        let title = format!("{} MODE ({})", self, help);
        Block::default().borders(Borders::ALL).title(title)
    }

    fn cursor_style(&self) -> Style {
        let color = match self {
            Self::Normal => Color::Reset,
            Self::Insert => Color::LightBlue,
            Self::Visual => Color::LightYellow,
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
        }
    }
}

struct Vim<'a> {
    mode: Mode,
    normal: Keybinds<Action>,
    visual: Keybinds<Action>,
    insert: Keybinds<Action>,
    pending: Option<Operator>,
    textarea: TextArea<'a>,
}

impl<'a> Vim<'a> {
    fn new(mut textarea: TextArea<'a>) -> keybinds::Result<Self> {
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

        let mode = Mode::Normal;
        textarea.set_block(mode.block());
        textarea.set_cursor_style(mode.cursor_style());

        Ok(Self {
            mode,
            normal,
            visual,
            insert,
            pending: None,
            textarea,
        })
    }

    fn transition(&self, action: Action) -> Option<Mode> {
        match action {
            Action::DeleteEnd
            | Action::Paste
            | Action::Undo
            | Action::Redo
            | Action::DeleteChar
            | Action::Normal => Some(Mode::Normal),
            Action::ChangeEnd
            | Action::Insert
            | Action::InsertNext
            | Action::InsertEnd
            | Action::InsertNextLine
            | Action::InsertPrevLine
            | Action::InsertHead => Some(Mode::Insert),
            Action::Visual | Action::VisualLine => Some(Mode::Visual),
            Action::Quit => None,
            Action::Operator(op) if matches!(self.mode, Mode::Visual) => match op {
                Operator::Yank => Some(Mode::Normal),
                Operator::Change => Some(Mode::Insert),
                Operator::Delete => Some(Mode::Normal),
            },
            _ => match self.pending {
                Some(Operator::Yank) => Some(Mode::Normal),
                Some(Operator::Delete) => Some(Mode::Normal),
                Some(Operator::Change) => Some(Mode::Insert),
                _ => Some(self.mode),
            },
        }
    }

    fn edit(&mut self, action: Action) {
        match action {
            Action::Back => self.textarea.move_cursor(CursorMove::Back),
            Action::Down => self.textarea.move_cursor(CursorMove::Down),
            Action::Up => self.textarea.move_cursor(CursorMove::Up),
            Action::Forward => self.textarea.move_cursor(CursorMove::Forward),
            Action::WordForward => self.textarea.move_cursor(CursorMove::WordForward),
            Action::WordEnd => {
                self.textarea.move_cursor(CursorMove::WordEnd);
                if self.pending.is_some() {
                    self.textarea.move_cursor(CursorMove::Forward); // Include the text under the cursor
                }
            }
            Action::WordBack => self.textarea.move_cursor(CursorMove::WordBack),
            Action::Head => self.textarea.move_cursor(CursorMove::Head),
            Action::End => self.textarea.move_cursor(CursorMove::End),
            Action::DeleteEnd => {
                self.textarea.delete_line_by_end();
            }
            Action::ChangeEnd => {
                self.textarea.delete_line_by_end();
                self.textarea.cancel_selection();
            }
            Action::Paste => {
                self.textarea.paste();
            }
            Action::Undo => {
                self.textarea.undo();
            }
            Action::Redo => {
                self.textarea.redo();
            }
            Action::DeleteChar => {
                self.textarea.delete_next_char();
            }
            Action::Insert => {
                self.textarea.cancel_selection();
            }
            Action::InsertNext => {
                self.textarea.cancel_selection();
                self.textarea.move_cursor(CursorMove::Forward);
            }
            Action::InsertEnd => {
                self.textarea.cancel_selection();
                self.textarea.move_cursor(CursorMove::End);
            }
            Action::InsertNextLine => {
                self.textarea.move_cursor(CursorMove::End);
                self.textarea.insert_newline();
            }
            Action::InsertPrevLine => {
                self.textarea.move_cursor(CursorMove::Head);
                self.textarea.insert_newline();
                self.textarea.move_cursor(CursorMove::Up);
            }
            Action::InsertHead => {
                self.textarea.cancel_selection();
                self.textarea.move_cursor(CursorMove::Head);
            }
            Action::Quit => {}
            Action::ScrollDown => self.textarea.scroll((1, 0)),
            Action::ScrollUp => self.textarea.scroll((-1, 0)),
            Action::ScrollHalfPageDown => self.textarea.scroll(Scrolling::HalfPageDown),
            Action::ScrollHalfPageUp => self.textarea.scroll(Scrolling::HalfPageUp),
            Action::ScrollPageDown => self.textarea.scroll(Scrolling::PageDown),
            Action::ScrollPageUp => self.textarea.scroll(Scrolling::PageUp),
            Action::ScrollTop => self.textarea.move_cursor(CursorMove::Top),
            Action::ScrollBottom => self.textarea.move_cursor(CursorMove::Bottom),
            Action::Visual => {
                self.textarea.start_selection();
            }
            Action::VisualLine => {
                self.textarea.move_cursor(CursorMove::Head);
                self.textarea.start_selection();
                self.textarea.move_cursor(CursorMove::End);
            }
            Action::Normal => {
                self.textarea.cancel_selection();
            }
            Action::Operator(op) => {
                match self.mode {
                    Mode::Normal => {
                        if self.pending == Some(op) {
                            // Handle yy, dd, cc. (This is not strictly the same behavior as Vim)
                            self.textarea.move_cursor(CursorMove::Head);
                            self.textarea.start_selection();
                            let cursor = self.textarea.cursor();
                            self.textarea.move_cursor(CursorMove::Down);
                            if cursor == self.textarea.cursor() {
                                self.textarea.move_cursor(CursorMove::End); // At the last line, move to end of the line instead
                            }
                        } else {
                            self.pending = Some(op);
                            self.textarea.start_selection();
                            return; // Edge case where `self.pending` should not be cleared
                        }
                    }
                    Mode::Visual => {
                        self.textarea.move_cursor(CursorMove::Forward); // Vim's text selection is inclusive
                        op.operate(&mut self.textarea);
                    }
                    Mode::Insert => {}
                }
            }
        }

        if let Some(op) = self.pending.take() {
            if action.is_operatable(self.mode) {
                op.operate(&mut self.textarea);
            }
        }
    }

    fn dispatch(&mut self, input: KeyInput) -> Option<Action> {
        let keybinds = match self.mode {
            Mode::Normal => &mut self.normal,
            Mode::Visual => &mut self.visual,
            Mode::Insert => &mut self.insert,
        };
        keybinds.dispatch(input).copied()
    }

    fn convert_key_input(&self, input: KeyInput) -> Option<Input> {
        if self.mode != Mode::Insert {
            return None;
        }

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
            _ => return None,
        };

        Some(Input {
            key,
            ctrl: input.mods().contains(Mods::CTRL),
            alt: input.mods().contains(Mods::ALT),
            shift: input.mods().contains(Mods::SHIFT),
        })
    }

    fn input(&mut self, input: KeyInput) -> bool {
        if let Some(action) = self.dispatch(input) {
            let Some(next) = self.transition(action) else {
                return false;
            };
            self.edit(action);
            if self.mode != next {
                self.textarea.set_block(next.block());
                self.textarea.set_cursor_style(next.cursor_style());
            }
            self.mode = next;
        } else if let Some(input) = self.convert_key_input(input) {
            self.textarea.input(input);
        }
        true
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout().lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let textarea = if let Some(path) = env::args().nth(1) {
        io::BufReader::new(fs::File::open(path)?)
            .lines()
            .collect::<io::Result<_>>()?
    } else {
        TextArea::default()
    };

    let mut vim = Vim::new(textarea).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    loop {
        term.draw(|f| f.render_widget(&vim.textarea, f.area()))?;

        if !vim.input(crossterm::event::read()?.into()) {
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
