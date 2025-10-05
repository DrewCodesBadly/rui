// Temporary, most of this code should get moved to the library later. Just getting a feel for what the structure is like.
// Winit window stuff should be handled separately if possible to allow embedding using another setup
use rui::rui_macros::generate_app_state;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::EventLoop,
    window::{Window, WindowAttributes},
};

#[derive(Default)]
struct AppGlobalState {
    window: Option<Window>,
}

generate_app_state!("main.rui", AppGlobalState, AppState);

// Most of this is taken straight from the learn wgpu guide with minimal modifications
// All of the wasm stuff is absent since I don't care yet
#[derive(Default)]
struct WinitApp {
    state: Option<AppState>,
}

impl ApplicationHandler<AppState> for WinitApp {
    // Where the window is created
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = event_loop
            .create_window(WindowAttributes::default())
            .expect("Unable to create window");
        let size = window.inner_size();
        self.state =
            Some(pollster::block_on(AppState::new(window, size.width, size.height)).unwrap());
    }

    // Looks to be pointless off of web
    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: AppState) {
        self.state = Some(event)
    }

    // Where window events are handled
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            // WindowEvent::Resized(size) =>
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = WinitApp::default();
    event_loop.run_app(&mut app);

    Ok(())
}
