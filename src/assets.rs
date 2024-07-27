use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
    }
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
pub struct FontAssets {
    #[asset(path = "fonts/divinity-sans-regular.ttf")]
    pub sans: Handle<Font>,

    #[asset(path = "fonts/monogram-extended.ttf")]
    pub mono: Handle<Font>,
}

#[derive(AssetCollection, Resource, Default)]
pub struct ShaderAssets {
    #[asset(path = "shaders/crt.wgsl")]
    pub crt: Handle<Shader>,
}
