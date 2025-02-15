use keybinds::winit::WinitEventConverter;
use keybinds::{KeyBind, KeyBindMatcher, KeyBinds};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

// Actions triggered by key bindings
#[derive(Debug)]
enum Action {
    SayHi,
    ToggleMaximized,
    ToggleVisible,
    ExitApp,
}

struct App {
    window: Option<Window>,
    matcher: KeyBindMatcher<Action>,
    converter: WinitEventConverter,
}

impl Default for App {
    fn default() -> Self {
        // Key bindings to trigger the actions
        let keybinds = KeyBinds::new(vec![
            KeyBind::multiple("h i".parse().unwrap(), Action::SayHi),
            KeyBind::single("Mod+m".parse().unwrap(), Action::ToggleMaximized),
            KeyBind::single("Mod+Shift+v".parse().unwrap(), Action::ToggleVisible),
            KeyBind::multiple("Mod+x Mod+c".parse().unwrap(), Action::ExitApp),
        ]);
        Self {
            window: None,
            matcher: KeyBindMatcher::new(keybinds),
            converter: WinitEventConverter::default(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // Convert `WindowEvent` into `KeyInput` for checking key bindings. This returns "Unidentified"
        // key when the event is not for keyboard inputs.
        let input = self.converter.convert_window_event(&event);
        println!("Key input: {input:?}");

        if let Some(action) = self.matcher.trigger(input) {
            println!("Action: {action:?}");
            match action {
                Action::SayHi => println!("Hi!"),
                Action::ToggleMaximized => {
                    let window = self.window.as_ref().unwrap();
                    window.set_maximized(!window.is_maximized());
                }
                Action::ToggleVisible => {
                    let window = self.window.as_ref().unwrap();
                    window.set_visible(!window.is_visible().unwrap_or(false));
                }
                Action::ExitApp => event_loop.exit(),
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
