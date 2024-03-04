// These dimensions work with from 160 x 90 up to 1920 x 1080
// Valid Tile Widths: 4, 8, 16, 20, 32, 40, 80, 160
// Valid Tile Heights: 1, 2, 3, 5, 6, 9, 10, 15, 18, 30, 45, 90

// These dimensions work with VeryLow (128 x 72), and all others
// VeryLow Valid Widths: 4, 8, 16, 32
// VeryLow Valid Heights 1, 2, 3, 6, 9, 18

use super::{gpu::TRIANGLES_PER_BIN, rasterizer::RenderTriangle, FrameBuffer, ZBuffer};

struct BinnedTriangleWrapper {
    pointer: *const u8,
    param_count: u32,
}

// W and H represent the width and height of the tile
pub(super) struct RenderTile<const W: usize, const H: usize> {
    pub(super) x: usize, // Left point
    pub(super) y: usize, // Top point
    pub(super) z_buffer: ZBuffer,
    pub(super) frame_buffer: FrameBuffer,
}

impl<const W: usize, const H: usize> RenderTile<W, H> {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            z_buffer: ZBuffer::new(W, H),
            frame_buffer: FrameBuffer::new(W, H),
        }
    }

    pub(super) fn overlap<const P: usize>(&self, triangle: &RenderTriangle<P>) -> bool {
        let min_x = self.x as f32;
        let min_y = self.y as f32;
        let max_x = (self.x + W) as f32;
        let max_y = (self.y + H) as f32;

        if min_x <= triangle.max_x
            && max_x >= triangle.min_x
            && min_y <= triangle.max_y
            && max_y >= triangle.min_y
        {
            true
        } else {
            false
        }
    }

    pub(super) fn bin_triangle<const P: usize>(&mut self, trivial: bool, triangle: RenderTriangle<P>) {
    }
}
