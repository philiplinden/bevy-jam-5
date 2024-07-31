use bevy::prelude::*;
use bevy_fundsp::prelude::*;

pub mod dsp;
pub mod piano;
// pub mod soundtrack;
pub mod tee;
pub mod signal_gen;

pub fn plugin(app: &mut App) {
    app.add_plugins(DspPlugin::default()).add_plugins((
        dsp::plugin,
        // soundtrack::plugin,
        // piano::PianoPlugin,
        signal_gen::SignalGeneratorPlugin
    ));
    app.init_resource::<MasterVolume>();
}

#[derive(Component, Clone, Copy)]
pub struct Channel(pub u8);

#[derive(Resource)]
pub struct MasterVolume(f32);

impl Default for MasterVolume {
    fn default() -> Self {
        MasterVolume(0.2)
    }
}
