//! Spawn the main level by triggering other observers.

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::game::settings::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Earth"),
        ShapeBundle::circle(
            &ShapeConfig {
                color: Color::Srgba(bevy::color::palettes::basic::GREEN),
                hollow: true,
                ..ShapeConfig::default_2d()
            },
            EARTH_RADIUS,
        ),
        RigidBody::Static,
        Collider::circle(EARTH_RADIUS),
    ));
}
