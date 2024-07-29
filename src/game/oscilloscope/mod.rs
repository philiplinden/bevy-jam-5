//! This is the base module for rendering the oscilloscope display.

pub mod controls;
mod crt;
mod material;
pub mod render;

use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};
use bevy_video_glitch::VideoGlitchSettings;

use crate::ui::Screen;
use crt::{CrtPlugin, CrtSettings};
use material::OscilloscopeMaterial;
pub use render::{DisplayMode, SetDisplayModeEvent, ToggleDisplayModeEvent};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        CrtPlugin,
        material::plugin,
        render::plugin,
        controls::plugin,
    ));
    app.observe(new_oscilloscope);
    app.add_systems(Startup, setup_camera);
}

#[derive(Bundle)]
pub struct OscilloscopeBundle {}

/// We want to spawn the oscilloscope display on command. This is set up as an Observer because not every game screen
/// needs the display. We let the game screen systems decide when to spawn the display, and can keep the display scoped
/// to that screen if we want. This emulates turning the screen on and off.
#[derive(Event)]
pub struct SpawnOscilloscope;

/// Set up the oscilloscope display mesh and apply the appropriate shaders.
///
/// The display doesn't retain info about the waveforms themselves, it only sees a vector of (x, y) coordinates.
/// Internally, the DisplayMode enum sets whether to plot x and y as two timeseries waves or as 2D spatial coordinates.
pub fn new_oscilloscope(
    _trigger: Trigger<SpawnOscilloscope>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
    window: Query<&Window>,
) {
    // We query the window to get its current width and height. This is used to scale the display.
    let window = window.single();

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes
            .add(Rectangle::new(
                1218.0,
                917.0,

                // window.resolution.width(),
                // window.resolution.height(),
            ))
            .into(),
        transform: Transform::default(),
        material: materials.add(OscilloscopeMaterial::default()),
        ..default()
    },
    RenderLayers::layer(1),
    StateScoped(Screen::Playing),
    ));
}

#[derive(Resource)]
pub struct OscilloscopeImage(pub Handle<Image>);

pub fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 1218,
        height: 975,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    // let mut camera_bundle = Camera2dBundle::default();
    // camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed {
    //     width: 512.,
    //     height: 512.,
    // };
    commands.spawn((
        Name::new("Oscilloscope Camera"),
        Camera2dBundle {
            camera: Camera {
                order: -1,
                clear_color: Color::BLACK.into(),
                target: image_handle.clone().into(),
                ..default()
            },
            ..default()
        },
        // camera_bundle,
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        // This component is also used to determine on which camera to run the post processing effect.
        VideoGlitchSettings {
            intensity: 0.1,
            color_aberration: Mat3::IDENTITY,
        },
        CrtSettings::default(),
        RenderLayers::layer(1),
    ));
    commands.insert_resource(OscilloscopeImage(image_handle));
}
