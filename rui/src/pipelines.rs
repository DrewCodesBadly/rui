use wgpu::{
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, BufferBindingType,
    ColorTargetState, ColorWrites, Device, FragmentState, MultisampleState,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, RenderPipeline,
    RenderPipelineDescriptor, ShaderStages, SurfaceConfiguration, VertexState, include_wgsl,
};

use crate::graphics_foundation::Vertex;

pub struct AppPipelines {
    solid_color: RenderPipeline,
}

impl AppPipelines {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        // TODO: Helper functions for this - I don't want to write this out 30 times.
        // A lot of it should be the same across pipelines, just a different shader and layout.
        let solid_color_shader =
            device.create_shader_module(include_wgsl!("shaders/solid_color.wgsl"));
        let solid_color_bind_groups = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Solid Color Binds"),
            entries: &[widget_transform_layout(0), color_uniform_layout(1)],
        });
        let solid_color_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Solid Color Layout"),
            bind_group_layouts: &[&solid_color_bind_groups],
            push_constant_ranges: &[],
        });

        let solid_color = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Solid Color Pipeline"),
            layout: Some(&solid_color_layout),
            vertex: VertexState {
                module: &solid_color_shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(FragmentState {
                module: &solid_color_shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::ALPHA_BLENDING), // use alpha blending
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                // usually the type of topology we want
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,        // No culling - we aren't in 3D so no need.
                unclipped_depth: false, // useless in 2D, but might be unsupported so who cares.
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false, // Don't care.
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,                         // No multisampling (yet)
                mask: !0,                         // Everything
                alpha_to_coverage_enabled: false, // No AA so I don't care.
            },
            multiview: None, // 1 view
            cache: None,     // Not going to bother with this yet.
        });

        Self { solid_color }
    }

    pub fn solid_color(&self) -> &RenderPipeline {
        &self.solid_color
    }
}

// Binding layouts and related structs

// TODO: Rotation (needed?)
struct WidgetTransformUniform {
    x: f32,
    y: f32,
    sx: f32,
    sy: f32,
}

fn widget_transform_layout(binding: u32) -> BindGroupLayoutEntry {
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

fn color_uniform_layout(binding: u32) -> BindGroupLayoutEntry {
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
