use wgpu::{BindGroup, Color};

use crate::{AppGraphicsState, Widget};

// TODO: Widget must implement Default. Might want to set default values.
#[derive(Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    widget_render_data: Option<RectangleRenderData>,
}

struct RectangleRenderData {
    transform_bind: BindGroup,
    color_bind: BindGroup,
}

impl Widget for Rectangle {
    fn render(&self, render_pass: &mut wgpu::RenderPass, graphics_state: &AppGraphicsState) {
        if let Some(data) = &self.widget_render_data {
            render_pass.set_pipeline(graphics_state.pipelines.solid_color());
            // 0: transform
            render_pass.set_bind_group(0, &data.transform_bind, &[]);
            // 1: color
            render_pass.set_bind_group(0, &data.color_bind, &[]);
            render_pass.set_vertex_buffer(0, graphics_state.square_vertices.slice(..));
            render_pass.draw(0..6, 0..1);
        }
    }
}
