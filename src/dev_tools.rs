//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, diagnostic::FrameTimeDiagnosticsPlugin, prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        // Print state transitions in dev builds
        app.add_systems(Update, (
            log_transitions::<crate::ui::screens::Screen>,
            log_transitions::<crate::ui::screens::loading::LoadingStatus>,
            log_transitions::<crate::game::oscilloscope::render::DisplayMode>,
        ));
        app.add_plugins((
            WorldInspectorPlugin::default(),
        ));
    }
}
