//! Game mechanics and content.

use bevy::prelude::*;

pub mod audio;
pub mod camera;
pub mod physics;
pub mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        audio::plugin,
        spawn::plugin,
        physics::plugin,
    ));
}
