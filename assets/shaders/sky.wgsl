#import bevy_pbr::forward_io::VertexOutput

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let direction = normalize(in.world_position);
    let height = direction.y;
    let t = smoothstep(0.0, 1.0, height);

    // TODO pass in colors
    let color = mix(
        vec3(0, 0, 255),
        vec3(0, 220, 255),
        t
    );

    return vec4(color, 1.0);
}