#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::view

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var scene_texture: texture_2d<f32>;
@group(2) @binding(2) var scene_sampler: sampler;
@group(2) @binding(3) var<uniform> surface_y: f32;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var screen_uv = mesh.position.xy / view.viewport.zw;

    let surface_clip = view.clip_from_world * vec4<f32>(0.0, surface_y, 0.0, 1.0);
    let surface_ndc_y = surface_clip.y / surface_clip.w;
    let surface_uv_y = 0.5 - surface_ndc_y * 0.5;

    var reflection_uv = vec2<f32>(screen_uv.x, 2.0 * surface_uv_y - screen_uv.y);

    reflection_uv.x += sin(mesh.uv.y * 20.0) * 0.01;

    let reflection_color = textureSample(scene_texture, scene_sampler, reflection_uv);
    let final_color = mix(reflection_color, material_color, 0.4);
    return vec4<f32>(final_color.rgb, 0.5);
}