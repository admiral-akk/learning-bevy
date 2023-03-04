

@group(1) @binding(0)
var<uniform> time: f32;
@group(1) @binding(1)
var<uniform> stroke_duration: f32;
@group(1) @binding(2)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(3)
var base_color_sampler: sampler;
@group(1) @binding(4)
var mask_texture: texture_2d<f32>;
@group(1) @binding(5)
var mask_texture_sampler: sampler;
@group(1) @binding(6)
var<uniform> base: f32;
@group(1) @binding(7)
var<uniform> alpha: f32;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    var t = (2.0-base) * smoothstep(0., 1.,time / stroke_duration);
    var tex_delay = textureSample(mask_texture, mask_texture_sampler, uv).x;
    var progress = base + clamp(t - tex_delay, 0., 1. - base);
    var color = vec4(vec3(progress), mix(1., progress, alpha)) * textureSample(base_color_texture, base_color_sampler, uv);
    return color;
}