struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct WidgetTransform {
    position: vec2<f32>,
    scale: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> widget_transform: WidgetTransform;
@group(0) @binding(1)
var<uniform> color: vec4<f32>;

// TODO: Change these to relative coordinates. Probably will involve a UBO with DPI info and viewport size.
@vertex
fn vs_main(
    vertex: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    // Apply widget transform
    let new_position = widget_transform.position + vertex.position * widget_transform.scale;

    // Transform to clip space
    let cs = (vec2<f32>(new_position.x, -new_position.y) * 2.0) + vec2<f32>(-1.0, 1.0);
    out.clip_position = vec4<f32>(cs, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return color;
}
