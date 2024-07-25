//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
    render_asset::RenderAssets,
        render_resource::{AsBindGroup, ShaderRef, ShaderType, AsBindGroupShaderType},
    texture::{GpuImage, Image},
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/oscilloscope.wgsl";

#[derive(Event)]
pub struct SpawnOscilloscope;

pub struct WaveForm {
    amp: f32,
    freq: f32,
    phase: f32,
}

impl Default for WaveForm {
    fn default() -> Self {
        Self {
            amp: 1.0,
            freq: 1.0,
            phase: 0.0,
        }
    }
}

impl WaveForm {
    fn iter(&self, mut t: f32, dt: f32) -> impl Iterator<Item = f32> + '_{
        std::iter::from_fn(move || {
            t += dt;
            Some(self.amp * (self.freq * t + self.phase).sin())
        })
    }
}

pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<OscilloscopeMaterial>::default())
        .observe(|_trigger: Trigger<SpawnOscilloscope>,
                 mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<OscilloscopeMaterial>>,
                 | {
                     let x = WaveForm {
                         amp: 0.25,
                         ..default()
                     };
                     let y = WaveForm {
                         freq: 1.5,
                         ..x
                     };
                     let data = x.iter(0.0, 0.1)
                         .zip(y.iter(0.0, 0.1))
                                 .take(1000).map(|(x, y)| Vec2::new(x, y)).collect();
                     commands.spawn(MaterialMesh2dBundle {
                         mesh: meshes.add(Rectangle::default()).into(),
                         transform: Transform::default().with_scale(Vec3::splat(512.)),
                         material: materials.add(OscilloscopeMaterial {
                             // foreground: LinearRgba::GREEN,
                             foreground: Color::hsl(118.882, 0.535, 0.109).into(),
                             // background: LinearRgba::BLUE,
                             background: Color::hsl(192.671, 0.800, 0.658).into(),
                             offset: Vec2::new(0.35, -0.35),
                             // channels: vec![Vec2::splat(0.0), Vec2::splat(1.)],
                             channels: data,
                             // mode: Mode::XY,
                             mode: Mode::TimeSeries,
                             // color_texture: Some(asset_server.load("branding/icon.png")),
                         }),
                         ..default()
                     });
                 });
}

#[derive(Debug, Default, Clone, Copy)]
pub enum Mode {
    #[default]
    XY = 1,
    TimeSeries = 2,
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
// #[reflect(Default, Debug)]
#[uniform(0, OscilloscopeMaterialUniform)]
pub struct OscilloscopeMaterial {
    pub foreground: Color,
    pub background: Color,
    pub offset: Vec2,
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
    pub mode: u32,
}

impl AsBindGroupShaderType<OscilloscopeMaterialUniform> for OscilloscopeMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<GpuImage>) -> OscilloscopeMaterialUniform {
        // let mut flags = OscilloscopeMaterialFlags::NONE;
        // if self.texture.is_some() {
        //     flags |= OscilloscopeMaterialFlags::TEXTURE;
        // }

        OscilloscopeMaterialUniform {
            foreground: LinearRgba::from(self.foreground),
            background: LinearRgba::from(self.background),
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
