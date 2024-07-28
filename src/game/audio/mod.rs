use bevy::prelude::*;
use bevy_fundsp::prelude::*;

// pub mod sfx;
// pub mod soundtrack;
pub mod piano;

pub fn plugin(app: &mut App) {
    app
        .add_plugins(DspPlugin::default())
        .add_plugins(piano::PianoPlugin);
}
