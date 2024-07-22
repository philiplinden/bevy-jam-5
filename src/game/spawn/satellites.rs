//! Spawn a satellite in orbit.

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use avian2d::{math, prelude::*};
use rand::{self, distributions::{Uniform, Distribution}};

use crate::{
    game::{
        physics::nbody::{PhysicsBody, PhysicsBodyBundle},
        settings::*,
    },
    screen::Screen,
    ui::palette,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_satellite);
    app.register_type::<Satellite>();
}

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct Satellite;

#[derive(Event, Debug)]
pub struct SpawnSatellite {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: Vec2,
    pub mass: f32,
    pub color: Color,
}

impl Default for SpawnSatellite {
    fn default() -> Self {
        SpawnSatellite {
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            size: Vec2::new(0.5, 0.1),
            mass: 5.0,
            color: palette::SATELLITE,
        }
    }
}


fn spawn_satellite(
    trigger: Trigger<SpawnSatellite>,
    mut commands: Commands,
) {
    let spawn_event = trigger.event();
    commands.spawn((
        Name::new("Satellite"),
        PhysicsBodyBundle {
            body: PhysicsBody::new(spawn_event.position, spawn_event.mass),
            collider: Collider::rectangle(spawn_event.size.x, spawn_event.size.y),
            ..default()
        },
        LinearVelocity(spawn_event.velocity),
        ShapeBundle::rect(
            &ShapeConfig {
                transform: Transform::from_xyz(spawn_event.position.x, spawn_event.position.y, 0.0),
                color: spawn_event.color,
                hollow: false,
                ..ShapeConfig::default_2d()
            },
            spawn_event.size,
        ),
        StateScoped(Screen::Playing),
    ));
}

pub fn random_starting_position() -> Vec2 {
    let mut rng = rand::thread_rng();
    let altitude_range = Uniform::new(10.0, 100.0);
    let raan_range = Uniform::new(0.0, 2.0 * math::PI);

    let radius = EARTH_RADIUS + altitude_range.sample(&mut rng);
    let raan = raan_range.sample(&mut rng);
    let (sin, cos) = raan.sin_cos();
    Vec2::new(radius * sin, radius * cos)
}
