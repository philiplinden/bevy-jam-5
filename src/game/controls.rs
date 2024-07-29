use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use super::oscilloscope::ToggleDisplayModeEvent;

pub const X_PHASE_AXIS: VirtualAxis = VirtualAxis::horizontal_arrow_keys();
pub const X_FREQUENCY_AXIS: VirtualAxis = VirtualAxis::vertical_arrow_keys();
pub const Y_PHASE_AXIS: VirtualAxis = VirtualAxis::ad();
pub const Y_FREQUENCY_AXIS: VirtualAxis = VirtualAxis::ws();

#[derive(Component)]
pub struct WaveformControls {
    pub phase_axis: VirtualAxis,
    pub frequency_axis: VirtualAxis,
}
