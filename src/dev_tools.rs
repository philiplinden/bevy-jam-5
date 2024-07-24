//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::screen::Screen;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Print state transitions in dev builds
        app.add_systems(Update, log_transitions::<Screen>);
        // Add inspector to dev builds
        #[cfg(feature = "inspect")]
        app.add_plugins(WorldInspectorPlugin::default());
    }
}
