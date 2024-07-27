//! The screen state for the main game loop.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::oscilloscope,
    ui::{interaction, palette::OSCILLOSCOPE_SCREEN_COLOR},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.insert_resource(ClearColor(OSCILLOSCOPE_SCREEN_COLOR));
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(oscilloscope::SpawnOscilloscope);
}
