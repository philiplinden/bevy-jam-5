//! The game's main screen states and transitions between them.

mod credits;
mod loading;
mod playing;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.init_resource::<Screen>();
    app.enable_state_scoped_entities::<Screen>();
    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        playing::plugin,
        credits::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Resource, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    Splash,
    Loading,
    Title,
    Credits,
    #[default]
    Playing,
}
