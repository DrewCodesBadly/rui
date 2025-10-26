use wgpu::util::DeviceExt;

use crate::{Widget, graphics_foundation::Vertex, pipelines::AppPipelines};

// TODO: Widget must implement Default. Might want to set default values.
#[derive(Default)]
pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    // In 0-255 format
    color: [u8; 3],
}

impl Widget for Rectangle {
    fn render(
        &self,
        device: &wgpu::Device,
        render_pass: &mut wgpu::RenderPass,
        pipelines: &AppPipelines,
    ) {
        // Converts to 0-1 format
        let color = wgpu::Color {
            r: self.color[0] as f64 / 255.0,
            g: self.color[1] as f64 / 255.0,
            b: self.color[2] as f64 / 255.0,
            a: 1.0,
        };

        // Defines vertices
        // top left, top right, bottom left, bottom right
        let vertices = [
            Vertex {
                position: [self.x, self.y],
                color: [color.r, color.g, color.b],
            },
            Vertex {
                position: [self.x + self.width, self.y],
                color: [color.r, color.g, color.b],
            },
            Vertex {
                position: [self.x, self.y + self.height],
                color: [color.r, color.g, color.b],
            },
            Vertex {
                position: [self.x + self.width, self.y + self.height],
                color: [color.r, color.g, color.b],
            },
        ];

        // initializes vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rectangle vert buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        render_pass.set_pipeline(pipelines.solid_color());
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..4, 0..1);
    }
}
