//! Game mechanics and content.

use bevy::prelude::*;

pub mod audio;
pub mod camera;
pub mod oscilloscope;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        camera::plugin,
        oscilloscope::plugin,
    ));
}
