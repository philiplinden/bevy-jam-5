//! Spawn a satellite in orbit.

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use avian2d::{math, prelude::*};
use rand::{self, distributions::{Uniform, Distribution}};

use crate::{
    game::settings::*,
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
    let satellite_size = 0.001;
    let satellite_density = 1.0;

    let mut rng = rand::thread_rng();
    let altitude_range = Uniform::new(10.0, 100.0);
    let raan_range = Uniform::new(0.0, 2.0 * math::PI);

    let radius = EARTH_RADIUS + altitude_range.sample(&mut rng);
    let raan = raan_range.sample(&mut rng);
    let (sin, cos) = raan.sin_cos();
    let initial_position = Transform::from_xyz(radius * sin, radius * cos, 0.0);

    commands.spawn((
        Name::new("Satellite"),
        Satellite,
        ShapeBundle::circle(
            &ShapeConfig {
                transform: initial_position,
                color: palette::SATELLITE,
                hollow: false,
                ..ShapeConfig::default_2d()
            },
            satellite_size,
        ),
        MassPropertiesBundle::new_computed(&Collider::circle(satellite_size), satellite_density),
        RigidBody::Dynamic,
        StateScoped(Screen::Playing),
    ));
}
