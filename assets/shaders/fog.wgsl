#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_render::globals::Globals
#import custom::noise::fbm

@group(0) @binding(1) var<uniform> globals: Globals;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    var screen_uv = mesh.position.xy / view.viewport.zw;

    let noise_pos = vec2<f32>(mesh.uv.x * 2.0, mesh.uv.y * 12.0) + vec2<f32>(globals.time * 0.02, 0.0);
    let n = fbm(noise_pos, 4);

    let fog_mask = smoothstep(0.45, 0.55, (n + 1.0) * 0.5);

    let dist_from_center = abs(mesh.uv.y - 0.5);
    let spread = 0.1;
    var band_fade = exp(-(dist_from_center * dist_from_center) / (2.0 * spread * spread));
    band_fade *= 1.0 - smoothstep(0.45, 0.5, dist_from_center);

    let fog = fog_mask * band_fade;

    return vec4<f32>(n, n, n, fog * 0.2);
}