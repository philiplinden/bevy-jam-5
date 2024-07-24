//! Spawn the nbody particles.

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::game::{physics::nbody::{circular_velocity, OrbitingBody}, settings::*};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_satellite);
}

#[derive(Event, Debug)]
pub struct SpawnSatellite;

fn spawn_satellite(
    _trigger: Trigger<SpawnSatellite>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Satellite"),
        ShapeBundle::circle(
            &ShapeConfig {
                color: Color::from(crate::ui::palette::SATELLITE),
                hollow: false,
                transform: Transform::from_xyz(378000.0, 0.0, 0.0),
                ..ShapeConfig::default_2d()
            },
            PARTICLE_RADIUS,
        ),
        RigidBody::Dynamic,
        Collider::circle(PARTICLE_RADIUS),
        LinearVelocity(circular_velocity(
            OrbitingBody { position: Vec2 {x: 378000.0, y: 0.0}, mass: PARTICLE_MASS },
            OrbitingBody { position: Vec2 {x: 0.0, y: 0.0}, mass: PLANET_MASS },
        )),
        Mass(PARTICLE_MASS),
    ));
}
