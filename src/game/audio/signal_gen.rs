use bevy::prelude::*;
use bevy_fundsp::prelude::*;
use uuid::Uuid;

use super::MasterVolume;

pub struct SignalGeneratorPlugin;

impl Plugin for SignalGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_signal_parameters);
        app.register_type::<AudioChannel>();
        app.add_event::<SpawnSignalEvent>();
        app.observe(spawn_signal);
        app.add_event::<PlaySignalsEvent>();
        app.observe(play_signals);
    }
}

#[derive(Component)]
pub struct SignalGenerator {
    pub waveform: Waveform,
    pub channel: AudioChannel,
    pub id: SignalId,
}

#[derive(Component)]
pub struct Waveform {
    pub frequency: Shared,
    pub phase: Shared,
}

impl Waveform {
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency.set_value(freq);
    }
    pub fn set_phase(&mut self, phase: f32) {
        self.phase.set_value(phase);
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default, Reflect)]
pub enum AudioChannel {
    Left,
    Right,
    #[default]
    Both,
}

#[derive(Debug, Component, Reflect)]
pub struct SignalId(pub Uuid);

#[derive(Event)]
pub struct SpawnSignalEvent {
    pub frequency: f32,
    pub phase: f32,
    pub channel: AudioChannel,
}

fn spawn_signal(trigger: Trigger<SpawnSignalEvent>, mut commands: Commands) {
    let event = trigger.event();

    let frequency = shared(event.frequency);
    let phase = shared(event.phase);
    let channel = event.channel;

    let signal_pan = match channel {
        AudioChannel::Left => -1.0,
        AudioChannel::Right => 1.0,
        AudioChannel::Both => 0.0,
    };
    let freq = frequency.clone();
    let ph = phase.clone();
    let source = move || sine() * var(&freq) >> pan(signal_pan) >> rotate(ph.value(), 1.0);

    let signal_dsp = SignalGeneratorDsp(source);
    let signal_id = signal_dsp.id();

    commands.add(Dsp(signal_dsp, SourceType::Dynamic));
    commands.spawn(SignalGenerator {
        waveform: Waveform { frequency, phase },
        channel,
        id: SignalId(signal_id),
    });
}

fn update_signal_parameters(
    mut signals: Query<(&mut SignalGenerator, &AudioSink)>,
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

    for (mut signal, sink) in signals.iter_mut() {
        let mut frequency = signal.waveform.frequency.value();
        let mut phase = signal.waveform.phase.value();
        match signal.channel {
            AudioChannel::Left => {
                // Phase control for left channel
                if keyboard_input.pressed(KeyCode::KeyA) {
                    phase -= 0.1 * delta;
                }
                if keyboard_input.pressed(KeyCode::KeyD) {
                    phase += 0.1 * delta;
                }
                // Frequency control for left channel
                if keyboard_input.pressed(KeyCode::KeyW) {
                    frequency *= 1.01;
                }
                if keyboard_input.pressed(KeyCode::KeyS) {
                    frequency *= 0.99;
                }
            }
            AudioChannel::Right => {
                // Phase control for right channel
                if keyboard_input.pressed(KeyCode::ArrowLeft) {
                    phase -= 0.1 * delta;
                }
                if keyboard_input.pressed(KeyCode::ArrowRight) {
                    phase += 0.1 * delta;
                }
                // Frequency control for right channel
                if keyboard_input.pressed(KeyCode::ArrowUp) {
                    frequency *= 1.01;
                }
                if keyboard_input.pressed(KeyCode::ArrowDown) {
                    frequency *= 0.99;
                }
            }
            AudioChannel::Both => {
                // For signals on both channels, we'll update based on either set of controls
                // Phase control
                if keyboard_input.pressed(KeyCode::KeyA)
                    || keyboard_input.pressed(KeyCode::ArrowLeft)
                {
                    phase -= 0.1 * delta;
                }
                if keyboard_input.pressed(KeyCode::KeyD)
                    || keyboard_input.pressed(KeyCode::ArrowRight)
                {
                    phase += 0.1 * delta;
                }
                // Frequency control
                if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp)
                {
                    frequency *= 1.01;
                }
                if keyboard_input.pressed(KeyCode::KeyS)
                    || keyboard_input.pressed(KeyCode::ArrowDown)
                {
                    frequency *= 0.99;
                }
            }
        }

        // Ensure phase stays within 0 to 2π
        signal
            .waveform
            .set_phase(phase.rem_euclid(std::f32::consts::TAU));

        // Clamp frequency to a reasonable range (e.g., 20 Hz to 20000 Hz)
        signal
            .waveform
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
        let source = assets.add(
            dsp_manager
                .get_graph_by_id(&signal_id.0)
                .expect("DSP source"),
        );
        commands.spawn(AudioSourceBundle {
            source,
            ..default()
        });
    }
}
