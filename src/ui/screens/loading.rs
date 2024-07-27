//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on WASM.

use bevy::prelude::*;

use super::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Loading), enter_screen);
    app.add_systems(
        Update,
        continue_to_next_screen.run_if(in_state(Screen::Loading).and_then(all_assets_loaded)),
    );
}

fn enter_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Loading))
        .with_children(|children| {
            children.label("Loading...");
        });
}

fn all_assets_loaded(
    asset_server: Res<AssetServer>,
) -> bool {
    true
}

fn continue_to_next_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Playing);
}
