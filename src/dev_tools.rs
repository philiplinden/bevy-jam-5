//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use avian2d::debug_render::PhysicsDebugPlugin;

use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, log_transitions::<Screen>);
    // Add inspector to dev builds
    #[cfg(feature = "inspect")]
    app.add_plugins(WorldInspectorPlugin::default());

    #[cfg(feature = "physicsdebug")]
    app.add_plugins(PhysicsDebugPlugin::default());
}
