// use crate::game::crt::{CrtPlugin, CrtSettings};
use bevy::prelude::*;
use bevy_video_glitch::{VideoGlitchPlugin, VideoGlitchSettings};
use super::crt::{CrtPlugin, CrtSettings};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        VideoGlitchPlugin,
        CrtPlugin,
    ));
    app.register_type::<VideoGlitchSettings>();
    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera);
}

// // Change the intensity over time to show that the effect is controlled from the main world
// fn breathe_glitch_settings(mut settings: Query<&mut VideoGlitchSettings>, time: Res<Time>) {
//     for mut setting in &mut settings {
//         let mut intensity = time.elapsed_seconds();
//         // Make it loop periodically.
//         intensity = intensity.sin().abs(); // the intensity can't be negative

//         // Set the intensity.
//         //
//         // This will then be extracted to the render world and uploaded to the
//         // gpu automatically by the [`UniformComponentPlugin`].
//         setting.intensity = intensity;
//     }
// }

fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    // camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed {
    //     width: 512.,
    //     height: 512.,
    // };
    commands.spawn((
        Name::new("Camera"),
        camera_bundle,
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
}
