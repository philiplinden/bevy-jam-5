pub mod display;
pub mod waveform;

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
};

use display::{OscilloscopeMaterial, Mode};
pub use waveform::Waveform;
use crate::ui::palette::{OSCILLOSCOPE_SCREEN_COLOR, WAVEFORM_COLOR};


pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        display::plugin,
        waveform::plugin,
    ));
    app.observe(new_oscilloscope);
}

#[derive(Event)]
pub struct SpawnOscilloscope;

fn new_oscilloscope(
    _trigger: Trigger<SpawnOscilloscope>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
    window: Query<&Window>,
) {
        let window = window.single();
        let x = Waveform {
            amp: 0.25,
            ..default()
        };
        let y = Waveform {
            freq: 1.5,
            ..x
        };
        let data = x.iter(0.0, 0.1)
            .zip(y.iter(0.0, 0.1))
                    .take(1000).map(|(x, y)| Vec2::new(x, y)).collect();
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
                mode: Mode::XY,
                // mode: Mode::TimeSeries,
                // color_texture: Some(asset_server.load("branding/icon.png")),
            }),
            ..default()
        });
}
