use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};
use crate::game::audio::piano::{PianoId, PitchVar};
use crate::game::oscilloscope::OscilloscopeMaterial;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_oscilloscope);
}

/// Update the oscilloscope material to match the waveform's shape.
pub fn update_oscilloscope(
    dsp_manager: Res<DspManager>,
    // piano_ids: Query<&PianoId, Changed<PitchVar>>,
    piano_ids: Query<&PianoId>,
    // mut x_ctrl: Query<(&mut XAxis, &WaveformControls)>,
    // mut y_ctrl: Query<(&mut XAxis, &WaveformControls)>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {

    // let (x_axis, x_input) = x_ctrl.single_mut();
    // let (y_axis, y_input) = y_ctrl.single_mut();
    for piano_id in &piano_ids {
    for (_id, material) in materials.iter_mut() {
        // let x = Waveform::default();
        // let y = Waveform::default();
        // let data: Vec<Vec2> = x.iter(0.0, 0.1)
        //                        .zip(y.iter(0.0, 0.1))
        //                        .take(100).map(|(x, y)| Vec2::new(x, y)).collect();
        // material.channels = data;
        let mut audio_unit = dsp_manager
            .get_graph_by_id(&piano_id.0)
            .expect("graph");

        material.channels.clear();
        material.channels.extend(audio_unit.into_iter().take(1000).map(|[a, b]| Vec2::new(a,b)));
    }
    }
}
