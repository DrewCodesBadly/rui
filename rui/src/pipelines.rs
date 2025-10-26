use wgpu::RenderPipeline;

pub struct AppPipelines {
    solid_color: RenderPipeline,
}

impl AppPipelines {
    pub fn new() -> Self {
        todo!()
    }

    pub fn solid_color(&self) -> &RenderPipeline {
        &self.solid_color
    }
}
