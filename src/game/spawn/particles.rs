//! Spawn the nbody particles.

use avian2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::game::{
    physics::nbody::{PhysicsBodyBundle, PhysicsBody}, settings::*,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_particle);
}

#[derive(Event, Debug)]
pub struct SpawnParticle;

fn spawn_particle(
    _trigger: Trigger<SpawnParticle>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: EARTH_RADIUS * 0.10 })),
            material: materials.add(Color::Srgba(bevy::color::palettes::basic::WHITE)),
            ..default()
        },
        PhysicsBodyBundle {
            body: PhysicsBody::new(Vec2::new(EARTH_RADIUS * 1.1, 0.0), EARTH_MASS * 0.10),
            rigidbody: RigidBody::Kinematic,
            collider: Collider::circle(5.0)
        },
        Position(Vec2::new(EARTH_RADIUS + 500.0, 0.0)),
        LinearVelocity(Vec2::new(0.0, 10.0)),
    ));
}
