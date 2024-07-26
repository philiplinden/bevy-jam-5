struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

@group(0) @binding(2) var<uniform> iTime: f32;
@group(0) @binding(3) var<uniform> iResolution: vec2<f32>;

fn scanline(coord: vec2<f32>, screen: vec3<f32>) -> vec3<f32> {
    var result = screen;
    result -= sin((coord.y + (iTime * 29.0))) * 0.02;
    return result;
}

fn crt(coord: vec2<f32>, bend: f32) -> vec2<f32> {
    var result = (coord - 0.5) * 2.0;
    result *= 1.1;
    result.x *= 1.0 + pow((abs(result.y) / bend), 2.0);
    result.y *= 1.0 + pow((abs(result.x) / bend), 2.0);
    result = (result / 2.0) + 0.5;
    return result;
}

fn sampleSplit(coord: vec2<f32>) -> vec3<f32> {
    var frag: vec3<f32>;
    frag.r = textureSample(texture, texture_sampler, vec2<f32>(coord.x - 0.01 * sin(iTime), coord.y)).r;
    frag.g = textureSample(texture, texture_sampler, vec2<f32>(coord.x, coord.y)).g;
    frag.b = textureSample(texture, texture_sampler, vec2<f32>(coord.x + 0.01 * sin(iTime), coord.y)).b;
    return frag;
}

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    uv.y = 1.0 - uv.y; // flip tex
    let crtCoords = crt(uv, 3.2);

    // WGSL doesn't have a discard keyword, so we'll return a transparent color instead
    if (crtCoords.x < 0.0 || crtCoords.x > 1.0 || crtCoords.y < 0.0 || crtCoords.y > 1.0) {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }

    var fragColor: vec4<f32>;
    fragColor.rgb = sampleSplit(crtCoords);

    let screenSpace = crtCoords * iResolution;
    fragColor.rgb = scanline(screenSpace, fragColor.rgb);
    fragColor.a = 1.0;

    return fragColor;
}
