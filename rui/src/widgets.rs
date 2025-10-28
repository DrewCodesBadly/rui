use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutEntry, BindingResource,
    BindingType, Buffer, BufferBinding, BufferBindingType, BufferDescriptor, BufferUsages,
    ShaderStages, util::DeviceExt,
};

use crate::{
    AppGraphicsState, Color,
    bind_groups::{WidgetTransformUniform, color_buffer_descriptor, transform_buffer_descriptor},
};

// TODO: Figure out how to initialize and uninitialize widget resources.
/// Trait for any struct representing a widget in the framework. These structs contain
/// all of the data of their corresponding widget's state, and have a bound ephemeral type
/// which contains handles to any needed GPU resources that might be freed if the widget is
/// off screen.
pub trait Widget<E> {
    /// Draws the widget to the screen during a render pass.
    fn render(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        graphics_state: &AppGraphicsState,
        ephemeral: &E,
    );

    /// Destroys any data associated with the widget. Called when the widget is no longer
    /// on screen.
    fn destroy(&mut self);

    /// Updates ephemeral data from own state data. Called whenever the widget's internal state
    /// is changed.
    fn update_ephemeral(&self, graphics_state: &AppGraphicsState, old: Option<E>) -> E;
}

// TODO: Widget must implement Default. Might want to set default values.
#[derive(Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

pub struct RectangleEphemeral {
    pub transform_buffer: Buffer,
    pub color_buffer: Buffer,
    pub bind_group: BindGroup,
}

impl RectangleEphemeral {
    pub fn destroy(&mut self) {
        self.transform_buffer.destroy();
        self.color_buffer.destroy();
    }
}

impl Widget<RectangleEphemeral> for Rectangle {
    fn render(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        graphics_state: &AppGraphicsState,
        ephemeral: &RectangleEphemeral,
    ) {
        render_pass.set_pipeline(&graphics_state.pipelines.solid_color);
        render_pass.set_bind_group(0, &ephemeral.bind_group, &[]);
        render_pass.set_vertex_buffer(0, graphics_state.square_vertices.slice(..));
        render_pass.draw(0..6, 0..1);
    }

    fn destroy(&mut self) {}

    fn update_ephemeral(
        &self,
        graphics_state: &AppGraphicsState,
        old: Option<RectangleEphemeral>,
    ) -> RectangleEphemeral {
        if let Some(_) = old {
            old.unwrap()
        } else {
            let transform_buffer =
                graphics_state
                    .device
                    .create_buffer_init(&transform_buffer_descriptor(&WidgetTransformUniform {
                        x: self.x,
                        y: self.y,
                        sx: self.width,
                        sy: self.height,
                    }));
            let color_buffer = graphics_state
                .device
                .create_buffer_init(&color_buffer_descriptor(&self.color));

            let bind_group = graphics_state
                .device
                .create_bind_group(&BindGroupDescriptor {
                    label: None,
                    layout: &graphics_state.pipelines.solid_color_bind_layout,
                    entries: &[
                        BindGroupEntry {
                            binding: 0,
                            resource: BindingResource::Buffer(
                                transform_buffer.as_entire_buffer_binding(),
                            ),
                        },
                        BindGroupEntry {
                            binding: 1,
                            resource: BindingResource::Buffer(
                                color_buffer.as_entire_buffer_binding(),
                            ),
                        },
                    ],
                });

            RectangleEphemeral {
                transform_buffer,
                color_buffer,
                bind_group,
            }
        }
    }
}
