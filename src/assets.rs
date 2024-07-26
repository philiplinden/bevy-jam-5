use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use log::info;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LoadingStatus>();
        app.init_resource::<LoadingStatus>();
        app.add_plugins(
            ProgressPlugin::new(LoadingStatus::Working).continue_to(LoadingStatus::Done),
        );
        app.add_loading_state(
            LoadingState::new(LoadingStatus::Working)
                .load_collection::<FontAssets>()
                .load_collection::<SoundtrackAssets>()
                .load_collection::<ShaderAssets>()
                .init_resource::<ShaderAssets>(),
        );
        app.add_systems(
            Update, print_progress.run_if(in_state(LoadingStatus::Working))
        );
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States, Resource)]
enum LoadingStatus {
    #[default]
    Working,
    Done,
}

fn print_progress(
    progress: Option<Res<ProgressCounter>>,
    mut last_done: Local<u32>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
            info!("Progress: {:?}", progress);
        }
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
