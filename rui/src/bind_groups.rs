use bytemuck::{AnyBitPattern, NoUninit};
use wgpu::{
    BindGroupLayoutEntry, BindingType, BufferBindingType, BufferUsages, ShaderStages,
    util::BufferInitDescriptor,
};

// TODO: Rotation (needed?)
#[repr(C)]
#[derive(Clone, Copy, NoUninit, AnyBitPattern)]
pub struct WidgetTransformUniform {
    pub x: f32,
    pub y: f32,
    pub sx: f32,
    pub sy: f32,
}

pub fn widget_transform_layout(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::VERTEX,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub fn color_uniform_layout(binding: u32) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility: ShaderStages::FRAGMENT,
        ty: BindingType::Buffer {
            ty: BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

pub fn transform_buffer_descriptor<'a>(
    transform: &'a WidgetTransformUniform,
) -> BufferInitDescriptor<'a> {
    BufferInitDescriptor {
        label: None,
        contents: bytemuck::bytes_of(transform),
        usage: BufferUsages::UNIFORM,
    }
}
pub fn color_buffer_descriptor<'a>(color: &'a crate::Color) -> BufferInitDescriptor<'a> {
    BufferInitDescriptor {
        label: None,
        contents: bytemuck::bytes_of(color),
        usage: BufferUsages::UNIFORM,
    }
}
