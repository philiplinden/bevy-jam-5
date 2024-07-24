//! Spawn the nbody particles.

use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

use crate::game::settings::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_particle);
}

#[derive(Event, Debug)]
pub struct SpawnParticle;

fn spawn_particle(
    _trigger: Trigger<SpawnParticle>,
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Particle"),
        ShapeBundle::circle(
            &ShapeConfig {
                color: Color::Srgba(bevy::color::palettes::basic::WHITE),
                hollow: false,
                transform: Transform::from_xyz(EARTH_RADIUS * 1.1, 0.0, 0.0),
                ..ShapeConfig::default_2d()
            },
            PARTICLE_RADIUS,
        ),
        RigidBody::Dynamic,
        Collider::circle(PARTICLE_RADIUS),
        // LinearVelocity(Vec2::new(0.0, 0.0)),
    ));
}
