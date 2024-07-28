use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};
use crate::game::audio::piano::{PianoId, PianoUnit, PitchVar, VolVar};
use crate::game::oscilloscope::OscilloscopeMaterial;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_oscilloscope);
}

/// Update the oscilloscope material to match the waveform's shape.
// pub fn update_oscilloscope(
//     dsp_manager: Res<DspManager>,
//     // piano_ids: Query<&PianoId, Changed<PitchVar>>,
//     piano_ids: Query<&PianoId>,
//     mut materials: ResMut<Assets<OscilloscopeMaterial>>,
// ) {

//     for piano_id in &piano_ids {
//     for (_id, material) in materials.iter_mut() {
//         let mut audio_unit = dsp_manager
//             .get_graph_by_id(&piano_id.0)
//             .expect("graph");

//         material.channels.clear();
//         material.channels.extend(audio_unit.into_iter().take(1000).map(|[a, b]| Vec2::new(a,b)));
//     }
//     }
// }
pub fn update_oscilloscope(
    dsp_manager: Res<DspManager>,
    mut piano_units: Query<&mut PianoUnit, Or<(Changed<VolVar>, Changed<PitchVar>)>>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
    for mut piano_unit in &mut piano_units {
        // eprintln!("."); // to ensure it only updates when changed.
        for (_id, material) in materials.iter_mut() {

            let input : [f32; 0] = [];
            let mut output : [f32; 2] = [0., 0.];

            piano_unit.set_sample_rate(4410.0); // instead of 44,100 Hz

            material.channels.clear();
            for i in 0..1000 {
                piano_unit.tick(&input, &mut output);
                material.channels.push(Vec2::new(output[0], output[1]));
            }
        }
    }
}
