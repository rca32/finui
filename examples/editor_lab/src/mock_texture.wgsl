struct FrameUniform {
    time_seconds: f32,
    width: f32,
    height: f32,
    generation: f32,
};

@group(0) @binding(0)
var<uniform> frame: FrameUniform;

@vertex
fn vertex_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    let positions = array(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0, -1.0),
        vec2<f32>(-1.0, 3.0),
    );
    return vec4<f32>(positions[vertex_index], 0.0, 1.0);
}

@fragment
fn fragment_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    let dimensions = vec2<f32>(frame.width, frame.height);
    let uv = position.xy / dimensions;
    let aspect_uv = vec2<f32>(uv.x * frame.width / frame.height, uv.y);
    let time = frame.time_seconds;

    let grid_x = smoothstep(0.92, 1.0, cos(uv.x * 80.0));
    let grid_y = smoothstep(0.92, 1.0, cos(uv.y * 45.0));
    let grid = max(grid_x, grid_y) * 0.12;

    let center = vec2<f32>(
        frame.width / frame.height * (0.5 + 0.24 * sin(time * 0.8)),
        0.5 + 0.18 * cos(time * 1.1),
    );
    let pulse = 0.10 + 0.02 * sin(time * 2.4 + frame.generation);
    let glow = 1.0 - smoothstep(pulse, pulse + 0.18, distance(aspect_uv, center));

    let wave = 0.5 + 0.5 * sin(uv.x * 14.0 - time * 2.0 + uv.y * 6.0);
    let base = mix(vec3<f32>(0.025, 0.055, 0.11), vec3<f32>(0.04, 0.19, 0.24), uv.y);
    let accent = vec3<f32>(0.12, 0.78, 0.91) * glow;
    let color = base + accent + grid + vec3<f32>(0.02, 0.04, 0.08) * wave;
    return vec4<f32>(color, 1.0);
}
