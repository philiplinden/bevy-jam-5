//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
};
#[cfg(features = "inspect")]
use bevy_inspector_egui::prelude::WorldInspectorPlugin;
#[cfg(features = "physics_debug")]
use avian2d::prelude::*;

use crate::ui::screens::Screen;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        // Print state transitions in dev builds
        app.add_systems(Update, log_transitions::<Screen>);
        #[cfg(features = "inspect")]
        app.add_plugins(WorldInspectorPlugin);
        #[cfg(features = "physics_debug")]
        app.add_plugins(PhysicsDebugPlugin);
    }
}
