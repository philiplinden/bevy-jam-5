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
    pub position: Position,
    pub velocity: LinearVelocity,
    pub size: Vec2,
    pub mass: f32,
    pub color: Color,
}

impl Default for SpawnSatellite {
    fn default() -> Self {
        SpawnSatellite {
            position: Position(Vec2::ZERO),
            velocity: LinearVelocity(Vec2::ZERO),
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
            body: PhysicsBody {
                position: Position(spawn_event.position.0),
                velocity: LinearVelocity(spawn_event.velocity.0),
                mass: Mass(spawn_event.mass),
                ..default()
            },
            collider: Collider::rectangle(spawn_event.size.x, spawn_event.size.y),
            ..default()
        },
        ShapeBundle::rect(
            &ShapeConfig {
                color: spawn_event.color,
                hollow: false,
                ..ShapeConfig::default_2d()
            },
            spawn_event.size,
        ),
        StateScoped(Screen::Playing),
    ));
}

pub fn random_starting_position() -> Position {
    let mut rng = rand::thread_rng();
    let altitude_range = Uniform::new(10.0, 100.0);
    let raan_range = Uniform::new(0.0, 2.0 * math::PI);

    let radius = EARTH_RADIUS + altitude_range.sample(&mut rng);
    let raan = raan_range.sample(&mut rng);
    let (sin, cos) = raan.sin_cos();
    Position::from_xy(radius * sin, radius * cos)
}
