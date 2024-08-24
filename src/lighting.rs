use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PostStartup, setup_lighting);
}

fn setup_lighting(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            illuminance: 1_500.,
            ..default()
        },
        ..default()
    });
}
