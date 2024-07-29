use bevy::prelude::*;

use super::ToggleDisplayModeEvent;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, handle_inputs);
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


// #[derive(Component)]
// pub struct WaveformControls {
//     pub phase_axis: _,
//     pub frequency_axis: _,
// }

// fn init_waveform_controls(mut commands: Commands) {
//     commands.spawn()
// }

fn handle_inputs(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        commands.trigger(ToggleDisplayModeEvent);
    }
}
