//! Simple tones.

use bevy::prelude::*;
use bevy_fundsp::prelude::*;
use uuid::Uuid;

use super::{
    dsp::{DspBuffer, SynthDsp}, tee::tee, Channel
};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<SynthPlayer>();
    app.add_event::<SetWaveShapeEvent>();
    app.add_event::<ModulateChannelEvent>();
    app.add_systems(Startup, initialize_synth);
    app.add_systems(OnEnter(SynthPlayer::Playing), play_synths);
}

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Hash, Eq)]
pub enum SynthPlayer {
    Playing,
    #[default]
    Paused,
}

#[derive(Component)]
pub struct SynthTone(pub Shared);

impl SynthTone {
    pub fn set_frequency(&mut self, freq_hz: f32) {
        self.0.set_value(freq_hz)
    }
}

#[derive(Bundle)]
struct SynthBundle {
    channel: Channel,
    tone: SynthWave,
    id: SynthId,
}

#[derive(Debug, Component)]
struct SynthId(pub Uuid);

#[derive(Component, Default)]
pub struct SynthWave {
    wave: Waveform,
    shape: WaveShape,
}

pub struct Waveform {
    amplitude: f32,
    phase: f32,
    frequency: f32,
}

impl Default for Waveform {
    fn default() -> Self {
        Self {
            amplitude: 1.0,
            phase: 0.0,
            frequency: 100.0,
        }
    }
}

#[derive(Default)]
pub enum WaveShape {
    #[default]
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

#[derive(Event)]
pub struct SetWaveShapeEvent(pub WaveShape);

pub enum WaveParam {
    Amplitude,
    Phase,
    Frequency,
}

#[derive(Event)]
pub struct ModulateChannelEvent {
    pub channel: Channel,
    pub parameter: WaveParam,
    pub delta: f32,
}

impl Default for ModulateChannelEvent {
    fn default() -> Self {
        Self {
            channel: Channel(0),
            parameter: WaveParam::Frequency,
            delta: 10.0,
        }
    }
}

fn play_synths(
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
    tones: Query<&SynthTone>,
) {
    for tone in tones.iter() {
        let source = assets.add(
            dsp_manager
                .get_graph_by_id(&tone.id)
                .expect("DSP source"),
        );
        commands.spawn(AudioSourceBundle {
            source,
            ..default()
        });
    }
}

pub fn initialize_synth(mut commands: Commands, tones: Query<&SynthWave>) {
    move |mut commands: Commands| {
        for tone in tones.iter() {
            let wave = tone.wave;
            let left_freq = shared(wave.frequency.into());
            let left_buffer = DspBuffer::new();
            let right_freq = left_freq.clone();
            let right_buffer = DspBuffer::from(&left_buffer);

            let synth = match tone.shape {
                WaveShape::Sine => {let tone = move || var(&right_freq) >> sine() >> tee(&left_buffer.0) >> split::<U2>() * 0.2;},
                WaveShape::Square => {let tone = move || var(&right_freq) >> square() >> tee(&left_buffer.0) >> split::<U2>() * 0.2;},
                WaveShape::Sawtooth => {let tone = move || var(&right_freq) >> saw() >> tee(&left_buffer.0) >> split::<U2>() * 0.2;},
                WaveShape::Triangle => {let tone = move || var(&right_freq) >> triangle() >> tee(&left_buffer.0) >> split::<U2>() * 0.2;},
            };

            let synth_dsp = SynthDsp(synth);
            let synth_id = synth_dsp.id();
            commands.add(Dsp(synth_dsp, SourceType::Dynamic));
            commands.spawn((SynthBundle {
                channel: Channel(0),
                tone: SynthTone(left_freq),
                id: SynthId(synth_id),
            },
                right_buffer,
            ));
        }
    };
}
