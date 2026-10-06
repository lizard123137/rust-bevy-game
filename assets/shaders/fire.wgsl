#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::view
#import bevy_render::globals::Globals
#import custom::noise::fbm

@group(0) @binding(1) var<uniform> globals: Globals;

const RISE_SPEED: f32 = 1.2;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let uv = mesh.uv;

    let fuel = pow(uv.y, 1.3) * 1.5;

    let p = vec2<f32>(uv.x * 6.0, uv.y * 3.0 + globals.time * RISE_SPEED);
    let n = fbm(p, 4);

    let heat = clamp(fuel + (n - 0.5) * 1.1 - 0.35, 0.0, 1.0);

    var col = mix(vec3<f32>(0.30, 0.03, 0.02), vec3<f32>(0.90, 0.25, 0.03), smoothstep(0.0, 0.4, heat));
    col = mix(col, vec3<f32>(1.00, 0.65, 0.10), smoothstep(0.35, 0.75, heat));
    col = mix(col, vec3<f32>(1.00, 0.95, 0.65), smoothstep(0.70, 1.00, heat));

    let alpha = smoothstep(0.0, 0.25, heat);

    return vec4<f32>(col, alpha);
}