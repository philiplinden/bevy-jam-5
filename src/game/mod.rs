//! Game mechanics and content.

use bevy::prelude::*;

pub mod audio;
pub mod camera;
pub mod oscilloscope;

use crate::ui::interaction;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        camera::plugin,
        audio::plugin,
        oscilloscope::plugin,
    ));
}

fn spawn_xy_waves(mut commands: Commands) {

}
