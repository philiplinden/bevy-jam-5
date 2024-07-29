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

use crate::ui::palette::*;

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
    #[storage(2, read_only)]
    pub points: Vec<Vec2>,
    #[storage(3, read_only)]
    pub lines: Vec<UVec2>,

    // #[texture(3)]
    // #[sampler(4)]
    // color_texture: Option<Handle<Image>>,
}

impl Default for OscilloscopeMaterial {
    fn default() -> Self {
        OscilloscopeMaterial {
            foreground: WAVEFORM_COLOR,
            background: OSCILLOSCOPE_SCREEN_COLOR,
            points: vec![],
            lines: vec![],
        }
    }
}

/// The GPU representation of the uniform data of a [`OscilloscopeMaterial`].
#[derive(Clone, Default, ShaderType)]
struct OscilloscopeMaterialUniform {
    pub foreground: LinearRgba,
    pub background: LinearRgba,
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
