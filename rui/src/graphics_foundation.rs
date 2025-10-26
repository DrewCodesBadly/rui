use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu::{Device, util::DeviceExt};

// Implements the pod trait so bitemuck can work with it
// Shouldn't need data besides positon and uvs?
// We don't need, like, normals or lighting info so it should be okay if it stays simple.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}
impl Vertex {
    // Returns layout (how the vertex should be read)
    // Pass into render pipeline
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            // Total number of bytes in a vertex
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // Vertex buffer index by vertex
            step_mode: wgpu::VertexStepMode::Vertex,

            attributes: &[
                // First 2 f32s go to position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // Second two are UVs
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub fn get_square_vertex_buffer(device: &Device) -> wgpu::Buffer {
    let square_vertices_raw = [
        // 1st triangle
        Vertex {
            position: [0.0, 0.0],
            uv: [0.0, 0.0],
        },
        Vertex {
            position: [0.0, 1.0],
            uv: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
            uv: [1.0, 1.0],
        },
        // 2nd triangle
        Vertex {
            position: [1.0, 1.0],
            uv: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, 0.0],
            uv: [1.0, 0.0],
        },
        Vertex {
            position: [0.0, 0.0],
            uv: [0.0, 0.0],
        },
    ];

    // initializes vertex buffer
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Rectangle vert buffer"),
        contents: bytemuck::cast_slice(&square_vertices_raw),
        usage: wgpu::BufferUsages::VERTEX,
    })
}
