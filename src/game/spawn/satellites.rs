//! Spawn a satellite in orbit.

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

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

    commands.spawn((
        Name::new("Satellite"),
        Satellite,
        ShapeBundle::circle(
            &ShapeConfig {
                color: palette::SATELLITE,
                hollow: false,
                ..ShapeConfig::default_2d()
            },
            10.0,
        ),
        StateScoped(Screen::Playing),
    ));
}
