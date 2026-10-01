#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct LofiEffect {
    pixel_size: f32,
    color_levels: f32,
    gamma: vec2<f32>,
}

@group(0) @binding(2) var<uniform> settings: LofiEffect;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Pixelation
    let texture_size = vec2<f32>(textureDimensions(screen_texture));
    let pixel_count = texture_size / max(2.0 * settings.pixel_size, 1.0);
    let pixelated_uv = floor(in.uv * pixel_count) / pixel_count;

    // Color reduction
    var color = textureSample(screen_texture, texture_sampler, pixelated_uv);
    color = vec4<f32>(pow(color.rgb, vec3(settings.gamma.x)), color.a);

    let grayscale = max(color.r, max(color.g, color.b));

    let lower = floor(grayscale * settings.color_levels) / settings.color_levels;
    let higher = ceil(grayscale * settings.color_levels) / settings.color_levels;

    let lower_diff = abs(lower - grayscale);
    let higher_diff = abs(higher - grayscale);

    let level = select(higher, lower, lower_diff < higher_diff);
    let adjustment = level / grayscale;
    
    color = vec4<f32>(color.rgb * adjustment, color.a);
    color = vec4<f32>(pow(color.rgb, vec3(settings.gamma.y)), color.a);

    // Return adjusted color
    return color;
}