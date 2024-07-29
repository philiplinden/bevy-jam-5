//! This module performs signal processing on audio channels, such as sythesis, modulation, combination, and so on.
#![allow(clippy::precedence)]
use std::sync::{Mutex, Arc};
use circular_buffer::CircularBuffer;

use {bevy::prelude::*, bevy_fundsp::prelude::*, uuid::Uuid};

pub const BUFFER_SIZE: usize = 1000;

pub fn plugin(app: &mut App) {

}

#[derive(Component)]
pub struct Channel(pub u8);

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
