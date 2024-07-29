//! This is the base module for rendering the oscilloscope display.

pub mod controls;
pub mod waveform;
mod material;
mod render;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub use render::{DisplayMode, ToggleDisplayModeEvent, SetDisplayModeEvent};
use crate::ui::palette::{OSCILLOSCOPE_SCREEN_COLOR, WAVEFORM_COLOR};
use material::OscilloscopeMaterial;
use waveform::Waveform;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        material::plugin,
        render::plugin,
        waveform::plugin,
        controls::plugin
    ));
    app.observe(new_oscilloscope);
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

    commands.spawn(MaterialMesh2dBundle {
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
    });
}
