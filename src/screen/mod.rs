//! The game's main screen states and transitions between them.

mod credits;
mod loading;
mod playing;
mod splash;
mod title;
mod paused;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        credits::plugin,
        playing::plugin,
        paused::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum Screen {
    Splash,
    Loading,
    Title,
    Credits,
    Playing,
    Paused,
}

impl Default for Screen {
    fn default() -> Self {
        #[cfg(skip_menu)]
        {
            Screen::Title
        }

        #[cfg(not(skip_menu))]
        {
            Screen::Playing
        }
    }
}
