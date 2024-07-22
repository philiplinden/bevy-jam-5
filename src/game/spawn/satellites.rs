//! Spawn a satellite in orbit.

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use avian2d::prelude::*;

use crate::{
    screen::Screen,
    ui::palette,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_satellite);
    app.register_type::<Satellite>();
}

#[derive(Event, Debug)]
pub struct SpawnSatellite;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Satellite;

fn spawn_satellite(
    _trigger: Trigger<SpawnSatellite>,
    mut commands: Commands,
) {
    let radius = 10.0;
    commands.spawn((
        Name::new("Satellite"),
        Satellite,
        ShapeBundle::circle(
            &ShapeConfig {
                color: palette::SATELLITE,
                hollow: false,
                ..ShapeConfig::default_2d()
            },
            radius,
        ),
        TransformBundle::from_transform(Transform::from_xyz(0.0, 300.0, 0.0)),
        Collider::circle(radius),
        RigidBody::Dynamic,
        StateScoped(Screen::Playing),
    ));
}
