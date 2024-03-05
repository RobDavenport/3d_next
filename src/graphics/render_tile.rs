// These dimensions work with from 160 x 90 up to 1920 x 1080
// Valid Tile Widths: 4, 8, 16, 20, 32, 40, 80, 160
// Valid Tile Heights: 1, 2, 3, 5, 6, 9, 10, 15, 18, 30, 45, 90

// These dimensions work with VeryLow (128 x 72), and all others
// VeryLow Valid Widths: 4, 8, 16, 32
// VeryLow Valid Heights 1, 2, 3, 6, 9, 18

use glam::{Vec2, Vec2Swizzles, Vec3Swizzles};

use super::{rasterizer::RenderTriangle, FrameBuffer, ZBuffer};

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

    pub(super) fn triangle_edges_intersect_aabb<const P: usize>(
        &self,
        triangle: &RenderTriangle<P>,
    ) -> bool {
        let ab = self.line_segment_intersects_aabb(triangle.a.xy(), triangle.b.xy());
        let bc = self.line_segment_intersects_aabb(triangle.b.xy(), triangle.c.xy());
        let ca = self.line_segment_intersects_aabb(triangle.c.xy(), triangle.a.xy());

        ab | bc | ca
    }

    fn line_segment_intersects_aabb(&self, a: Vec2, b: Vec2) -> bool {
        let aabb_min_y = self.y as f32;
        let aabb_min_x = self.x as f32;
        let aabb_max_x = (self.x + W) as f32;
        let aabb_max_y = (self.y + H) as f32;

        // Check if any part of the line segment is within the AABB
        if point_in_aabb(a, aabb_min_x, aabb_min_y, aabb_max_x, aabb_max_y)
            || point_in_aabb(b, aabb_min_x, aabb_min_y, aabb_max_x, aabb_max_y)
        {
            return true;
        }

        // Check for intersection with any side of the AABB
        line_intersects_aabb(a, b, aabb_min_x, aabb_min_y, aabb_max_x, aabb_max_y)
    }
}

// Helper function to check if a point is within an AABB
fn point_in_aabb(point: Vec2, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> bool {
    point.x >= min_x && point.x <= max_x && point.y >= min_y && point.y <= max_y
}

// Helper function to check if a line segment intersects with an AABB
fn line_intersects_aabb(
    a: Vec2,
    b: Vec2,
    aabb_min_x: f32,
    aabb_min_y: f32,
    aabb_max_x: f32,
    aabb_max_y: f32,
) -> bool {
    // Check if any part of the line segment intersects with the AABB's sides
    let dx = b.x - a.x;
    let dy = b.y - a.y;

    // Check for intersection with the left and right sides of the AABB
    let mut t_min = (aabb_min_x - a.x) / dx;
    let mut t_max = (aabb_max_x - a.x) / dx;

    if dx < 0.0 {
        std::mem::swap(&mut t_min, &mut t_max);
    }

    // Check for intersection with the top and bottom sides of the AABB
    let mut t3 = (aabb_min_y - a.y) / dy;
    let mut t4 = (aabb_max_y - a.y) / dy;

    if dy < 0.0 {
        std::mem::swap(&mut t3, &mut t4);
    }

    t_min = t_min.max(t3);
    t_max = t_max.min(t4);

    t_max >= t_min && t_max >= 0.0 && t_min <= 1.0
}
