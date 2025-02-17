use keybinds::winit::WinitEventConverter;
use keybinds::KeybindDispatcher;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Theme, Window, WindowId};

// Actions triggered by key bindings
#[derive(Debug)]
enum Action {
    SayHi,
    ToggleMaximized,
    ToggleTheme,
    Exit,
}

struct App {
    window: Option<Window>,
    dispatcher: KeybindDispatcher<Action>,
    converter: WinitEventConverter,
}

impl Default for App {
    fn default() -> Self {
        let mut dispatcher = KeybindDispatcher::default();

        // Key bindings to trigger the actions
        dispatcher.bind("h i", Action::SayHi).unwrap();
        dispatcher.bind("Mod+m", Action::ToggleMaximized).unwrap();
        dispatcher.bind("Mod+Shift+t", Action::ToggleTheme).unwrap();
        dispatcher.bind("Mod+x Mod+c", Action::Exit).unwrap();

        Self {
            window: None,
            dispatcher,
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

        // Check if the converted key input triggers some action
        if let Some(action) = self.dispatcher.trigger(input) {
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
