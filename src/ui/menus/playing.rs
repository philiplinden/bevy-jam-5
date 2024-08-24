//! The screen state for the main game loop.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::Screen;
use crate::{
    assets::ImageAssets,
    oscilloscope::{
        crt::{DisplayMode, SetDisplayModeEvent},
        SpawnOscilloscope,
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    // app.insert_resource(ClearColor(OSCILLOSCOPE_SCREEN_COLOR));
}

fn enter_playing(
    mut commands: Commands,
) {
    commands.trigger(SpawnOscilloscope);
    commands.trigger(SetDisplayModeEvent(DisplayMode::XY));
    // commands.trigger(PlaySignalsEvent);
}
