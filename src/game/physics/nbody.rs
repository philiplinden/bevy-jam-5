// N-body accelerations

use bevy::prelude::*;
use avian2d::prelude::*;
use particular::prelude::*;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

use crate::game::settings::*;

pub const G: f32 = NEWTONIAN_CONSTANT_OF_GRAVITATION as f32;

#[cfg(target_arch = "wasm32")]
const COMPUTE_METHOD: sequential::BruteForceSIMD<f32> = sequential::BarnesHut { theta: BARNES_HUT_THETA };
#[cfg(not(target_arch = "wasm32"))]
const COMPUTE_METHOD: parallel::BarnesHut<f32> = parallel::BarnesHut { theta: BARNES_HUT_THETA };

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gravity::ZERO);
    // Add Particular n-body plugin
    app.register_type::<PhysicsBody>();
    app.add_systems(
        PhysicsSchedule,
        accelerate_particles.in_set(PhysicsStepSet::First),
    );
}


#[derive(Component, Particle, Default, Reflect)]
#[dim(2)]
pub struct PhysicsBody {
    position: Vec2,
    mu: f32,
}

impl PhysicsBody {
    pub fn new(position: Vec2, mass: f32) -> PhysicsBody {
        PhysicsBody {
            position,
            mu: mass * G,
        }
    }
}

#[derive(Bundle, Default)]
pub struct PhysicsBodyBundle {
    pub body: PhysicsBody,
    pub rigidbody: RigidBody,
    pub collider: Collider,
}

fn accelerate_particles(time: Res<Time>, mut query: Query<(&mut PhysicsBody, &mut LinearVelocity)>) {
    query
        .iter()
        .map(|(body, .., )| body )
        .accelerations(&mut COMPUTE_METHOD.clone())
        .map(Vec2::from)
        .zip(&mut query)
        .for_each(|(acceleration, (.., mut velocity))| {
            velocity.0 += acceleration * time.delta_seconds();
        });
}
