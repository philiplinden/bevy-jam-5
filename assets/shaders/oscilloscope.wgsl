#define IS_2D
#import bevy_render::{
    view::View,
    globals::Globals,
}
#import bevy_core_pipeline::tonemapping::tone_mapping
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
//
//
@group(0) @binding(0) var<uniform> view: View;
#ifdef IS_2D
@group(0) @binding(1) var<uniform> globals: Globals; // Works on 2d.
#else
@group(0) @binding(9) var<uniform> globals: Globals; // Works on 3d.
#endif

struct OscilloscopeMaterial {
    foreground: vec4<f32>,
    background: vec4<f32>,
};

@group(2) @binding(0) var<uniform> material: OscilloscopeMaterial;
@group(2) @binding(2) var<storage> points: array<vec2<f32>>;
@group(2) @binding(3) var<storage> lines: array<vec2<u32>>;

fn sd_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba)/dot(ba, ba), 0., 1.);
    return length(pa - ba * h);
}

fn sdf(p: vec2<f32>) -> f32 {

    var d = 16777216.0;
    let n = arrayLength(&lines);
    for (var i = 0u; i < n; i++) {
        var a = points[lines[i].x];
        for (var j = lines[i].x + 1; j < lines[i].y; j++) {
            let b = points[j];
            d = min(d, sd_segment(p, a, b));
            a = b;
        }
    }
    return d;
}


fn effect(d: f32, pp: vec2<f32>, resolution: vec2<f32>) -> vec3<f32> {
    let aa: f32 = 2.0 / resolution.y;
    let fg = material.foreground.rgb;
    let bg = material.background.rgb;
    var col: vec3<f32> = 0.1 * bg.rgb;
    col += fg.rgb / sqrt(abs(d));
    col += bg.rgb * smoothstep(aa, -aa, (d - 0.001));

    col *= smoothstep(1.5, 0.5, length(pp));

    return col;
}

// License: Unknown, author: Matt Taylor (https://github.com/64), found: https://64.github.io/tonemapping/
fn aces_approx(v_: vec3<f32>) -> vec3<f32> {
    var v = max(v_, vec3<f32>(0.0));
    v *= 0.6;
    let a: f32 = 2.51;
    let b: f32 = 0.03;
    let c: f32 = 2.43;
    let d: f32 = 0.59;
    let e: f32 = 0.14;
    return saturate((v * (a * v + b)) / (v * (c * v + d) + e));
}

fn to_linear(nonlinear: vec4<f32>) -> vec4<f32> {
    let cutoff = step(nonlinear, vec4<f32>(0.04045));
    let higher = pow((nonlinear + vec4<f32>(0.055)) / vec4<f32>(1.055), vec4<f32>(2.4));
    let lower = nonlinear / vec4<f32>(12.92);
    return mix(higher, lower, cutoff);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let q: vec2<f32> = in.uv;
    var p: vec2<f32> = -1.0 + 2.0 * q;
    let resolution: vec2<f32> = view.viewport.zw;
    let pp: vec2<f32> = p;
    // p.x = p.x * resolution.x / resolution.y;
    let d = sdf(p);
    var col: vec3<f32> = effect(d, pp, resolution);
    col = aces_approx(col);
    col = sqrt(col);
    let c = to_linear(vec4<f32>(col.rgb, 1.));
#ifdef TONEMAP_IN_SHADER
    return tone_mapping(c, view.color_grading);
#else
    return c;
#endif
}
