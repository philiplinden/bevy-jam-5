use bevy::prelude::*;

use crate::ui::interaction::WaveformControlsBundle;

use super::display::OscilloscopeMaterial;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_wave_form);
    app.observe(spawn_xy_waves);
}

#[derive(Component)]
pub struct Waveform {
    pub amp: f32,
    pub freq: f32,
    pub phase: f32,
}

impl Default for Waveform {
    fn default() -> Self {
        Self {
            amp: 1.0,
            freq: 1.0,
            phase: 0.0,
        }
    }
}

impl Waveform {
    pub fn iter(&self, mut t: f32, dt: f32) -> impl Iterator<Item = f32> + '_{
        std::iter::from_fn(move || {
            t += dt;
            Some(self.amp * (self.freq * t + self.phase).sin())
        })
    }
}

#[derive(Component, Default)]
pub struct XAxisWave (Waveform);

#[derive(Component, Default)]
pub struct YAxisWave (Waveform);

#[derive(Event, Debug)]
pub struct SpawnXYWaves;

pub fn spawn_xy_waves(
    _trigger: Trigger<SpawnXYWaves>,
    mut commands: Commands,
) {
    commands.spawn( XAxisWave::default() );
    commands.spawn(YAxisWave::default() );
    commands.spawn(WaveformControlsBundle::default());
}

pub fn update_wave_form(mut materials: ResMut<Assets<OscilloscopeMaterial>>, time: Res<Time>) {
    for (_id, material) in materials.iter_mut() {
        let x = Waveform {
            amp: 0.25,
            phase: time.elapsed_seconds(),
            ..default()
        };
        let y = Waveform {
            freq: 1.5,
            ..x
        };
        let data: Vec<Vec2> = x.iter(0.0, 0.1)
                               .zip(y.iter(0.0, 0.1))
                               .take(100).map(|(x, y)| Vec2::new(x, y)).collect();
        material.channels = data;
    }
}
