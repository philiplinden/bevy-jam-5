use bevy::prelude::*;

use super::ToggleDisplayModeEvent;

pub fn plugin(app: &mut App) {
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum InputAction {
    ToggleDisplayMode,
    ChangeXPhase,
    ChangeXFrequency,
    ChangeYPhase,
    ChangeYFrequency,
    Pause,
}


#[derive(Component)]
pub struct WaveformControls {
    pub phase_axis: _,
    pub frequency_axis: _,
}

fn init_waveform_controls(mut commands: Commands) {
    commands.spawn()
}
