//! The screen state for the main game loop.

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::Screen;
use crate::{
    assets::ImageAssets,
    oscilloscope::{
        render::{DisplayMode, SetDisplayModeEvent},
        OscilloscopeImage, SpawnOscilloscope,
    },
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.insert_resource(ClearColor(OSCILLOSCOPE_SCREEN_COLOR));
}

fn enter_playing(
    mut commands: Commands,
    osc_image: Res<OscilloscopeImage>,
    images: Res<ImageAssets>,
) {
    commands.trigger(SpawnOscilloscope);
    commands.trigger(SetDisplayModeEvent(DisplayMode::XY));
    // commands.trigger(PlaySignalsEvent);

    commands
        .ui_root()
        .insert(StateScoped(Screen::Playing))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(3280.),
                        height: Val::Px(2038.),
                        ..default()
                    },
                    image: images.reference_no_bg.clone().into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.label("Playing");
                    parent.slider_large(&images);
                    parent.spawn((
                        Name::new("Oscilloscope Node"),
                        ImageBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                left: Val::Px(933.),
                                top: Val::Px(421.),
                                width: Val::Px(1416.),
                                height: Val::Px(1142.),
                                ..default()
                            },
                            image: osc_image.0.clone().into(),
                            ..default()
                        },
                    ));
                });
        });
}
