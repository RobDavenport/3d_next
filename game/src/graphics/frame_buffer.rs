use std::ops::{Index, IndexMut};

use gamercade_rs::api::graphics_parameters::GraphicsParameters;

pub struct FrameBuffer {
    pub frame_buffer: Box<[GraphicsParameters]>,
}

impl FrameBuffer {
    pub fn new(screen_width: usize, screen_height: usize) -> Self {
        Self {
            frame_buffer: (0..(screen_height * screen_width))
                .map(|_| GraphicsParameters::default())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub(crate) fn clear(&mut self) {
        self.frame_buffer
            .iter_mut()
            .for_each(|d| *d = GraphicsParameters::default());
    }
}

impl Index<usize> for FrameBuffer {
    type Output = GraphicsParameters;

    fn index(&self, index: usize) -> &Self::Output {
        &self.frame_buffer[index]
    }
}

impl IndexMut<usize> for FrameBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.frame_buffer[index]
    }
}
