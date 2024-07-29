#define IS_2D

#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::globals::Globals


struct CrtSettings {
    scanline_intensity: f32, // 0.01
    bend_radius: f32, // 3.2
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<u32>
#endif
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: CrtSettings;
@group(0) @binding(3) var<uniform> globals: Globals; // Works on 3d.

// @group(0) @binding(2) var<uniform> globals.time: f32;
// @group(0) @binding(3) var<uniform> iResolution: vec2<f32>;

fn scanline(coord: vec2<f32>, screen: vec3<f32>) -> vec3<f32> {
    var result = screen;
    result -= sin((coord.y + (globals.time * 29.0))) * settings.scanline_intensity;
    return result;
}

fn crt(coord: vec2<f32>, bend: f32) -> vec2<f32> {
    var result = (coord - 0.5) * 2.0;
    result *= 1.1;
    result *= 1.0 + pow((abs(result.yx) / bend), vec2(2.4, 2.4));
    result = (result / 2.0) + 0.5;
    return result;
}

fn sample_split(coord: vec2<f32>) -> vec3<f32> {
    let s1 = textureSample(screen_texture, texture_sampler, vec2<f32>(coord.x - 0.01 * sin(globals.time), coord.y));
    let s2 = textureSample(screen_texture, texture_sampler, vec2<f32>(coord.x, coord.y));
    let s3 = textureSample(screen_texture, texture_sampler, vec2<f32>(coord.x + 0.01 * sin(globals.time), coord.y));
    return vec3<f32>(s1.r, s2.g, s3.b);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    uv.y = 1.0 - uv.y; // flip tex
    let crt_coords = crt(uv, settings.bend_radius);
    let resolution = textureDimensions(screen_texture);

    // WGSL doesn't have a discard keyword, so we'll return a transparent color instead
    if (crt_coords.x < 0.0 || crt_coords.x > 1.0 || crt_coords.y < 0.0 || crt_coords.y > 1.0) {
        discard;
        // return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    var fragColor = vec4<f32>(sample_split(crt_coords), 1.0);

    let screen_space = crt_coords * bitcast<vec2<f32>>(resolution);
    fragColor = vec4<f32>(scanline(screen_space, fragColor.rgb), 1.0);

    return fragColor;
}
