use bevy::prelude::*;
use avian2d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
    ));
}
