//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use super::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), enter_credits);

    app.add_systems(
        Update,
        handle_credits_action.run_if(in_state(Screen::Credits)),
    );
    app.register_type::<CreditsAction>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(Component)]
enum CreditsAction {
    Back,
}

fn enter_credits(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.header("Made by");
            children.label("Philip Linden @philiplinden - Concept & Code");
            children.label("Shane Celis @shanecelis - Art & Code");
            children.label("David Breen - Concept");

            children.header("Assets");
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Used with permission.");
            children.label("Music - CC-0 by DOS-88");

            children.button("Back").insert(CreditsAction::Back);
        });
}

fn handle_credits_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&CreditsAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                CreditsAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
