use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::game::oscilloscope::waveform::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(Update, apply_interaction_palette);
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

pub const X_PHASE_AXIS: VirtualAxis = VirtualAxis::horizontal_arrow_keys();
pub const X_FREQUENCY_AXIS: VirtualAxis = VirtualAxis::vertical_arrow_keys();
pub const Y_PHASE_AXIS: VirtualAxis = VirtualAxis::ad();
pub const Y_FREQUENCY_AXIS: VirtualAxis = VirtualAxis::ws();

#[derive(Component)]
pub struct WaveformControls {
    pub phase_axis: VirtualAxis,
    pub frequency_axis: VirtualAxis,
}
