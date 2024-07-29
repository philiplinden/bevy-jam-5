use bevy::prelude::*;
use bevy_fundsp::prelude::*;

pub mod dsp;
pub mod piano;
// pub mod soundtrack;
pub mod synth;
pub mod tee;

pub fn plugin(app: &mut App) {
    app.add_plugins(DspPlugin::default()).add_plugins((
        dsp::plugin,
        synth::plugin,
        // soundtrack::plugin,
        #[cfg(feature = "piano_mode")]
        piano::PianoPlugin,
    ));
}

#[derive(Component, Clone, Copy)]
pub struct Channel(pub u8);

#[derive(Component)]
pub struct LeftAudioChannel(pub Shared);

#[derive(Component)]
pub struct RightAudioChannel(pub Shared);
