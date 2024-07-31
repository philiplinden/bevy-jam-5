use bevy::prelude::*;

use super::ToggleDisplayModeEvent;
#[cfg(feature = "piano_mode")]
use crate::game::audio::piano::{Pitch, SetPitchEvent};

const DEFAULT_INCREMENT: f32 = 0.1;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, handle_inputs);
}

#[derive(Resource)]
struct FrequencyIncrement(f32);

fn handle_inputs(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::Space) {
        // push to talk!
    }
    for keycode in input.get_just_pressed() {
        // #[cfg(not(feature = "piano_mode"))]
        match keycode {
            KeyCode::Space => commands.trigger(ToggleDisplayModeEvent),
        //     // KeyCode::Tab => commands.trigger(ToggleMusicMixEvent),
        //     KeyCode::KeyW => commands.trigger(ModulateChannelEvent {
        //         channel: left,
        //         parameter: WaveParam::Frequency,
        //         delta: delta.frequency_hz,
        //     }),
        //     KeyCode::KeyS => commands.trigger(ModulateChannelEvent {
        //         channel: left,
        //         parameter: WaveParam::Frequency,
        //         delta: delta.frequency_hz,
        //     }),
        //     KeyCode::KeyD => commands.trigger(ModulateChannelEvent {
        //         channel: left,
        //         parameter: WaveParam::Phase,
        //         delta: delta.phase_radians,
        //     }),
        //     KeyCode::KeyA => commands.trigger(ModulateChannelEvent {
        //         channel: left,
        //         parameter: WaveParam::Phase,
        //         delta: delta.phase_radians,
        //     }),
        //     KeyCode::ArrowUp => commands.trigger(ModulateChannelEvent {
        //         channel: right,
        //         parameter: WaveParam::Frequency,
        //         delta: delta.frequency_hz,
        //     }),
        //     KeyCode::ArrowDown => commands.trigger(ModulateChannelEvent {
        //         channel: right,
        //         parameter: WaveParam::Frequency,
        //         delta: delta.frequency_hz,
        //     }),
        //     KeyCode::ArrowRight => commands.trigger(ModulateChannelEvent {
        //         channel: right,
        //         parameter: WaveParam::Phase,
        //         delta: delta.phase_radians,
        //     }),
        //     KeyCode::ArrowLeft => commands.trigger(ModulateChannelEvent {
        //         channel: right,
        //         parameter: WaveParam::Phase,
        //         delta: delta.phase_radians,
        //     }),

        //     KeyCode::Digit1 => commands.trigger(SetWaveShapeEvent(WaveShape::Sine)),
        //     KeyCode::Digit2 => commands.trigger(SetWaveShapeEvent(WaveShape::Square)),
        //     KeyCode::Digit3 => commands.trigger(SetWaveShapeEvent(WaveShape::Triangle)),
        //     KeyCode::Digit4 => commands.trigger(SetWaveShapeEvent(WaveShape::Sawtooth)),
            _ => {}
        }
        #[cfg(feature = "piano_mode")]
        match keycode {
            KeyCode::Space => commands.trigger(ToggleDisplayModeEvent),
            KeyCode::KeyA => commands.trigger(SetPitchEvent(Pitch::C)),
            KeyCode::KeyW => commands.trigger(SetPitchEvent(Pitch::Cs)),
            KeyCode::KeyS => commands.trigger(SetPitchEvent(Pitch::D)),
            KeyCode::KeyE => commands.trigger(SetPitchEvent(Pitch::Ds)),
            KeyCode::KeyD => commands.trigger(SetPitchEvent(Pitch::E)),
            KeyCode::KeyF => commands.trigger(SetPitchEvent(Pitch::F)),
            KeyCode::KeyT => commands.trigger(SetPitchEvent(Pitch::Fs)),
            KeyCode::KeyG => commands.trigger(SetPitchEvent(Pitch::G)),
            KeyCode::KeyY => commands.trigger(SetPitchEvent(Pitch::Gs)),
            KeyCode::KeyH => commands.trigger(SetPitchEvent(Pitch::A)),
            KeyCode::KeyU => commands.trigger(SetPitchEvent(Pitch::As)),
            KeyCode::KeyJ => commands.trigger(SetPitchEvent(Pitch::B)),
            _ => {}
        };
    }
}
