use iced::alignment::Horizontal;
use iced::event::{listen_with, Event};
use iced::widget::{button, column, container, row, text, Column};
use iced::{keyboard, window, Element, Length::Fill, Subscription, Task, Theme};
use keybinds::{Key, KeyInput, KeybindDispatcher};
use std::fmt::Write;

// Actions dispatched by the key bindings
#[derive(Clone, Copy, Debug)]
enum Action {
    SayHello,
    ToggleMaximize,
    ToggleTheme,
    Exit,
}

#[derive(Clone, Debug)]
enum Message {
    WindowOpen(window::Id),
    KeyEvent(keyboard::Event),
    Reset,
}

struct Example {
    window_id: window::Id,
    ongoing_input: String,
    last_action: String,
    keybinds: KeybindDispatcher<Action>,
    maximized: bool,
    theme: Theme,
    help: Vec<(String, Action)>,
}

impl Default for Example {
    fn default() -> Self {
        let mut keybinds = KeybindDispatcher::default();

        // Define the key bindings
        keybinds.bind("Mod+m", Action::ToggleMaximize).unwrap();
        keybinds.bind("H e l l o", Action::SayHello).unwrap();
        keybinds.bind("Mod+T", Action::ToggleTheme).unwrap();
        keybinds.bind("Mod+x Mod+c", Action::Exit).unwrap();

        Self {
            window_id: window::Id::unique(),
            ongoing_input: "".to_string(),
            last_action: "".to_string(),
            maximized: false,
            theme: Theme::Dark,
            help: keybinds
                .keybinds()
                .iter()
                .map(|b| (format!("{}", b.seq), b.action))
                .collect(),
            keybinds,
        }
    }
}

impl Example {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpen(id) => self.window_id = id,
            Message::KeyEvent(event) => {
                let input = KeyInput::from(&event);
                if input.key() != Key::Ignored {
                    println!("Key input: {input:?}");
                    if !self.keybinds.is_ongoing() {
                        self.last_action.clear();
                        self.ongoing_input.clear();
                    }
                    if !self.ongoing_input.is_empty() {
                        self.ongoing_input.push_str(" → ");
                    }
                    write!(self.ongoing_input, "{input:}").unwrap();
                }

                if let Some(action) = self.keybinds.dispatch(event) {
                    self.last_action = format!("{action:?}");

                    // Handle the dispatched action
                    match action {
                        Action::SayHello => println!("Hello!"),
                        Action::ToggleMaximize => {
                            self.maximized = !self.maximized;
                            return window::maximize(self.window_id, self.maximized);
                        }
                        Action::ToggleTheme => {
                            self.theme = match self.theme {
                                Theme::Dark => Theme::Light,
                                _ => Theme::Dark,
                            };
                        }
                        Action::Exit => return iced::exit(),
                    }
                }
            }
            Message::Reset => {
                self.keybinds.reset();
                self.ongoing_input.clear();
                self.last_action.clear();
            }
        }
        Task::none()
    }

    fn view(&self) -> impl Into<Element<Message>> {
        let help: Column<_> = self
            .help
            .iter()
            .map(|(key, action)| {
                row![
                    text("Key sequence "),
                    text(key).style(text::primary),
                    text(format!(" triggers {action:?}")),
                ]
                .into()
            })
            .collect();
        let inputs = text(&self.ongoing_input).size(32.0);
        let action = text(&self.last_action).style(text::primary).size(36.0);
        let reset = button("Reset").on_press(Message::Reset);
        let content = column![help, inputs, action, reset]
            .spacing(24.0)
            .padding(24.0)
            .align_x(Horizontal::Center)
            .width(Fill);
        container(content).center(Fill)
    }

    fn subscription(&self) -> Subscription<Message> {
        listen_with(|event, _, id| match event {
            Event::Window(window::Event::Opened { .. }) => Some(Message::WindowOpen(id)),
            Event::Keyboard(event) => Some(Message::KeyEvent(event)),
            _ => None,
        })
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn main() -> iced::Result {
    iced::application("Keybinds Example", Example::update, Example::view)
        .subscription(Example::subscription)
        .theme(Example::theme)
        .window_size((600.0, 400.0))
        .run()
}
