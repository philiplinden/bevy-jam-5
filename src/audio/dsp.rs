#![allow(clippy::precedence)]
use std::f32::consts::FRAC_PI_2;
use bevy::prelude::*;
use bevy_fundsp::prelude::*;

use super::{MasterVolume, AudioChannel};

pub const BUFFER_SIZE: usize = 1000;

// https://github.com/harudagondi/bevy_fundsp/pull/6
//Added
// A way to play streaming DSP sources. See SourceType::Dynamic.
// You can play DSP sources using Audio::play_dsp.
// Two iterators on streaming audio sources: Iter and IterMono.
//Changed
// Adding the DSP plugin.
// No more initializing using DspAssets!
// Just add your DSP function using app.add_dsp_source
// Playing DSP sources require Audio to be mutable. (Use ResMut)

pub fn plugin(app: &mut App) {
    app.add_plugins((
        DspPlugin::default(),
        WaveformGeneratorPlugin,
    ));
}

pub struct WaveformGeneratorPlugin;

impl Plugin for WaveformGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AudioChannel>();
        app.add_systems(Startup, setup_channel(0));
        // app.add_systems(PostStartup, (
        //     spawn_signal(440.0, 0.0, AudioChannel::Left),
        //     spawn_signal(440.0, FRAC_PI_2, AudioChannel::Right),
        // ));
        // app.add_systems(Update, update_signal_parameters);
        // app.add_event::<PlaySignalsEvent>();
        // app.observe(play_signals);
    }
}

#[derive(Debug, PartialEq)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
    Organ,
    Hammond,
    Pulse,
    Pluck,
    Noise,
}

struct Signal {
    waveform: Waveform,
    frequency: Shared,
    phase: Shared,
}
impl Signal {
    pub fn generate(&self) -> impl AudioUnit32 {
        let freq = self.frequency.get_value();
        match self.waveform {
            Waveform::Sine => sine_hz(freq),
            Waveform::Square => square_hz(freq),
            Waveform::Saw => todo!(),
            Waveform::Triangle => todo!(),
            Waveform::Organ => todo!(),
            Waveform::Hammond => todo!(),
            Waveform::Pulse => todo!(),
            Waveform::Pluck => todo!(),
            Waveform::Noise => todo!(),
        }
    }

    pub fn set_waveform(&mut self, shape: Waveform) {
        self.waveform = shape;
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency.set_value(freq);
    }

    pub fn set_phase(&mut self, phase: f32) {
        self.phase.set_value(phase);
    }
}

#[derive(Debug, Component, Reflect)]
pub struct SignalId(pub Uuid);

#[derive(Bundle)]
pub struct SignalBundle {
    signal: Signal,
    id: SignalId,
}

#[derive(Event)]
pub struct SpawnSignalEvent {
    pub frequency: f32,
    pub phase: f32,
    pub channel: AudioChannel,
}

fn spawn_signal(frequency: f32, phase: f32, channel: AudioChannel) -> impl FnMut(Commands) {

    move |mut commands: Commands| {

        let frequency = shared(frequency);
        let phase = shared(phase);
        let channel = channel;
        let buffer = DspBuffer::new();
        let buffer_clone = DspBuffer::from(&buffer);

        let freq = frequency.clone();
        let ph = phase.clone();
        // let source = move || sine() * var(&freq) >> pan(signal_pan) >> rotate(ph.value(), 1.0) >> tee(&buffer.0);
        let source = move || sine_hz(freq.value()) >> tee(&buffer.0);

        let signal_dsp = SignalGeneratorDsp(source);
        let signal_id = signal_dsp.id();

        commands.add(Dsp(signal_dsp, SourceType::Dynamic));
        commands.spawn(
            SignalGeneratorBundle {
                waveform: Waveform { frequency, phase },
                channel,
                id: SignalId(signal_id),
                buffer: buffer_clone,
            });
        }
}

fn update_signal_parameters(
    mut signals: Query<(&mut Waveform, &AudioChannel, &AudioSink)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut master_volume: ResMut<MasterVolume>,
) {
    let delta = time.delta_seconds();

    // Global volume control
    if keyboard_input.pressed(KeyCode::Equal) || keyboard_input.pressed(KeyCode::NumpadAdd) {
        master_volume.0 = (master_volume.0 + 0.1 * delta).min(1.0);
    }
    if keyboard_input.pressed(KeyCode::Minus) || keyboard_input.pressed(KeyCode::NumpadSubtract) {
        master_volume.0 = (master_volume.0 - 0.1 * delta).max(0.0);
    }

    for (mut waveform, channel, sink) in signals.iter_mut() {

        }

        // Ensure phase stays within 0 to 2π
        waveform
            .set_phase(phase.rem_euclid(std::f32::consts::TAU));

        // Clamp frequency to a reasonable range (e.g., 20 Hz to 20000 Hz)
        waveform
            .set_frequency(frequency.clamp(20.0, 20000.0));

        // Update the AudioSink with new parameters
        sink.set_volume(master_volume.0);
    }
}

pub struct SignalGeneratorDsp<F>(F);

impl<T: AudioUnit + 'static, F: Send + Sync + 'static + Fn() -> T> DspGraph
    for SignalGeneratorDsp<F>
{
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit> {
        Box::new((self.0)())
    }
}

#[derive(Event)]
pub struct PlaySignalsEvent;

fn play_signals(
    _trigger: Trigger<PlaySignalsEvent>,
    mut commands: Commands,
    mut assets: ResMut<Assets<DspSource>>,
    dsp_manager: Res<DspManager>,
    signal_ids: Query<&SignalId>,
) {
    for signal_id in signal_ids.iter() {
        info!("Trying to play dsp source {:?}", signal_id);
        let source = assets.add(
            dsp_manager
                .get_graph_by_id(&signal_id.0)
                .expect("Could not spawn DSP source"),
        );
        commands.spawn(AudioSourceBundle {
            source,
            ..default()
        });
    }
}

pub struct SynthDsp<F>(pub F);

impl<T: AudioUnit + 'static, F: Send + Sync + 'static + Fn() -> T> DspGraph for SynthDsp<F> {
    fn id(&self) -> Uuid {
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }

    fn generate_graph(&self) -> Box<dyn AudioUnit> {
        Box::new((self.0)())
    }
}

#[derive(Debug, Component)]
pub struct DspBuffer(pub Arc<Mutex<CircularBuffer<BUFFER_SIZE, f32>>>);

impl DspBuffer {
    pub fn new() -> Self {
        DspBuffer(Arc::new(Mutex::new(CircularBuffer::new())))
    }
}

impl From<&DspBuffer> for DspBuffer {
    fn from(value: &DspBuffer) -> Self {
        DspBuffer(Arc::clone(&value.0))
    }
}

/// Compute (x, y) display coordinates of a sine wave over time.
pub fn to_xy() {
    // no op
}

fn mix_sources() {
    // no op
}
