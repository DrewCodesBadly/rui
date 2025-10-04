use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
pub use rui_macros;
// Re-exports depended on by the macros
pub use raw_window_handle;
pub use wgpu;
use wgpu::{Backends, InstanceDescriptor, wgc::instance::Instance};

pub trait AppState {
    // TODO: Define this trait

    fn render() {}
}

#[derive(Debug)]
pub enum AppStateCreationError {
    TODOERROR,
    CreateSurfaceError(wgpu::CreateSurfaceError),
}

/// Struct containing everything necessary to communicate with graphics APIs and draw to the screen.
pub struct AppGraphicsState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
}

impl AppGraphicsState {
    /// Called by the generate_app_state macro in rui_macros. Gets needed WGPU structs from handles.
    pub async fn new(
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
    ) -> Result<Self, AppStateCreationError> {
        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        Err(AppStateCreationError::TODOERROR)
    }
}
