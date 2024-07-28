use std::time::Duration;
use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};
use bevy::time::common_conditions::on_timer;
use crate::game::audio::piano::{PianoId, PitchVar, DspBuffer};
use crate::game::oscilloscope::material::OscilloscopeMaterial;

pub fn plugin(app: &mut App) {
    app
        .init_state::<DisplayMode>()
        .add_systems(Update, render_xy_oscilloscope.run_if(in_state(DisplayMode::XY).and_then(on_timer(Duration::from_millis(100)))))
        .add_systems(Update, render_time_series_oscilloscope.run_if(in_state(DisplayMode::TimeSeries).and_then(on_timer(Duration::from_millis(100)))))
        ;
}

/// Select the waveform plotting mode.
///
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
///
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
///
/// Note: The actual pattern may vary depending on the frequency and phase
/// relationship between the two sine waves.
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Hash, Eq)]
pub enum DisplayMode {
    XY,
    #[default]
    TimeSeries,
}

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
                    material.lines = vec![UVec2::new(0, material.points.len().saturating_sub(1) as u32)];
                } else {
                    continue;
                }
            }
        }
    }
}

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
                let mut i = mutex.iter().enumerate().map(|(n, x)| Vec2::new(-1. + n as f32 * dt, *x));
                if let Some(x) = i.next() {
                    material.points.clear();
                    material.points.push(x);
                    material.points.extend(i);
                    material.lines = vec![UVec2::new(0, material.points.len().saturating_sub(1) as u32)];
                } else {
                    continue;
                }
            }
        }
    }
}
