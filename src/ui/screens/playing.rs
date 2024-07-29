//! The screen state for the main game loop.

use bevy::prelude::*;

use super::Screen;
use crate::{
    assets::ImageAssets,
    game::oscilloscope::{self, SpawnOscilloscope, SetDisplayModeEvent, DisplayMode, OscilloscopeImage},
    ui::{interaction, palette::OSCILLOSCOPE_SCREEN_COLOR, widgets::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), (enter_playing, oscilloscope::setup_camera).chain());
    app.insert_resource(ClearColor(OSCILLOSCOPE_SCREEN_COLOR));
}

fn enter_playing(mut commands: Commands, osc_image: Res<OscilloscopeImage>, images: Res<ImageAssets>) {
    commands.trigger(SpawnOscilloscope);
    commands.trigger(SetDisplayModeEvent(DisplayMode::XY));

    commands
        .ui_root()
        .insert(StateScoped(Screen::Playing))
        .with_children(|children| {
            children.label("Playing");
            children.slider_large(&images);
            children.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(1218.), height: Val::Px(975.),
                    ..default()
                },
                image: osc_image.0.clone().into(),
                ..default()
            });
        });
}
