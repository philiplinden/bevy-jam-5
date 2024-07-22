//! Spawn a planetoid.

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use avian2d::prelude::*;

use crate::{
    game::{
        physics::nbody::{PhysicsBody, PhysicsBodyBundle},
        settings::*,
    },
    screen::Screen,
    ui::palette,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_earth);
    app.register_type::<Earth>();
}

#[derive(Event, Debug)]
pub struct SpawnEarth;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Earth;

fn spawn_earth(
    _trigger: Trigger<SpawnEarth>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Earth"),
        Earth,
        ShapeBundle::circle(
            &ShapeConfig {
                color: palette::EARTH,
                hollow: true,
                ..ShapeConfig::default_2d()
            },
            EARTH_RADIUS,
        ),
        PhysicsBodyBundle {
            body: PhysicsBody::new(Vec2::ZERO, EARTH_MASS),
            rigidbody: RigidBody::Dynamic,
            collider: Collider::circle(EARTH_RADIUS),
        },
        StateScoped(Screen::Playing),
    ));
}
