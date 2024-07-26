//! A shader and a material that uses it.
use bevy::{
    core_pipeline::{
        core_2d::graph::{Core2d, Node2d},
        core_3d::graph::{Core3d, Node3d},
        fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    },
    ecs::{system::SystemState, query::QueryItem},
    prelude::*,
    reflect::TypePath,
    render::{
        extract_component::{
            ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
        },
        globals::{GlobalsBuffer, GlobalsUniform},
        render_asset::RenderAssets,
        render_graph::{
            NodeRunError, RenderGraphApp, RenderGraphContext, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            AsBindGroup, AsBindGroupShaderType, BindGroupEntries, BindGroupLayout,
            BindGroupLayoutEntries, CachedRenderPipelineId, ColorTargetState, ColorWrites,
            FragmentState, MultisampleState, Operations, PipelineCache, PrimitiveState,
            RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, Sampler,
            SamplerBindingType, SamplerDescriptor, ShaderRef, ShaderStages, ShaderType,
            TextureFormat, TextureSampleType,
        },
        renderer::{RenderContext, RenderDevice},
        texture::BevyDefault,
        texture::GpuImage,
        view::ViewTarget,
        RenderApp,
    },
    sprite::{Material2d, Material2dPlugin},
};

use crate::assets::ShaderAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        Material2dPlugin::<OscilloscopeMaterial>::default(),
        // CrtPlugin,
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

#[derive(Debug, Default, Clone, Copy)]
pub enum Mode {
    #[default]
    XY = 1,
    #[allow(dead_code)]
    TimeSeries = 2,
}

