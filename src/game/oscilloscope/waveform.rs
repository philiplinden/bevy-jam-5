use std::fmt;

use bevy::prelude::*;

use super::{display::OscilloscopeMaterial, controls::WaveformControls};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Waveform>();
    // app.add_systems(Update, update_wave_form);
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

impl fmt::Display for Waveform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Waveform {{ amp: {}, freq: {}, phase: {} }}", self.amp, self.freq, self.phase)
    }
}

impl Waveform {
    /// An iterator that propagates the wave forward one time step every iteration
    ///
    /// Inputs:
    ///     t   the initial time
    ///     dt  the duration of one time step
    pub fn iter(&self, mut t: f32, dt: f32) -> impl Iterator<Item = f32> + '_{
        std::iter::from_fn(move || {
            t += dt;
            Some(self.amp * (self.freq * t + self.phase).sin())
        })
    }

    /// A timeseries at time step dt for the first N samples.
    pub fn timeseries(&self, samples: usize, dt: f32) -> Vec<f32> {
        self.iter(0.0, dt).take(samples).collect()
    }
}


/// Update the oscilloscope material to match the waveform's shape.
pub fn update_wave_form(
    // mut x_ctrl: Query<(&mut XAxis, &WaveformControls)>,
    // mut y_ctrl: Query<(&mut XAxis, &WaveformControls)>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
    // let (x_axis, x_input) = x_ctrl.single_mut();
    // let (y_axis, y_input) = y_ctrl.single_mut();
    for (_id, material) in materials.iter_mut() {
        let x = Waveform::default();
        let y = Waveform::default();
        let data: Vec<Vec2> = x.iter(0.0, 0.1)
                               .zip(y.iter(0.0, 0.1))
                               .take(100).map(|(x, y)| Vec2::new(x, y)).collect();
        material.channels = data;
    }
}

/// Compute (x, y) display coordinates of a sine wave over time.
pub fn compute_coordinates() {
    // no op
}
