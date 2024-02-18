use gamercade_rs::prelude as gc;
use ultraviolet::{Vec2, Vec4};
use wide::{f32x4, u32x4, CmpGt};

use crate::types::Color;

use super::Gpu;

const X_STEP_SIZE: usize = 2;
const Y_STEP_SIZE: usize = 2;

impl Gpu {
    // TODO: Optimize: https://fgiesen.wordpress.com/2013/02/10/optimizing-the-basic-rasterizer/
    // TODO: Consider using Fixed Point math
    // TODO: Incorporate a better boundingbox traversal algorithm
    pub(super) fn rasterize_triangle(&mut self, a: Vec4, b: Vec4, c: Vec4) {
        // Determine the bounding box of the triangle in screen space
        let min_x = a.x.min(b.x).min(c.x).max(0.0) as usize;
        let min_y = a.y.min(b.y).min(c.y).max(0.0) as usize;
        let max_x = (a.x.max(b.x).max(c.x).min((self.screen_width - 1) as f32)) as usize;
        let max_y = (a.y.max(b.y).max(c.y).min((self.screen_height - 1) as f32)) as usize;

        // // Triangle Setup
        let top_left = Vec2::new(min_x as f32, min_y as f32);
        let (a_edge, mut wa_row) = Edge::initialize(b.xy(), c.xy(), top_left);
        let (b_edge, mut wb_row) = Edge::initialize(c.xy(), a.xy(), top_left);
        let (c_edge, mut wc_row) = Edge::initialize(a.xy(), b.xy(), top_left);

        // Iterate over each pixel in the bounding box
        for y in (min_y..=max_y).step_by(Y_STEP_SIZE) {
            // Barycentric coordinates at start of row
            let mut wa = wa_row;
            let mut wb = wb_row;
            let mut wc = wc_row;

            for x in (min_x..=max_x).step_by(X_STEP_SIZE) {
                // If the pixel is inside the triangle (barycentric coordinates are non-negative)
                let zero = f32x4::splat(0.0);
                let wa_mask = wa.cmp_gt(zero);
                let wb_mask = wb.cmp_gt(zero);
                let wc_mask = wc.cmp_gt(zero);
                let mask = wa_mask & wb_mask & wc_mask;

                if mask.any() {
                    self.render_pixels(
                        x,
                        y,
                        RenderVertex::new(a, wa),
                        RenderVertex::new(b, wb),
                        RenderVertex::new(c, wc),
                        mask,
                    );
                }

                // Increment weights one step to the right
                wa += a_edge.step_x;
                wb += b_edge.step_x;
                wc += c_edge.step_x;
            }

            // Increment weights one step down
            wa_row += a_edge.step_y;
            wb_row += b_edge.step_y;
            wc_row += c_edge.step_y;
        }
    }

    fn render_pixels(
        &mut self,
        x: usize,
        y: usize,
        a: RenderVertex,
        b: RenderVertex,
        c: RenderVertex,
        mask: f32x4,
    ) {
        const X_OFFSETS: [i32; 4] = [0, 1, 0, 1];
        const Y_OFFSETS: [i32; 4] = [0, 0, 1, 1];
        // Interpolate attributes (e.g., depth value, texture coordinates) at the current pixel
        let interpolated_depths = 1.0 / (a.depth_weight() + b.depth_weight() + c.depth_weight());

        let pixel_index = y * self.screen_width + x;
        let pixel_indices = u32x4::splat(pixel_index as u32)
            + u32x4::new([0, 1, self.screen_width as u32, self.screen_width as u32 + 1]);
        // Calculate the pixel's index

        // Perform depth testing
        // TODO: Double check this
        let mask = self
            .z_buffer
            .test_and_set(pixel_indices, interpolated_depths, mask);

        // TODO: Double check this / clean it up
        if mask > 0 {
            for bit in 0..4 {
                if (mask & 1 << bit) != 0 {
                    let x = x as i32 + X_OFFSETS[bit];
                    let y = y as i32 + Y_OFFSETS[bit];

                    // Skip Rasterizing if its off the screen
                    if x >= self.screen_width as i32 || y >= self.screen_height as i32 {
                        continue;
                    }

                    // Perform fragment shading (e.g., apply lighting calculations, texture mapping)
                    let fragment_color = Color::new(255, 255, 255);

                    // Write the fragment color to the frame buffer
                    gc::set_pixel(
                        fragment_color.to_graphics_params(),
                        x,
                        self.screen_height as i32 - y,
                    );
                }
            }
        }
    }
}

struct RenderVertex {
    position: Vec4,
    weight: f32x4,
}

impl RenderVertex {
    fn new(position: Vec4, weight: f32x4) -> Self {
        Self { position, weight }
    }

    fn depth_weight(&self) -> f32x4 {
        self.position.z / self.weight
    }
}

struct Edge {
    step_x: f32x4,
    step_y: f32x4,
}

impl Edge {
    fn initialize(v0: Vec2, v1: Vec2, origin: Vec2) -> (Self, f32x4) {
        // Edge setup
        let a = v0.y - v1.y;
        let b = v1.x - v0.x;
        let c = v0.x * v1.y - v0.y * v1.x;

        // Step Deltas
        let step_x = f32x4::splat(a * X_STEP_SIZE as f32);
        let step_y = f32x4::splat(b * Y_STEP_SIZE as f32);

        let out = Self { step_x, step_y };

        // x/y values for initial pixel block
        let x: f32x4 = f32x4::splat(origin.x) + f32x4::new([0.0, 1.0, 0.0, 1.0]);
        let y: f32x4 = f32x4::splat(origin.y) + f32x4::new([0.0, 0.0, 1.0, 1.0]);

        // Edge function weights at origin
        let weight = f32x4::splat(a) * x + f32x4::splat(b) * y + f32x4::splat(c);
        (out, weight)
    }
}
