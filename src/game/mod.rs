//! Game mechanics and content.

use bevy::prelude::*;

pub mod audio;
pub mod camera;
pub mod oscilloscope;
pub mod physics;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        camera::plugin,
        oscilloscope::plugin,
        physics::plugin,
    ));
}
