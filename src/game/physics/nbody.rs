// N-body accelerations

use bevy::prelude::*;
use avian2d::prelude::*;
use particular::prelude::*;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION;

pub const G: f32 = NEWTONIAN_CONSTANT_OF_GRAVITATION as f32;

#[cfg(target_arch = "wasm32")]
const COMPUTE_METHOD: sequential::BruteForceSIMD<4> = sequential::BarnesHut;
#[cfg(not(target_arch = "wasm32"))]
const COMPUTE_METHOD: parallel::BruteForceSIMD<8> = parallel::BruteForceSIMD;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(Gravity::ZERO);
    // Add Particular n-body plugin
    app.add_systems(
        PhysicsSchedule,
        accelerate_particles.in_set(PhysicsSet::Prepare),
    );
}


#[derive(Component, Default)]
pub struct PhysicsBody {
    pub position: Position,
    pub velocity: LinearVelocity,
    pub mass: Mass,
}

impl Particle for PhysicsBody {
    type Array = [f32; 2];

    fn position(&self) -> [f32; 2] {
        self.position.0.into()
    }

    fn mu(&self) -> f32 {
        self.mass.0 * G
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

// fn circular_velocity(first_body: RigidBody, second_body: RigidBody)
