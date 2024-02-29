// These dimensions work with from 160 x 90 up to 1920 x 1080
// Valid Tile Widths: 4, 8, 16, 20, 32, 40, 80, 160
// Valid Tile Heights: 1, 2, 3, 5, 6, 9, 10, 15, 18, 30, 45, 90

// These dimensions work with VeryLow (128 x 72), and all others
// VeryLow Valid Widths: 4, 8, 16, 32
// VeryLow Valid Heights 1, 2, 3, 6, 9, 18

use super::{FrameBuffer, ZBuffer};

pub struct RenderTile<const W: usize, const H: usize> {
    x: usize,
    y: usize,
    z_bufffer: ZBuffer,
    frame_buffer: FrameBuffer,
}

impl<const W: usize, const H: usize> RenderTile<W, H> {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            z_bufffer: ZBuffer::new(W, H),
            frame_buffer: FrameBuffer::new(W, H),
        }
    }
}
