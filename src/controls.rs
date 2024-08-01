use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
}
//         let mut frequency = waveform.frequency.value();
//         let mut phase = waveform.phase.value();
//         match channel {
//             AudioChannel::Left => {
//                 // Phase control for left channel
//                 if keyboard_input.pressed(KeyCode::KeyA) {
//                     phase -= 0.1 * delta;
//                 }
//                 if keyboard_input.pressed(KeyCode::KeyD) {
//                     phase += 0.1 * delta;
//                 }
//                 // Frequency control for left channel
//                 if keyboard_input.pressed(KeyCode::KeyW) {
//                     frequency *= 1.01;
//                 }
//                 if keyboard_input.pressed(KeyCode::KeyS) {
//                     frequency *= 0.99;
//                 }
//             }
//             AudioChannel::Right => {
//                 // Phase control for right channel
//                 if keyboard_input.pressed(KeyCode::ArrowLeft) {
//                     phase -= 0.1 * delta;
//                 }
//                 if keyboard_input.pressed(KeyCode::ArrowRight) {
//                     phase += 0.1 * delta;
//                 }
//                 // Frequency control for right channel
//                 if keyboard_input.pressed(KeyCode::ArrowUp) {
//                     frequency *= 1.01;
//                 }
//                 if keyboard_input.pressed(KeyCode::ArrowDown) {
//                     frequency *= 0.99;
//                 }
//             }
//             AudioChannel::Both => {
//                 // For signals on both channels, we'll update based on either set of controls
//                 // Phase control
//                 if keyboard_input.pressed(KeyCode::KeyA)
//                     || keyboard_input.pressed(KeyCode::ArrowLeft)
//                 {
//                     phase -= 0.1 * delta;
//                 }
//                 if keyboard_input.pressed(KeyCode::KeyD)
//                     || keyboard_input.pressed(KeyCode::ArrowRight)
//                 {
//                     phase += 0.1 * delta;
//                 }
//                 // Frequency control
//                 if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp)
//                 {
//                     frequency *= 1.01;
//                 }
//                 if keyboard_input.pressed(KeyCode::KeyS)
//                     || keyboard_input.pressed(KeyCode::ArrowDown)
//                 {
//                     frequency *= 0.99;
//                 }
//             }


// fn handle_inputs(
//     mut commands: Commands,
//     input: Res<ButtonInput<KeyCode>>,
// ) {
//     if input.pressed(KeyCode::Space) {
//         // push to talk!
//     }
//     for keycode in input.get_just_pressed() {
//         // #[cfg(not(feature = "piano_mode"))]
//         match keycode {
//             KeyCode::Space => commands.trigger(oscilloscope::ToggleDisplayModeEvent),
//         //     // KeyCode::Tab => commands.trigger(ToggleMusicMixEvent),
//         //     KeyCode::KeyW => commands.trigger(ModulateChannelEvent {
//         //         channel: left,
//         //         parameter: WaveParam::Frequency,
//         //         delta: delta.frequency_hz,
//         //     }),
//         //     KeyCode::KeyS => commands.trigger(ModulateChannelEvent {
//         //         channel: left,
//         //         parameter: WaveParam::Frequency,
//         //         delta: delta.frequency_hz,
//         //     }),
//         //     KeyCode::KeyD => commands.trigger(ModulateChannelEvent {
//         //         channel: left,
//         //         parameter: WaveParam::Phase,
//         //         delta: delta.phase_radians,
//         //     }),
//         //     KeyCode::KeyA => commands.trigger(ModulateChannelEvent {
//         //         channel: left,
//         //         parameter: WaveParam::Phase,
//         //         delta: delta.phase_radians,
//         //     }),
//         //     KeyCode::ArrowUp => commands.trigger(ModulateChannelEvent {
//         //         channel: right,
//         //         parameter: WaveParam::Frequency,
//         //         delta: delta.frequency_hz,
//         //     }),
//         //     KeyCode::ArrowDown => commands.trigger(ModulateChannelEvent {
//         //         channel: right,
//         //         parameter: WaveParam::Frequency,
//         //         delta: delta.frequency_hz,
//         //     }),
//         //     KeyCode::ArrowRight => commands.trigger(ModulateChannelEvent {
//         //         channel: right,
//         //         parameter: WaveParam::Phase,
//         //         delta: delta.phase_radians,
//         //     }),
//         //     KeyCode::ArrowLeft => commands.trigger(ModulateChannelEvent {
//         //         channel: right,
//         //         parameter: WaveParam::Phase,
//         //         delta: delta.phase_radians,
//         //     }),

//         //     KeyCode::Digit1 => commands.trigger(SetWaveShapeEvent(WaveShape::Sine)),
//         //     KeyCode::Digit2 => commands.trigger(SetWaveShapeEvent(WaveShape::Square)),
//         //     KeyCode::Digit3 => commands.trigger(SetWaveShapeEvent(WaveShape::Triangle)),
//         //     KeyCode::Digit4 => commands.trigger(SetWaveShapeEvent(WaveShape::Sawtooth)),
//             _ => {}
//         }
//     }
// }
