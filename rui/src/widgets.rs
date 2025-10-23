use wgpu::util::DeviceExt;

use crate::{Widget, graphics_foundation::Vertex};

pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    // In 0-255 format
    color: [i8; 3],
}

// TODO: Fix
impl Widget for Rectangle {
    fn render(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        target_view: &wgpu::TextureView,
        render_pipeline: &wgpu::RenderPipeline,
    ) {
        // Converts to 0-1 format
        let color = wgpu::Color {
            r: self.color.0 as f64 / 255.0,
            g: self.color.1 as f64 / 255.0,
            b: self.color.2 as f64 / 255.0,
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

        // actual render pass
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Rectangle!"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                // View the render pass applies to
                view: target_view,
                // useless
                resolve_target: None,

                ops: wgpu::Operations {
                    // Keep previous stuff in view
                    load: wgpu::LoadOp::Load,
                    // Store the new data in the view (as in actually edit it)
                    store: true,
                },
            }],
            // useless
            depth_stencil_attachment: None,
        });

        rpass.set_pipeline(render_pipeline);
        rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        rpass.draw(0..4, 0..1);
    }
}
