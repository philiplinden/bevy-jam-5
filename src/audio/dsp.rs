//! Modify, route, and combine signals.

#![allow(clippy::precedence)]
use bevy::prelude::*;
use bevy_fundsp::prelude::*;
use uuid::Uuid;
use circular_buffer::CircularBuffer;

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
        SignalGeneratorPlugin,
    ));
}

/// Generates clean tones.
pub struct SignalGeneratorPlugin;

impl Plugin for SignalGeneratorPlugin {
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
    Hammond,
    Pulse,
    Pluck,
    Noise,
}

struct Signal {
    waveform: Waveform,
    frequency: f32,
    phase: f32,
}

impl Signal {
    pub fn generate(&self) -> impl AudioUnit {
        let freq = self.frequency.get_value();
        match self.waveform {
            Waveform::Sine => sine_hz(freq),
            Waveform::Square => square_hz(freq),
            Waveform::Saw => todo!(),
            Waveform::Triangle => todo!(),
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
        self.frequency.set_value(freq.clamp(20.0, 20000.0));
    }

    pub fn set_phase(&mut self, phase: f32) {
        self.phase.set_value(phase.rem_euclid(std::f32::consts::TAU));
    }
}

/// A marker component mapped to the id of an audio signal. This is used to cheaply query references to signals with ECS
#[derive(Debug, Component, Reflect)]
pub struct SignalId(pub Uuid);

#[derive(Bundle)]
pub struct SignalBundle {
    signal: Signal,
    id: SignalId,
}
