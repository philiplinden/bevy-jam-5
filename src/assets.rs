use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(AssetCollection, Resource)]
pub struct SoundtrackAssets {
    #[asset(path = "audio/soundtracks/DOS-88_Race-to-Mars.ogg")]
    pub title: Handle<AudioSource>,

    #[asset(path = "audio/soundtracks/DOS-88_Checking-Manifest.ogg")]
    pub game: Handle<AudioSource>,

    #[asset(path = "audio/soundtracks/DOS-88_Double-Tap.ogg")]
    pub credits: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/Knobs/1/knob1-1.png")]
    pub knob1_1: Handle<AudioSource>,

    #[asset(path = "images/Knobs/1/knob1-2.png")]
    pub knob1_2: Handle<AudioSource>,

    #[asset(path = "images/Knobs/1/knob1-3.png")]
    pub knob1_3: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
}

#[derive(AssetCollection, Resource, Default)]
pub struct ShaderAssets {
    #[asset(path = "shaders/oscilloscope.wgsl")]
    pub oscilloscope: Handle<Shader>,

    #[asset(path = "shaders/video-glitch.wgsl")]
    pub glitch: Handle<Shader>,

    #[asset(path = "shaders/crt.wgsl")]
    pub crt: Handle<Shader>,
}
