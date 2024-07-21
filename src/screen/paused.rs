//! The screen that appears when the game is paused.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::audio::soundtrack::PlaySoundtrack,
    ui::prelude::*
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Paused), enter_paused);
    app.add_systems(OnExit(Screen::Title), exit_paused);

    app.register_type::<PausedAction>();
    app.add_systems(Update, handle_paused_action.run_if(in_state(Screen::Paused)));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum PausedAction {
    Resume,
    Title,
}

fn enter_paused(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Paused))
        .with_children(|children| {
            children.button("Resume").insert(PausedAction::Resume);
            children.button("Main Menu").insert(PausedAction::Title);
        });
}

fn exit_paused(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Disable);
}

fn handle_paused_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&PausedAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                PausedAction::Resume => next_screen.set(Screen::Playing),
                PausedAction::Title => next_screen.set(Screen::Title),
            }
        }
    }
}
