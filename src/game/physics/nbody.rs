// N-body accelerations

use avian2d::{math::FRAC_PI_2, prelude::*};
use bevy::prelude::*;
use particular::prelude::*;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

use crate::game::settings::*;

pub const G: f32 = NEWTONIAN_CONSTANT_OF_GRAVITATION as f32;

#[cfg(target_arch = "wasm32")]
const COMPUTE_METHOD: sequential::BarnesHut<f32> = sequential::BarnesHut {
    theta: BARNES_HUT_THETA,
};
#[cfg(not(target_arch = "wasm32"))]
const COMPUTE_METHOD: parallel::BarnesHut<f32> = parallel::BarnesHut {
    theta: BARNES_HUT_THETA,
};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gravity::ZERO);
    app.add_plugins(big_space::BigSpacePlugin::<i64>::new(true));
    // Add Particular n-body plugin
    app.add_systems(
        PhysicsSchedule,
        accelerate_particles.in_set(PhysicsStepSet::First),
    );
}

fn accelerate_particles(
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &mut Transform, &Mass)>,
) {
    query
        .iter()
        .map(|(.., transform, mass)| (transform.translation.to_array(), mass.0 * G))
        .accelerations(&mut COMPUTE_METHOD.clone())
        .map(Vec3::from)
        .zip(&mut query)
        .for_each(|(acceleration, (mut velocity, ..))| (
            velocity.0 += Vec2 {x: acceleration.x, y: acceleration.y} * time.delta_seconds()
        ));
}

pub struct OrbitingBody {
    pub position: Vec2,
    pub mass: f32,
}

/// Relative velocity of Body 2 such that it has a circular orbit around Body 1
pub fn circular_velocity(body1: OrbitingBody, body2: OrbitingBody) -> Vec2 {
    let magnitude = f32::sqrt( ( G * body2.mass ) / ( body2.position.distance(body1.position)));
    let direction = (body2.position - body1.position).normalize().perp();
    direction * magnitude
}
