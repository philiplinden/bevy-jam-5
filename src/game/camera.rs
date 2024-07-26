use bevy::prelude::*;
use bevy_video_glitch::{VideoGlitchPlugin, VideoGlitchSettings};
use crate::game::crt::CrtSettings;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        VideoGlitchPlugin,
    ));
    app.register_type::<VideoGlitchSettings>();
    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera);


}

// Change the intensity over time to show that the effect is controlled from the main world
fn update_glitch_settings(mut settings: Query<&mut VideoGlitchSettings>, time: Res<Time>) {
    for mut setting in &mut settings {
        let mut intensity = time.elapsed_seconds();
        // Make it loop periodically.
        intensity = intensity.sin();
        // Remap it to 0..1 because the intensity can't be negative.
        intensity = intensity * 0.5 + 0.5;
        // Set the intensity.
        //
        // This will then be extracted to the render world and uploaded to the
        // gpu automatically by the [`UniformComponentPlugin`].
        setting.intensity = intensity;
    }
}

fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed {width: 512., height: 512.};
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
        // This component is also used to determine on which camera to run the post processing effect.
        VideoGlitchSettings {
            intensity: 0.4,
            color_aberration: Mat3::IDENTITY,
        },
        CrtSettings {
            intensity: 0.1,
        }
    ));
}
