#import bevy_sprite::mesh2d_view_bind_group
#import bevy_sprite::mesh2d_struct

struct CustomMaterial {
    color: vec4<f32>;
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32;
};
let COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;

[[group(0), binding(0)]]
var<uniform> view: View;

[[group(1), binding(0)]]
var<uniform> material: CustomMaterial;
[[group(1), binding(1)]]
var texture: texture_2d<f32>;
[[group(1), binding(2)]]
var texture_sampler: sampler;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh2d;

// The structure of the vertex buffer is as specified in `specialize()`
struct Vertex {
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] position: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] world_position: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
};

/// Entry point for the vertex shader
[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
    // Project the world position of the mesh into screen position
    out.clip_position = view.view_proj * out.world_position;
    out.uv = vertex.uv;
    out.color = vertex.color;

    return out;
}

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] world_position: vec4<f32>;
    [[location(2)]] uv: vec2<f32>;
};

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    var output_color: vec4<f32> = material.color;
    var vert_color = in.color;
    if ((material.flags & COLOR_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
        output_color = output_color * textureSample(texture, texture_sampler, in.uv) * vert_color;
    }
    return output_color * vert_color;
    //return vec4<f32>(1.0,1.0,1.0,1.0);
}