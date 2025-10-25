use std::mem;
use bytemuck::{Pod, Zeroable};

//Implements the pod trait so bitemuck can work with it
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f64; 3],
}
impl Vertex {
    // Returns layout (how the vertex should be read)
    // Pass into render pipeline
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            // Total number of bytes in a vertex
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            // Vertex buffer index by vertex
            step_mode: wgpu::VertexStepMode::Vertex,

            attributes: &[
                // First 2 float32s go to position attribute (0)
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // 3-5th float 32s go to color attribute (1)
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}
