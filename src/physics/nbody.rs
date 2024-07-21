// N-body accelerations

use bevy::prelude::*;
use avian2d::prelude::*;
use particular::prelude::*;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

#[cfg(target_arch = "wasm32")]
const COMPUTE_METHOD: sequential::BruteForceSIMD<4> = sequential::BruteForceSIMD;
#[cfg(not(target_arch = "wasm32"))]
const COMPUTE_METHOD: parallel::BruteForceSIMD<8> = parallel::BruteForceSIMD;

pub(super) fn plugin(app: &mut App) {
    // Add Particular n-body plugin
    app.add_systems(PreUpdate, accelerate_rigidbodies);
    app.insert_resource(Gravity::ZERO);
}

#[derive(Component)]
pub enum PointMass {
    HasGravity { mass: f32 },
    AffectedByGravity,
}

impl PointMass {
    fn mass(&self) -> f32 {
        match *self {
            PointMass::HasGravity { mass } => mass,
            PointMass::AffectedByGravity => 0.0,
        }
    }
}

/// Batched n-body accelerations from Particular example
fn accelerate_rigidbodies(mut query: Query<(&mut LinearVelocity, &GlobalTransform, &PointMass)>, time: Res<Time>) {
    let accelerations = query
        .iter()
        .map(|(.., transform, mass)| {
            (
                transform.translation().truncate().to_array(),
                mass.mass() * NEWTONIAN_CONSTANT_OF_GRAVITATION as f32,
            )
        })
        .accelerations(&mut COMPUTE_METHOD.clone());
    accelerations
        .zip(&mut query)
        .for_each(|(acceleration, (mut velocity, ..))| {
            let delta_v = Vec2::from(acceleration) * time.delta_seconds();
            velocity.0 += delta_v;
        });
}
