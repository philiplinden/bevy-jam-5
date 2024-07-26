//! Handles spawning of entities. Here, we are using
//! [observers](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Observer.html)
//! for this, but you could also use `Events<E>` or `Commands`.

use bevy::prelude::*;

pub mod planetoids;
pub mod satellites;
pub mod oscilloscope;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        planetoids::plugin,
        satellites::plugin,
        oscilloscope::plugin,
    ));
}
