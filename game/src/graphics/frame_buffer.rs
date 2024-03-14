use std::ops::{Index, IndexMut};

use gamercade_rs::api::graphics_parameters::GraphicsParameters;

pub struct FrameBuffer<const P: usize> {
    pub frame_buffer: [GraphicsParameters; P],
}

impl<const P: usize> FrameBuffer<P> {
    pub fn new() -> Self {
        Self {
            frame_buffer: [GraphicsParameters::default(); P],
        }
    }

    pub(crate) fn clear(&mut self) {
        self.frame_buffer = [GraphicsParameters::default(); P];
    }
}

impl<const P: usize> Index<usize> for FrameBuffer<P> {
    type Output = GraphicsParameters;

    fn index(&self, index: usize) -> &Self::Output {
        &self.frame_buffer[index]
    }
}

impl<const P: usize> IndexMut<usize> for FrameBuffer<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.frame_buffer[index]
    }
}
