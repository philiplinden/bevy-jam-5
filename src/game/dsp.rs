use std::time::Duration;
use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};
use bevy::time::common_conditions::on_timer;
use crate::game::audio::piano::{PianoId, PitchVar, DspBuffer};
use crate::game::oscilloscope::material::OscilloscopeMaterial;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_oscilloscope.run_if(on_timer(Duration::from_millis(100))));
}

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
                    material.points.clear();
                    material.points.push(x);
                    material.points.extend(i);
                    material.lines = vec![UVec2::new(0, material.points.len().saturating_sub(1) as u32)];
                } else {
                    continue;
                }
            }
        }
    }
}
