use bevy::prelude::*;

use super::{display::OscilloscopeMaterial, WaveformControls, XAxis};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Waveform>();
    app.add_systems(Update, update_wave_form);
}

#[derive(Component, Reflect, Clone, Copy, Debug)]
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



pub fn update_wave_form(
    mut x_ctrl: Query<(&mut XAxis, &WaveformControls)>,
    mut y_ctrl: Query<(&mut XAxis, &WaveformControls)>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
    let (x_axis, x_input) = x_ctrl.single_mut();
    let (y_axis, y_input) = y_ctrl.single_mut();

    for (_id, material) in materials.iter_mut() {
        let x = x_axis.0;
        let y = y_axis.0;
        let data: Vec<Vec2> = x.iter(0.0, 0.1)
                               .zip(y.iter(0.0, 0.1))
                               .take(100).map(|(x, y)| Vec2::new(x, y)).collect();
        material.channels = data;
    }
}
