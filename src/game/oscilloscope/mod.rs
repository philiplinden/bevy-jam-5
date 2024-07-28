pub mod display;
pub mod waveform;
pub mod display_dsp;
pub mod controls;

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use display::{OscilloscopeMaterial, DisplayMode};
use waveform::Waveform;
use controls::*;
use crate::ui::palette::{OSCILLOSCOPE_SCREEN_COLOR, WAVEFORM_COLOR};


pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        display::plugin,
        waveform::plugin,
        display_dsp::plugin,
    ));
    app.observe(new_oscilloscope);
}

/// We want to spawn the oscilloscope display before any waves, but we enforce that elsewhere. This is set up as an
/// Observer because not every game screen needs the display. We let the game screen systems decide when to spawn the
/// display, and can keep the display scoped to that screen if we want.
#[derive(Event)]
pub struct SpawnOscilloscope;

/// Set up the oscilloscope display mesh and apply the appropriate shaders. The o-scope is just a rectangle with a fancy
/// material slapped on it.
///
/// Note that we initialize it without any waveforms. Instead, we will use a system to query for all waveforms and then
/// add then insert them into the data struct as a list of (x,y) coordinates to plot (technically just a vector of
/// 2d-vectors). The display doesn't retain info about the waveforms themselves, it only sees the list of (x,y) pairs.
/// Internally, the DisplayMode enum sets whether to plot x and y as two timeseries waves or as 2D spatial coordinates.
fn new_oscilloscope(
    _trigger: Trigger<SpawnOscilloscope>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();

    let x = Waveform::default();
    let y = Waveform::default();
    let data = x.iter(0.0, 0.1)
        .zip(y.iter(0.0, 0.1))
                .take(1000).map(|(x, y)| Vec2::new(x, y)).collect();

    // commands.spawn( (
    //     XAxis(x),
    //     WaveformControls {
    //         phase_axis: X_PHASE_AXIS,
    //         frequency_axis: X_FREQUENCY_AXIS,
    //     },
    // ));
    // commands.spawn((
    //     YAxis(y),
    //     WaveformControls {
    //         phase_axis: Y_PHASE_AXIS,
    //         frequency_axis: Y_FREQUENCY_AXIS,
    //     }
    // ));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(window.resolution.width(), window.resolution.height())).into(),
        transform: Transform::default(),
        material: materials.add(OscilloscopeMaterial {
            foreground: WAVEFORM_COLOR,
            background: OSCILLOSCOPE_SCREEN_COLOR,
            offset: Vec2::new(0.35, -0.35),
            begin: UVec2::new(0, 0),
            // channels: vec![Vec2::splat(0.0), Vec2::splat(1.)],
            channels: data,
            mode: DisplayMode::XY,
            // mode: DisplayMode::TimeSeries,
            // color_texture: Some(asset_server.load("branding/icon.png")),
        }),
        ..default()
    });
}
