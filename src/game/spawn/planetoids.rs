//! Spawn a planetoid.

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use avian2d::prelude::*;
use avian2d::math::PI;

use crate::{
    game::settings::*,
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
    commands.spawn((
        Name::new("Earth"),
        Earth,
        ShapeBundle::circle(
            &ShapeConfig {
                color: palette::EARTH,
                hollow: true,
                ..ShapeConfig::default_2d()
            },
            EARTH_RADIUS,
        ),
        RigidBody::Static,
        StateScoped(Screen::Playing),
    ));
}

// since we are in 2d, the density uses AREA not VOLUME
pub fn earth_density() -> f32 {
    EARTH_MASS / (PI * ( EARTH_RADIUS.powi(2) ))
}
