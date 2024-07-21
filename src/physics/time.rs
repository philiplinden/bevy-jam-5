use bevy::prelude::*;
use avian2d::prelude::*;

use super::settings::*;

pub(super) fn plugin(app: &mut App) {
    // Add Particular n-body plugin
    app.insert_resource(Time::new_with(Physics::fixed_hz(FIXED_TIMESTEP_HZ)));
}
