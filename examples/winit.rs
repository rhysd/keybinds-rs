use keybinds::winit::WinitEventConverter;
use keybinds::{Key, Keybinds};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Theme, Window, WindowId};

// Actions dispatched by key bindings
#[derive(Debug)]
enum Action {
    SayHi,
    ToggleMaximized,
    ToggleTheme,
    Exit,
}

struct App {
    window: Option<Window>,
    keybinds: Keybinds<Action>,
    converter: WinitEventConverter,
}

impl Default for App {
    fn default() -> Self {
        // Create a key bindings dispatcher to dispatch actions for upcoming key inputs
        let mut keybinds = Keybinds::default();

        // Key bindings to dispatch the actions
        keybinds.bind("h i", Action::SayHi).unwrap();
        keybinds.bind("Mod+m", Action::ToggleMaximized).unwrap();
        keybinds.bind("Mod+Alt+t", Action::ToggleTheme).unwrap();
        keybinds.bind("Mod+x Mod+c", Action::Exit).unwrap();

        Self {
            window: None,
            keybinds,
            converter: WinitEventConverter::default(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes().with_theme(Some(Theme::Dark));
        let window = event_loop.create_window(attrs).unwrap();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // Convert the window event into key input
        let input = self.converter.convert(&event);
        if input.key() != Key::Ignored {
            println!("Key input: {input:?}");
        }

        // Check if the converted key input dispatches some action
        if let Some(action) = self.keybinds.dispatch(input) {
            println!("Action: {action:?}");

            match action {
                Action::SayHi => println!("Hi!"),
                Action::ToggleMaximized => {
                    let window = self.window.as_ref().unwrap();
                    window.set_maximized(!window.is_maximized());
                }
                Action::ToggleTheme => {
                    let window = self.window.as_ref().unwrap();
                    let theme = match window.theme() {
                        Some(Theme::Dark) => Theme::Light,
                        _ => Theme::Dark,
                    };
                    window.set_theme(Some(theme));
                }
                Action::Exit => event_loop.exit(),
            }
        }

        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut App::default()).unwrap();
}
