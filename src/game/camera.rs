use bevy::prelude::*;
use bevy_video_glitch::{VideoGlitchPlugin, VideoGlitchSettings};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        VideoGlitchPlugin,
    ));

    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2dBundle::default(),
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
        // This component is also used to determine on which camera to run the post processing effect.
        VideoGlitchSettings {
            intensity: 1.0,
            color_aberration: Mat3::IDENTITY,
        },
    ));
}
