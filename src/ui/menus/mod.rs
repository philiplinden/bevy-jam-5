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
