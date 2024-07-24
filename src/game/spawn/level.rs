//! Spawn the main level by triggering other observers.

use avian2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::game::{
    physics::nbody::{PhysicsBodyBundle, PhysicsBody}, settings::*,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_level);
}

#[derive(Event, Debug)]
pub struct SpawnLevel;

fn spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: EARTH_RADIUS })),
            material: materials.add(Color::Srgba(bevy::color::palettes::basic::GREEN)),
            ..default()
        },
        PhysicsBodyBundle {
            body: PhysicsBody::new(Vec2::new(0.0, 0.0), EARTH_MASS),
            rigidbody: RigidBody::Kinematic,
            collider: Collider::circle(EARTH_RADIUS)
        }
    ));
}
