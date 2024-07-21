//! Physics simulation.

use bevy::prelude::*;
use avian2d::prelude::*;

pub mod movement;
pub mod nbody;
pub mod time;
pub mod settings;

use settings::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default().with_length_unit(PIXELS_PER_METER),
        time::plugin,
        nbody::plugin,
        movement::plugin,
    ));
}
