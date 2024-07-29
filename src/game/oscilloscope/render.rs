//! Render signals as waves on the display.

use bevy::prelude::*;

use crate::game::audio::dsp::DspBuffer;
use super::OscilloscopeMaterial;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<DisplayMode>();
    app.observe(toggle_display_mode);
    app.observe(set_display_mode);
    app.add_event::<ToggleDisplayModeEvent>();
    app.add_event::<SetDisplayModeEvent>();
    app.add_systems(
        PostUpdate,
        (
            render_xy_oscilloscope.run_if(in_state(DisplayMode::XY)),
            render_time_series_oscilloscope.run_if(in_state(DisplayMode::TimeSeries)),
        )
    );
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
pub fn render_xy_oscilloscope(
    dsp_buffers: Query<&DspBuffer>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
    for dsp_buffer in &dsp_buffers {
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
    dsp_buffers: Query<&DspBuffer>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
    for dsp_buffer in &dsp_buffers {
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
