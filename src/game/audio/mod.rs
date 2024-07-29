use bevy::prelude::*;
use bevy_fundsp::prelude::*;

pub mod dsp;
pub mod piano;
// pub mod soundtrack;
pub mod tee;

pub fn plugin(app: &mut App) {
    app.add_plugins(DspPlugin::default()).add_plugins((
        dsp::plugin,
        // soundtrack::plugin,
        piano::PianoPlugin,
    ));
}

#[derive(Component, Clone, Copy)]
pub struct Channel(pub u8);
