//! Render signals as waves on the display.

use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    sprite::MaterialMesh2dBundle,
};
use bevy_video_glitch::VideoGlitchSettings;

// use crate::audio::dsp::DspBuffer;
use super::material::CrtSettings;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<DisplayMode>();
    app.observe(toggle_display_mode);
    app.observe(set_display_mode);
    app.add_event::<ToggleDisplayModeEvent>();
    app.add_event::<SetDisplayModeEvent>();
    app.add_systems(
        PostUpdate,
        (
            render_xy.run_if(in_state(DisplayMode::XY)),
            render_time_series.run_if(in_state(DisplayMode::TimeSeries)),
        )
    );
}

#[derive(Component)]
pub struct CrtDisplay {
    frontpanel_image: Handle<Image>,
    display_mode: DisplayMode,
}

/// Select the waveform plotting mode.
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Hash, Eq)]
pub enum DisplayMode {
    XY,
    #[default]
    TimeSeries,
}

/// Trigger the display to change modes.
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

#[derive(Event)]
pub struct SetDisplayModeEvent(pub DisplayMode);

fn set_display_mode(
    trigger: Trigger<SetDisplayModeEvent>,
    mut next_mode: ResMut<NextState<DisplayMode>>,
) {
    next_mode.set(trigger.event().0);
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
pub fn render_xy(
    // dsp_buffers: Query<&DspBuffer>,
    // mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
//     for dsp_buffer in &dsp_buffers {
//         for (_id, material) in materials.iter_mut() {
//             let mut lock = dsp_buffer.0.try_lock();
//             if let Ok(ref mut mutex) = lock {
//                 let mut i = mutex.iter().map(|x| Vec2::new(*x, *x));
//                 if let Some(x) = i.next() {
//                     material.points.clear();
//                     material.points.push(x);
//                     material.points.extend(i);
//                     material.lines = vec![UVec2::new(
//                         0,
//                         material.points.len().saturating_sub(1) as u32,
//                     )];
//                 } else {
//                     continue;
//                 }
//             }
//         }
//     }
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
pub fn render_time_series(
    // dsp_buffers: Query<&DspBuffer>,
    // mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
//     for dsp_buffer in &dsp_buffers {
//         for (_id, material) in materials.iter_mut() {
//             let mut lock = dsp_buffer.0.try_lock();
//             if let Ok(ref mut mutex) = lock {
//                 let l = mutex.len();
//                 let dt = 2.0 / l as f32;
//                 let mut i = mutex
//                     .iter()
//                     .enumerate()
//                     .map(|(n, x)| Vec2::new(-1. + n as f32 * dt, *x));
//                 if let Some(x) = i.next() {
//                     material.points.clear();
//                     material.points.push(x);
//                     material.points.extend(i);
//                     material.lines = vec![UVec2::new(
//                         0,
//                         material.points.len().saturating_sub(1) as u32,
//                     )];
//                 } else {
//                     continue;
//                 }
//             }
//         }
//     }
}

#[derive(Resource)]
pub struct CrtImage(pub Handle<Image>);

/// The camera that renders only the CRT
pub fn setup_crt(mut commands: Commands,
    mut materials: ResMut<Assets<super::OscilloscopeMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    osc_image: Res<CrtImage>,) {
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
        Name::new("CRT Node"),
        VideoGlitchSettings {
            intensity: 0.1,
            color_aberration: Mat3::IDENTITY,
        },
        CrtSettings::default(),
        Camera2dBundle {
            camera: Camera {
                order: -1,
                clear_color: Color::BLACK.into(),
                target: image_handle.clone().into(),
                ..default()
            },
            ..default()
        },
        ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(933.),
                top: Val::Px(421.),
                width: Val::Px(1416.),
                height: Val::Px(1142.),
                ..default()
            },
            image: osc_image.0.clone().into(),
            ..default()
        },
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(
                    1218.0,
                    917.0,
                    // window.resolution.width(),
                    // window.resolution.height(),
                ))
                .into(),
            transform: Transform::default(),
            material: materials.add(super::OscilloscopeMaterial::default()),
            ..default()
        },
    ));
    commands.insert_resource(CrtImage(image_handle));
}
