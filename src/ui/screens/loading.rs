//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use log::info;

use super::Screen;
use crate::{assets::*, ui::prelude::*};

#[cfg(not(feature = "autoplay"))]
const POST_LOADING_SCREEN: Screen = Screen::Title;
#[cfg(feature = "autoplay")]
const POST_LOADING_SCREEN: Screen = Screen::Playing;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), spawn_screen);
    app.add_plugins(
        ProgressPlugin::new(Screen::Loading).continue_to(POST_LOADING_SCREEN),
    );
    app.add_loading_state(
        LoadingState::new(LoadingStatus::Working)
            .load_collection::<FontAssets>()
            .load_collection::<SoundtrackAssets>()
            .load_collection::<ShaderAssets>()
            .init_resource::<ShaderAssets>(),
    );
    app.add_systems(
        Update, (
            load_images.track_progress(),
            load_shaders.track_progress(),
            load_audio.track_progress(),
            print_progress
        ).run_if(in_state(Screen::Loading)).after(LoadingStateSet(LoadingStatus::Working))
    );
}

fn spawn_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
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
