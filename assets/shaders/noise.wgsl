#define_import_path custom::noise

fn permute3(x: vec3<f32>) -> vec3<f32> {
    return ((x * 34.0 + 1.0) * x) % 289.0;
}

fn simplex_noise_2d(v: vec2<f32>) -> f32 {
    let C = vec4<f32>(0.211324865405187, 0.366025403784439, -0.577350269189626, 0.024390243902439);
    var i = floor(v + dot(v, C.yy));
    let x0 = v - i + dot(i, C.xx);

    let i1 = select(vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 0.0), x0.x > x0.y);

    var x12 = x0.xyxy + C.xxzz;
    x12 = vec4<f32>(x12.xy - i1, x12.zw);

    i = i % 289.0;
    let p = permute3(permute3(i.y + vec3<f32>(0.0, i1.y, 1.0)) + i.x + vec3<f32>(0.0, i1.x, 1.0));

    var m = max(0.5 - vec3<f32>(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3<f32>(0.0));
    m = m * m;
    m = m * m;

    let x = 2.0 * fract(p * C.www) - 1.0;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;

    m = m * (1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h));

    let g = vec3<f32>(
        a0.x * x0.x + h.x * x0.y,
        a0.y * x12.x + h.y * x12.y,
        a0.z * x12.z + h.z * x12.w,
    );

    return 130.0 * dot(m, g);
}

fn fbm(pos: vec2<f32>, octaves: i32) -> f32 {
    var value = 0.0;
    var amplitude = 1.0;
    var frequency = 1.0;
    var max_value = 0.0;

    for (var i = 0; i < octaves; i++) {
        value += simplex_noise_2d(pos * frequency) * amplitude;
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    return value / max_value;
}