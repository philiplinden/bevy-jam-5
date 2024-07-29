//! This is the base module for rendering the oscilloscope display.

pub mod waveform;
mod material;
pub mod render;

use avian2d::prelude::PhysicsSet;
use bevy::{prelude::*,
           sprite::MaterialMesh2dBundle,
           render::{
               view::RenderLayers,

               render_resource::{
                   Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
               },
           }
};
use bevy_video_glitch::VideoGlitchSettings;

use crate::ui::{Screen, palette::{OSCILLOSCOPE_SCREEN_COLOR, WAVEFORM_COLOR}};
use crate::game::audio::piano::DspBuffer;
use crate::game::crt::CrtSettings;
use material::OscilloscopeMaterial;
use waveform::Waveform;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        material::plugin,
        render::plugin,
        waveform::plugin,
    ));
    app.init_state::<DisplayMode>();
    app.observe(new_oscilloscope);
    app.observe(toggle_display_mode);
    app.add_systems(Startup, setup_camera);
    // app.add_systems(OnEnter(Screen::Playing), new_oscilloscope);
    app.add_systems(Update, (
        render_xy_oscilloscope.run_if(in_state(DisplayMode::XY)),
        render_time_series_oscilloscope.run_if(in_state(DisplayMode::TimeSeries)),
    ).in_set(PhysicsSet::StepSimulation));
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

    let x = Waveform::default();
    let y = Waveform::default();
    let data: Vec<Vec2> = x
        .iter(0.0, 0.1)
        .zip(y.iter(0.0, 0.1))
        .take(1000)
        .map(|(x, y)| Vec2::new(x, y))
        .collect();

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes
            .add(Rectangle::new(
                window.resolution.width(),
                window.resolution.height(),
            ))
            .into(),
        transform: Transform::default(),
        material: materials.add(OscilloscopeMaterial {
            foreground: WAVEFORM_COLOR,
            background: OSCILLOSCOPE_SCREEN_COLOR,
            lines: vec![UVec2::new(0, data.len().saturating_sub(1) as u32)],
            points: data,
        }),
        ..default()
    },
    RenderLayers::layer(1),
                    StateScoped(Screen::Playing),
    ));

}

#[derive(Resource)]
pub struct OscilloscopeImage(pub Handle<Image>);

fn setup_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {

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

/// Select the waveform plotting mode.
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Hash, Eq)]
pub enum DisplayMode {
    XY,
    #[default]
    TimeSeries,
}

#[derive(Event)]
pub struct ToggleDisplayModeEvent;

fn toggle_display_mode(
    _trigger: Trigger<ToggleDisplayModeEvent>,
    mode: Res<State<DisplayMode>>,
    mut next_mode: ResMut<NextState<DisplayMode>>,
) {
    match mode.get() {
        DisplayMode::XY => next_mode.set(DisplayMode::TimeSeries),
        DisplayMode::TimeSeries => next_mode.set(DisplayMode::XY),
    }
}
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
pub fn render_xy_oscilloscope(
    mut dsp_buffers: Query<&DspBuffer>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
    for mut dsp_buffer in &dsp_buffers {
        for (_id, material) in materials.iter_mut() {
            let mut lock = dsp_buffer.0.try_lock();
            if let Ok(ref mut mutex) = lock {
                let mut i = mutex.iter().map(|x| Vec2::new(*x, *x));
                if let Some(x) = i.next() {
                    material.points.clear();
                    material.points.push(x);
                    material.points.extend(i);
                    material.lines = vec![UVec2::new(
                        0,
                        material.points.len().saturating_sub(1) as u32,
                    )];
                } else {
                    continue;
                }
            }
        }
    }
}

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
pub fn render_time_series_oscilloscope(
    mut dsp_buffers: Query<&DspBuffer>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
    for mut dsp_buffer in &dsp_buffers {
        for (_id, material) in materials.iter_mut() {
            let mut lock = dsp_buffer.0.try_lock();
            if let Ok(ref mut mutex) = lock {
                let l = mutex.len();
                let dt = 2.0 / l as f32;
                let mut i = mutex
                    .iter()
                    .enumerate()
                    .map(|(n, x)| Vec2::new(-1. + n as f32 * dt, *x));
                if let Some(x) = i.next() {
                    material.points.clear();
                    material.points.push(x);
                    material.points.extend(i);
                    material.lines = vec![UVec2::new(
                        0,
                        material.points.len().saturating_sub(1) as u32,
                    )];
                } else {
                    continue;
                }
            }
        }
    }
}
