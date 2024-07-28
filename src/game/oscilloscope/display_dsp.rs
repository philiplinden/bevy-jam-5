use std::time::Duration;
use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};
use bevy::time::common_conditions::on_timer;
use crate::game::audio::piano::{PianoId, PitchVar, DspBuffer};
use crate::game::oscilloscope::OscilloscopeMaterial;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_oscilloscope.run_if(on_timer(Duration::from_millis(100))));
}

// pub fn update_oscilloscope(
//     _dsp_manager: Res<DspManager>,
//     mut piano_units: Query<&mut PianoUnit, Changed<PitchVar>>,
//     mut materials: ResMut<Assets<OscilloscopeMaterial>>,
// ) {

//     for mut piano_unit in &mut piano_units {
//         for (_id, material) in materials.iter_mut() {

//             let input : [f32; 0] = [];
//             let mut output : [f32; 2] = [0., 0.];

//             piano_unit.set_sample_rate(4410.0); // instead of 44,100 Hz

//             material.channels.clear();
//             for i in 0..1000 {
//                 piano_unit.tick(&input, &mut output);
//                 material.channels.push(Vec2::new(output[0], output[1]));
//             }
//         }
//     }
// }

pub fn update_oscilloscope(
    mut dsp_buffers: Query<&DspBuffer>,
    mut materials: ResMut<Assets<OscilloscopeMaterial>>,
) {
    for mut dsp_buffer in &dsp_buffers {
        for (_id, material) in materials.iter_mut() {

            let mut lock = dsp_buffer.0.try_lock();
            if let Ok(ref mut mutex) = lock {
                let mut i = mutex.iter().map(|x| Vec2::new(*x, *x));
                if let Some(x) = i.next() {
                    material.channels.clear();
                    material.channels.push(x);
                    material.channels.extend(i);
                } else {
                    continue;
                }
            }
        }
    }
}
