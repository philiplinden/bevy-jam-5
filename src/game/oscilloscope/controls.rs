use bevy::prelude::*;

use super::ToggleDisplayModeEvent;
use crate::game::audio::piano::{SetPitchEvent, Pitch};

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
    for keycode in input.get_just_pressed() {
        match keycode {
            KeyCode::Space => commands.trigger(ToggleDisplayModeEvent),
            KeyCode::KeyA  => commands.trigger(SetPitchEvent(Pitch::C)),
            KeyCode::KeyW  => commands.trigger(SetPitchEvent(Pitch::Cs)),
            KeyCode::KeyS  => commands.trigger(SetPitchEvent(Pitch::D)),
            KeyCode::KeyE  => commands.trigger(SetPitchEvent(Pitch::Ds)),
            KeyCode::KeyD  => commands.trigger(SetPitchEvent(Pitch::E)),
            KeyCode::KeyF  => commands.trigger(SetPitchEvent(Pitch::F)),
            KeyCode::KeyT  => commands.trigger(SetPitchEvent(Pitch::Fs)),
            KeyCode::KeyG  => commands.trigger(SetPitchEvent(Pitch::G)),
            KeyCode::KeyY  => commands.trigger(SetPitchEvent(Pitch::Gs)),
            KeyCode::KeyH  => commands.trigger(SetPitchEvent(Pitch::A)),
            KeyCode::KeyU  => commands.trigger(SetPitchEvent(Pitch::As)),
            KeyCode::KeyJ  => commands.trigger(SetPitchEvent(Pitch::B)),
            _ => {},
        };
    };
}
