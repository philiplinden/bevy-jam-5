// pub mod sfx;
// pub mod soundtrack;
pub mod piano;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app
        .add_plugins(DspPlugin::default())
        .add_plugins(piano::PianoPlugin);
}
