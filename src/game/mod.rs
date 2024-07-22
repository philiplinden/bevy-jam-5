//! Game mechanics and content.

use bevy::prelude::*;

pub mod assets;
pub mod audio;
pub mod camera;
pub mod physics;
pub mod settings;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        audio::plugin,
        assets::plugin,
        spawn::plugin,
        physics::plugin,
    ));
}
