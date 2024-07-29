//! This module performs signal processing on audio channels, such as sythesis, modulation, combination, and so on.
#![allow(clippy::precedence)]
use std::sync::{Mutex, Arc};
use circular_buffer::CircularBuffer;

use bevy::prelude::*;

pub const BUFFER_SIZE: usize = 1000;

pub fn plugin(app: &mut App) {

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
