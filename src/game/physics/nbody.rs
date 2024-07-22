// N-body accelerations

use bevy::prelude::*;
use avian2d::prelude::*;
use particular::prelude::*;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

pub const G: f32 = NEWTONIAN_CONSTANT_OF_GRAVITATION as f32;

#[cfg(target_arch = "wasm32")]
const COMPUTE_METHOD: sequential::BruteForceSIMD<4> = sequential::BruteForceSIMD;
#[cfg(not(target_arch = "wasm32"))]
const COMPUTE_METHOD: parallel::BruteForceSIMD<8> = parallel::BruteForceSIMD;

pub(super) fn plugin(app: &mut App) {
    // Add Particular n-body plugin
    app.add_systems(PreUpdate, accelerate_rigidbodies);
    app.insert_resource(Gravity::ZERO);
}

/// Batched n-body accelerations from Particular example
fn accelerate_rigidbodies(mut query: Query<(&mut LinearVelocity, &GlobalTransform, &Mass)>, time: Res<Time>) {
    query
        .iter()
        .map(|(.., transform, mass)| {
            (
                transform.translation().truncate().to_array(),
                mass.0 * G,
            )
        })
        .accelerations(&mut COMPUTE_METHOD.clone())
        .zip(&mut query)
        .for_each(|(acceleration, (mut velocity, ..))| {
            let delta_v = Vec2::from(acceleration) * time.delta_seconds();
            velocity.0 += delta_v;
        });
}
