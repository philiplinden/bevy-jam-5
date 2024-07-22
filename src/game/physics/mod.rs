//! Physics simulation.

use bevy::prelude::*;
use avian2d::prelude::*;

pub mod propulsion;
pub mod nbody;

use super::settings::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default().with_length_unit(PIXELS_PER_METER),
        nbody::plugin,
        propulsion::plugin,
    ));
    app.insert_resource(Time::new_with(Physics::fixed_hz(FIXED_TIMESTEP_HZ)));
}
