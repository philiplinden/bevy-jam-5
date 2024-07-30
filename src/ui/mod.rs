//! Reusable UI widgets & theming.

// Unused utilities and re-exports may trigger these lints undesirably.
#![allow(dead_code, unused_imports)]

pub mod interaction;
pub mod palette;
pub mod screens;
pub mod slider;
mod widgets;

pub mod prelude {
    pub use super::{
        interaction::{InteractionPalette, InteractionQuery},
        palette::*,
        screens::*,
        widgets::{Containers as _, Widgets as _},
    };
}
pub use screens::Screen;

use bevy::prelude::*;
use bevy::winit::WinitWindows;
use winit::window::Icon;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((interaction::plugin, screens::plugin));
    app.insert_resource(ClearColor(palette::OSCILLOSCOPE_SCREEN_COLOR));

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Startup, set_window_icon);
}

#[cfg(not(target_arch = "wasm32"))]
fn set_window_icon(
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/images/icon.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    // do it for all windows
    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
