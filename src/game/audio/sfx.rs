use bevy::{audio::PlaybackMode, prelude::*};

pub(super) fn plugin(app: &mut App) {
    // app.observe(play_sfx);
}

// fn play_sfx(
//     trigger: Trigger<PlaySfx>,
//     mut commands: Commands,
//     sfx_handles: Res<HandleMap<SfxAssets>>,
// ) {
//     let sfx_key = match trigger.event() {
//         PlaySfx::Key(key) => *key,
//     };
//     commands.spawn(AudioSourceBundle {
//         source: sfx_handles[&sfx_key].clone_weak(),
//         settings: PlaybackSettings {
//             mode: PlaybackMode::Despawn,
//             ..default()
//         },
//     });
// }
