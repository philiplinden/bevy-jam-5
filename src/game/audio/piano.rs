#![allow(clippy::precedence)]

use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};

struct PianoPlugin;

struct PianoDsp<F>(F);

impl<T: AudioUnit + 'static, F: Send + Sync + 'static + Fn() -> T> DspGraph for PianoDsp<F> {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit> {
        Box::new((self.0)())
    }
}

#[derive(Debug, Component)]
struct PianoId(Uuid);

#[derive(Component)]
struct PitchVar(Shared);

impl PitchVar {
    fn set_pitch(&self, pitch: Pitch) {
        self.0.set_value(pitch.into());
    }
}

#[derive(Debug, Clone, Copy)]
enum Pitch {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl Pitch {
    fn to_f32(self) -> f32 {
        match self {
            Pitch::C => 261.626,
            Pitch::Cs => 277.183,
            Pitch::D => 293.665,
            Pitch::Ds => 311.127,
            Pitch::E => 329.628,
            Pitch::F => 349.228,
            Pitch::Fs => 369.994,
            Pitch::G => 391.995,
            Pitch::Gs => 415.305,
            Pitch::A => 440.0,
            Pitch::As => 466.164,
            Pitch::B => 493.883,
        }
    }
}

impl From<Pitch> for f32 {
    fn from(pitch: Pitch) -> Self {
        pitch.to_f32()
    }
}

#[derive(Component)]
struct Channel(u8);

impl Plugin for PianoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_channel(0))
           .add_systems(Update, switch_key)
           .add_systems(PostStartup, play_piano);
    }
}

fn setup_channel(number: u8) -> impl FnMut(Commands) {
    move |mut commands: Commands| {
        let pitch = shared(Pitch::C.into());
        let pitch2 = pitch.clone();

        let piano = move || var(&pitch2) >> square() >> split::<U2>() * 0.2;
        let piano_dsp = PianoDsp(piano);
        let piano_id = piano_dsp.id();
        commands.add(Dsp(piano_dsp, SourceType::Dynamic));
        commands.spawn((Channel(0),
                        PitchVar(pitch),
                        PianoId(piano_id)));
    }
}

fn switch_key(input: Res<ButtonInput<KeyCode>>, mut pitch_vars: Query<(&Channel, &mut PitchVar)>) {
    let mut keypress = |keycode, pitch| {
        if input.just_pressed(keycode) {
            for (channel, pitch_var) in &mut pitch_vars {
                if channel.0 == 0 {
                    pitch_var.set_pitch(pitch);
                }
            }
        }
    };

    keypress(KeyCode::KeyA, Pitch::C);
    keypress(KeyCode::KeyW, Pitch::Cs);
    keypress(KeyCode::KeyS, Pitch::D);
    keypress(KeyCode::KeyE, Pitch::Ds);
    keypress(KeyCode::KeyD, Pitch::E);
    keypress(KeyCode::KeyF, Pitch::F);
    keypress(KeyCode::KeyT, Pitch::Fs);
    keypress(KeyCode::KeyG, Pitch::G);
    keypress(KeyCode::KeyY, Pitch::Gs);
    keypress(KeyCode::KeyH, Pitch::A);
    keypress(KeyCode::KeyU, Pitch::As);
    keypress(KeyCode::KeyJ, Pitch::B);
}

fn play_piano(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
    piano_ids: Query<&PianoId>,
) {
    for piano_id in piano_ids.iter() {
    let source = assets.add(
        dsp_manager
            .get_graph_by_id(&piano_id.0)
            .expect("DSP source"),
    );
    commands.spawn(AudioSourceBundle {
        source,
        ..default()
    });
    }
}