use bevy::prelude::*;

use super::display::OscilloscopeMaterial;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_wave_form);
}

#[derive(Component)]
pub struct WaveForm {
    pub amp: f32,
    pub freq: f32,
    pub phase: f32,
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
    pub fn iter(&self, mut t: f32, dt: f32) -> impl Iterator<Item = f32> + '_{
        std::iter::from_fn(move || {
            t += dt;
            Some(self.amp * (self.freq * t + self.phase).sin())
        })
    }
}

pub fn update_wave_form(mut materials: ResMut<Assets<OscilloscopeMaterial>>, time: Res<Time>) {
    for (_id, material) in materials.iter_mut() {
        let x = WaveForm {
            amp: 0.25,
            phase: time.elapsed_seconds(),
            ..default()
        };
        let y = WaveForm {
            freq: 1.5,
            ..x
        };
        let data: Vec<Vec2> = x.iter(0.0, 0.1)
                               .zip(y.iter(0.0, 0.1))
                               .take(100).map(|(x, y)| Vec2::new(x, y)).collect();
        material.channels = data;
    }
}
