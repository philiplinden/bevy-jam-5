//! We make our own version of linux's `tee()` command line function for convenience

use std::sync::{Arc, Mutex};
use circular_buffer::CircularBuffer;
use bevy::prelude::*;
use bevy_fundsp::prelude::*;

/// Tee through input unchanged.
#[derive(Clone, Deref, DerefMut)]
pub struct Tee<const N: usize> {
    #[deref]
    buffer: Arc<Mutex<CircularBuffer::<N, f32>>>
}

impl<const N: usize> Tee<N> {
    pub fn new(buffer: Arc<Mutex<CircularBuffer::<N, f32>>>) -> Self {
        Tee {
            buffer
        }
    }
}

// Note. We have separate Tee and MultiPass structs
// because it helps a little with type inference.
impl<const N: usize> AudioNode for Tee<N> {
    const ID: u64 = 148;
    type Inputs = U1;
    type Outputs = U1;

    #[inline]
    fn tick(&mut self, input: &Frame<f32, Self::Inputs>) -> Frame<f32, Self::Outputs> {
        let mut lock = self.buffer.try_lock();
        if let Ok(ref mut mutex) = lock {
            mutex.push_back(input[0]);
            // eprintln!("{}, ", input[0]);
        }
        *input
    }
    fn process(&mut self, size: usize, input: &BufferRef, output: &mut BufferMut) {
        let mut lock = self.buffer.try_lock();
        if let Ok(ref mut mutex) = lock {
            for i in 0..simd_items(size) {
                let v = input.at(0, i);
                let a: &[f32] = v.as_array_ref();
                for x in a {
                    mutex.push_back(*x);
            eprintln!(",");
                }
                output.set(0, i, v);
            }
        } else {
            for i in 0..simd_items(size) {
                output.set(0, i, input.at(0, i));
            }
        }
    }
    fn route(&mut self, input: &SignalFrame, _frequency: f64) -> SignalFrame {
        input.clone()
    }
}

pub fn tee<const N: usize>(buffer: &Arc<Mutex<CircularBuffer::<N, f32>>>) -> An<Tee<N>> {
    An(Tee::new(Arc::clone(buffer)))
}
