//! The screen state for the main game loop.

use bevy::prelude::*;

use super::Screen;
use crate::{
    assets::ImageAssets,
    ui::{slider::*, widgets::Widgets},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(SliderPlugin);
    app.add_systems(OnEnter(Screen::Dev), enter_dev);
}

fn enter_dev(mut commands: Commands, images: Res<ImageAssets>) {
    commands.spawn(SliderBundle {
        style: Style {
            width: Val::Px(200.),
            height: Val::Px(20.),
            margin: UiRect::bottom(Val::Px(15.)),
            ..default()
        },
        background_color: Color::srgb(0.8, 0.8, 0.8).into(),
        ..default()
    }).with_children(|parent| {
        // Adding the slider handle
        parent.spawn(SliderHandleBundle {
            style: Style {
                width: Val::Px(15.), height: Val::Px(20.),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        });
    });
    commands.slider_large(&images);

}
