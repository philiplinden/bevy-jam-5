use bevy::prelude::*;

pub mod dsp;
pub mod piano;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        // soundtrack::plugin,
        // piano::PianoPlugin,
        dsp::plugin,
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

#[derive(Component, Clone, Copy, PartialEq, Debug, Default, Reflect)]
pub enum AudioChannel {
    Left,
    Right,
    #[default]
    Both,
}

impl AudioChannel {
    fn pan(&self) -> f32 {
        match self {
            AudioChannel::Left => -1.0,
            AudioChannel::Right => 1.0,
            AudioChannel::Both => 0.0,
        }
    }
}
