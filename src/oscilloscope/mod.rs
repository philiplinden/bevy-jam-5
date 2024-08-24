//! This is the base module for rendering the oscilloscope display.
pub mod crt;
mod material;

use bevy::{prelude::*, render::view::RenderLayers};

use crate::{assets::ImageAssets, ui::Screen};
use material::OscilloscopeMaterial;
use crt::CrtDisplay;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((crt::plugin, material::plugin));
    app.observe(new_oscilloscope);
}

#[derive(Bundle)]
pub struct OscilloscopeBundle {
    crt: CrtDisplay,
    frontpanel: ImageBundle,
}

#[derive(Event)]
pub struct SpawnOscilloscope;


pub fn new_oscilloscope(
    _trigger: Trigger<SpawnOscilloscope>,
    mut commands: Commands,
    images: Res<ImageAssets>,
) {
    info!("spawning new o-scope");
    commands
        .spawn((
            RenderLayers::layer(1),
            StateScoped(Screen::Playing),
            ImageBundle {
                style: Style {
                    width: Val::Px(3280.),
                    height: Val::Px(2038.),
                    ..default()
                },
                image: images.reference_no_bg.clone().into(),
                ..default()
            }
        ));
}
