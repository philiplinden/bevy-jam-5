//! The game's main screen states and transitions between them.

mod credits;
pub mod loading;
pub mod playing;
mod splash;
mod title;
mod dev;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();
    app.register_type::<InteractionPalette>();
    app.add_systems(Startup, spawn_ui_camera);
    app.add_systems(Update, apply_interaction_palette);
    app.add_plugins((
        splash::plugin,
        loading::plugin,
        title::plugin,
        playing::plugin,
        credits::plugin,
        dev::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum Screen {
    #[default]
    Splash,
    Loading,
    Title,
    Credits,
    Playing,
    Dev,
}

pub type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, T), Changed<Interaction>>;

/// Palette for widget interactions.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut palette_query: InteractionQuery<(&InteractionPalette, &mut BackgroundColor)>,
) {
    for (interaction, (palette, mut background)) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        // Camera3dBundle{
        //     projection: Projection::Orthographic(OrthographicProjection{
        //         scale: 0.01,
        //         ..default()
        //     }),
        //     ..default()
        // }
        Camera2dBundle::default(),
        // Render all UI to this camera.
        // Not strictly necessary since we only use one camera,
        // but if we don't use this component, our UI will disappear as soon
        // as we add another camera. This includes indirect ways of adding cameras like using
        // [ui node outlines](https://bevyengine.org/news/bevy-0-14/#ui-node-outline-gizmos)
        // for debugging. So it's good to have this here for future-proofing.
        IsDefaultUiCamera,
    ));
    commands.insert_resource(UiScale(0.5));
}
