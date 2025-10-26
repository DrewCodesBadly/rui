pub use rui_macros;
// Re-exports depended on by the macros
pub use raw_window_handle;
pub use wgpu;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, DeviceDescriptor, InstanceDescriptor,
    RequestAdapterOptions, SurfaceConfiguration, SurfaceTarget, TextureUsages,
    TextureViewDescriptor, util::DeviceExt,
};

pub use pipelines::AppPipelines;

use crate::graphics_foundation::get_square_vertex_buffer;
mod graphics_foundation;
mod pipelines;
pub mod widgets;

pub enum AppEvent {}

/// Trait for the app's state, implemented for states created by [`rui_macros::generate_app_state`]
pub trait AppState {
    fn render(&mut self);

    // TODO: Restructure event queue
    fn handle_event(&mut self, event: AppEvent);
}

#[derive(Debug)]
pub enum AppStateCreationError {
    TODOERROR,
    CreateSurfaceError(wgpu::CreateSurfaceError),
    RequestAdapterError(wgpu::RequestAdapterError),
    RequestDeviceError(wgpu::RequestDeviceError),
}

/// Struct containing everything necessary to communicate with graphics APIs and draw to the screen.
pub struct AppGraphicsState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    pipelines: AppPipelines,
    square_vertices: wgpu::Buffer,
}

impl AppGraphicsState {
    /// Called by the generate_app_state macro in rui_macros. Gets needed WGPU structs from handles.
    pub async fn new(
        window_handle: impl Into<SurfaceTarget<'static>>,
        width: u32,
        height: u32,
    ) -> Result<Self, AppStateCreationError> {
        let instance = wgpu::Instance::new(&InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance
            .create_surface(window_handle)
            .map_err(|e| AppStateCreationError::CreateSurfaceError(e))?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .map_err(|e| AppStateCreationError::RequestAdapterError(e))?;

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                ..Default::default()
            })
            .await
            .map_err(|e| AppStateCreationError::RequestDeviceError(e))?;

        let surface_caps = surface.get_capabilities(&adapter);
        let format = surface_caps
            .formats
            .iter()
            .find(|t| t.is_srgb())
            .cloned()
            .unwrap_or(surface_caps.formats.first().unwrap().clone());
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width,
            height,
            present_mode: surface_caps.present_modes.first().unwrap().clone(),
            alpha_mode: surface_caps.alpha_modes.first().unwrap().clone(),
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        let pipelines = AppPipelines::new(&device, &surface_config);

        // Square vertex buffer reused by most things
        let square_vertices = get_square_vertex_buffer(&device);

        Ok(Self {
            surface,
            surface_config,
            device,
            queue,
            pipelines,
            square_vertices,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn start_render(&mut self, widget: &impl Widget) -> anyhow::Result<()> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor {
            // TODO: Any needed overrides?
            ..Default::default()
        });
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Frame render encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Rectangle!"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                // View the render pass applies to
                view: &view,
                // useless
                resolve_target: None,

                ops: wgpu::Operations {
                    // Clear view before rendering to it.
                    // TODO: Not do this.
                    // load: wgpu::LoadOp::Clear(Color {
                    //     r: 0.0,
                    //     g: 0.0,
                    //     b: 0.0,
                    //     a: 1.0,
                    // }),
                    load: wgpu::LoadOp::Load,
                    // Store the new data in the view (as in actually edit it)
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            // useless
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        widget.render(&mut render_pass, &self);

        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();
        Ok(())
    }
}

// TODO: Figure out how to initialize and uninitialize widget resources.
pub trait Widget {
    fn render(&self, render_pass: &mut wgpu::RenderPass, graphics_state: &AppGraphicsState);
}
