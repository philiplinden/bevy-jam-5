//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use avian2d::prelude::PhysicsDebugPlugin;

use crate::ui::screens::Screen;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        // Print state transitions in dev builds
        app.add_systems(Update, log_transitions::<Screen>);
        app.add_plugins((
            WorldInspectorPlugin::default(),
            PhysicsDebugPlugin::default(),
        ));
    }
}
