//! A shader and a material that uses it.
use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        render_asset::RenderAssets,
        render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType},
        texture::GpuImage,
    },
    sprite::{Material2d, Material2dPlugin},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        Material2dPlugin::<OscilloscopeMaterial>::default(),
    ));
}

// FIXME: This should be loaded in `assets.rs` as a resource
const SHADER_ASSET_PATH: &str = "shaders/oscilloscope.wgsl";

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// #[reflect(Default, Debug)]
#[uniform(0, OscilloscopeMaterialUniform)]
pub struct OscilloscopeMaterial {
    pub foreground: Color,
    pub background: Color,
    pub offset: Vec2,
    pub begin: UVec2,
    pub mode: Mode,
    #[storage(2, read_only)]
    pub channels: Vec<Vec2>,
    // #[texture(3)]
    // #[sampler(4)]
    // color_texture: Option<Handle<Image>>,
}

/// The GPU representation of the uniform data of a [`OscilloscopeMaterial`].
#[derive(Clone, Default, ShaderType)]
pub struct OscilloscopeMaterialUniform {
    pub foreground: LinearRgba,
    pub background: LinearRgba,
    pub offset: Vec2,
    pub begin: UVec2,
    pub mode: u32,
}

impl AsBindGroupShaderType<OscilloscopeMaterialUniform> for OscilloscopeMaterial {
    fn as_bind_group_shader_type(
        &self,
        _images: &RenderAssets<GpuImage>,
    ) -> OscilloscopeMaterialUniform {
        // let mut flags = OscilloscopeMaterialFlags::NONE;
        // if self.texture.is_some() {
        //     flags |= OscilloscopeMaterialFlags::TEXTURE;
        // }

        OscilloscopeMaterialUniform {
            foreground: LinearRgba::from(self.foreground),
            background: LinearRgba::from(self.background),
            begin: self.begin,
            offset: self.offset,
            mode: self.mode as u32,
        }
    }
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for OscilloscopeMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

/// Select the waveform plotting mode.
///
/// `Mode::Timeseries`: plots all waves on amplitude over time axes.
/// ```
///      +1 |   /\      /\      /\      /\      /\      /\
///         |  /  \    /  \    /  \    /  \    /  \    /  \
///      +0 | /    \  /    \  /    \  /    \  /    \  /    \
///         |/      \/      \/      \/      \/      \/      \
///      -1 |
///         +-------------------------------------------------->
///           0      1      2      3      4      5      6    Time
/// ```
///
/// `Mode::XY`: Lissajous Pattern (Wave 1 Amplitude vs. Wave 2 Amplitude)
/// ```
///      +1 |    *   *
///         |  *       *
///         | *         *
///      +0 |*           *
///         | *         *
///         |  *       *
///      -1 |    *   *
///         +-------------------->
///         -1    0    +1
/// ```
///
/// Note: The actual pattern may vary depending on the frequency and phase
/// relationship between the two sine waves.
#[derive(Debug, Default, Clone, Copy)]
pub enum Mode {
    #[default]
    XY = 1,
    #[allow(dead_code)]
    TimeSeries = 2,
}
