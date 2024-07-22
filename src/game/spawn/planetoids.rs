//! Spawn a planetoid.

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use avian2d::prelude::*;

use crate::{
    physics::nbody::PointMass,
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
    let radius = 100.0;
    commands.spawn((
        Name::new("Earth"),
        Earth,
        ShapeBundle::circle(
            &ShapeConfig {
                color: palette::EARTH,
                hollow: true,
                ..ShapeConfig::default_2d()
            },
            radius,
        ),
        Collider::circle(radius),
        RigidBody::Static,
        StateScoped(Screen::Playing),
    ));
}
