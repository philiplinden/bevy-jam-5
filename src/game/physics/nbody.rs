// N-body accelerations

use avian2d::prelude::*;
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
    // Add Particular n-body plugin
    app.add_systems(
        PhysicsSchedule,
        accelerate_particles.in_set(PhysicsStepSet::First),
    );
}

#[derive(Particle, Default)]
#[dim(2)]
pub struct AvianParticle {
    position: Vec2,
    mu: f32,
}

impl AvianParticle {
    fn new(position: Position, mass: Mass) -> AvianParticle {
        AvianParticle {
            position: Vec2::from(position.to_array()),
            mu: mass.0 * G,
        }
    }
}

fn accelerate_particles(
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &mut Transform, &Mass)>,
) {
    query
        .iter()
        .map(|(.., transform, mass)| (transform.translation.to_array(), mass.0))
        .accelerations(&mut COMPUTE_METHOD.clone())
        .map(Vec3::from)
        .zip(&mut query)
        .for_each(|(acceleration, (mut velocity, ..))| (
            velocity.0 += Vec2 {x: acceleration.x, y: acceleration.y} * time.delta_seconds()
        ));
}
