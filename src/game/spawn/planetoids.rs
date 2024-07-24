//! Spawn the main level by triggering other observers.

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::game::settings::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_planet);
}

#[derive(Event, Debug)]
pub struct SpawnPlanet;

fn spawn_planet(
    _trigger: Trigger<SpawnPlanet>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Planet"),
        ShapeBundle::circle(
            &ShapeConfig {
                color: Color::from(crate::ui::palette::EARTH),
                hollow: true,
                ..ShapeConfig::default_2d()
            },
            PLANET_RADIUS,
        ),
        RigidBody::Static,
        Collider::circle(PLANET_RADIUS),
        Mass(PLANET_MASS),
    ));
}
