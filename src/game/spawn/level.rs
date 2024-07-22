//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use super::planetoids::SpawnEarth;
use super::satellites::{SpawnSatellite, random_starting_position};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    commands.trigger(SpawnEarth);
    commands.trigger(SpawnSatellite {
        position: random_starting_position(),
        ..default()
    });
}
