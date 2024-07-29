use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};
use super::tee::tee;
use crate::game::dsp::DspBuffer;

pub struct PianoPlugin;

impl Plugin for PianoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_channel(0));
        app.add_systems(PostStartup, play_pianos);
        app.observe(switch_key);
    }
}

pub struct PianoDsp<F>(F);

impl<T: AudioUnit + 'static, F: Send + Sync + 'static + Fn() -> T> DspGraph for PianoDsp<F> {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit> {
        Box::new((self.0)())
    }
}

#[derive(Debug, Component)]
pub struct PianoId(pub Uuid);

#[derive(Component)]
pub struct PitchVar(pub Shared);

impl PitchVar {
    pub fn set_pitch(&mut self, pitch: Pitch) {
        self.0.set_value(pitch.into());
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.0.set_value(freq);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum Pitch {
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
            // Octave 4
            // Pitch::HighC => 261.626,
            // Pitch::HighCs => 277.183,
            // Pitch::HighD => 293.665,
            // Pitch::HighDs => 311.127,
            // Pitch::HighE => 329.628,
            // Pitch::HighF => 349.228,
            // Pitch::HighFs => 369.994,
            // Pitch::HighG => 391.995,
            // Pitch::HighGs => 415.305,
            // Pitch::HighA => 440.0,
            // Pitch::HighAs => 466.164,
            // Pitch::HighB => 493.883,

            // Octave 2
            Pitch::C => 65.,
            Pitch::Cs => 69.,
            Pitch::D => 73.,
            Pitch::Ds => 77.,
            Pitch::E => 82.,
            Pitch::F => 87.,
            Pitch::Fs => 92.,
            Pitch::G => 98.,
            Pitch::Gs => 104.,
            Pitch::A => 110.,
            Pitch::As => 116.,
            Pitch::B => 123.,
        }
    }
}

impl From<Pitch> for f32 {
    fn from(pitch: Pitch) -> Self {
        pitch.to_f32()
    }
}

#[derive(Component)]
pub struct Channel(pub u8);

#[derive(Event)]
pub struct SetPitchEvent(pub Pitch);

fn switch_key(trigger: Trigger<SetPitchEvent>, mut pitch_vars: Query<(&Channel, &mut PitchVar)>) {
    for (channel, mut pitch_var) in &mut pitch_vars {
        if channel.0 == 0 {
            pitch_var.set_pitch(trigger.event().0);
        }
    }
}

#[derive(Bundle)]
struct PianoBundle{
    channel: Channel,
    pitch: PitchVar,
    id: PianoId,
}

pub fn setup_channel(number: u8) -> impl FnMut(Commands) {
    move |mut commands: Commands| {
        let pitch = shared(Pitch::C.into());
        let pitch2 = pitch.clone();
        let buffer = DspBuffer::new();
        let buffer2 = DspBuffer::from(&buffer);

        // let piano = move || var(&pitch2) >> square() * 0.2 >> tee(&buffer) >> split::<U2>();
        let piano = move || var(&pitch2) >> square() >> tee(&buffer.0) >> split::<U2>() * 0.2;

        let piano_dsp = PianoDsp(piano);
        let piano_id = piano_dsp.id();
        commands.add(Dsp(piano_dsp, SourceType::Dynamic));
        commands.spawn((PianoBundle {
            channel: Channel(0),
            pitch: PitchVar(pitch),
            id: PianoId(piano_id),
        },
            buffer2,
        ));
    }
}

fn play_pianos(
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
