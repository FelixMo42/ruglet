struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};

///////////////////
// Vertex shader //
///////////////////

struct ScreenSize {
    w: f32,
    h: f32,
};

@group(0) @binding(0)
var<uniform> screen_size: ScreenSize;

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(
        model.position.x / screen_size.w - 1.0,
        -model.position.y / screen_size.h + 1.0,
        model.position.z,
        1.0
    );
    out.tex_coords = model.tex_coords;
    out.color = model.color;
    return out;
}

/////////////////////
// Fragment shader //
/////////////////////

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (in.tex_coords.x > 1.0) {
        return vec4<f32>(in.color, 1.0);
    }

    return textureSample(t_diffuse, s_diffuse, in.tex_coords) * vec4<f32>(in.color, 1.0);
}
