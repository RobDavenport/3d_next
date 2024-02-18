use gamercade_rs::prelude as gc;
use ultraviolet::{Vec2, Vec4};

use crate::types::Color;

use super::Gpu;

impl Gpu {
    // TODO: Optimize: https://fgiesen.wordpress.com/2013/02/10/optimizing-the-basic-rasterizer/
    // TODO: Consider using Fixed Point math
    // TODO: Do SIMD Magic for faster performance
    pub(super) fn rasterize_triangle(&mut self, a: Vec4, b: Vec4, c: Vec4) {
        // Determine the bounding box of the triangle in screen space
        let min_x = a.x.min(b.x).min(c.x).max(0.0) as usize;
        let min_y = a.y.min(b.y).min(c.y).max(0.0) as usize;
        let max_x = (a.x.max(b.x).max(c.x).min((self.screen_width - 1) as f32)) as usize;
        let max_y = (a.y.max(b.y).max(c.y).min((self.screen_height - 1) as f32)) as usize;

        // // Triangle Setup
        let yab = a.y - b.y;
        let ybc = b.y - c.y;
        let yca = c.y - a.y;

        let xba = b.x - a.x;
        let xcb = c.x - b.x;
        let xac = a.x - c.x;

        let top_left = Vec2::new(min_x as f32, min_y as f32);
        let mut wa_row = edge_function(b.xy(), c.xy(), top_left);
        let mut wb_row = edge_function(c.xy(), a.xy(), top_left);
        let mut wc_row = edge_function(a.xy(), b.xy(), top_left);

        // Iterate over each pixel in the bounding box
        for y in min_y..=max_y {
            // Barycentric coordinates at start of row
            let mut wa = wa_row;
            let mut wb = wb_row;
            let mut wc = wc_row;

            for x in min_x..=max_x {
                // If the pixel is inside the triangle (barycentric coordinates are non-negative)
                //if wa >= 0.0 && wb >= 0.0 && wc >= 0.0 {
                if fast_check_weights_inside(wa, wb, wc) {
                    self.render_pixel(
                        x,
                        y,
                        RenderVertex::new(a, wa),
                        RenderVertex::new(b, wb),
                        RenderVertex::new(c, wc),
                    );
                }

                // Increment weights one step to the right
                wa += ybc;
                wb += yca;
                wc += yab;
            }

            // Increment weights one step down
            wa_row += xcb;
            wb_row += xac;
            wc_row += xba;
        }
    }

    fn render_pixel(
        &mut self,
        x: usize,
        y: usize,
        a: RenderVertex,
        b: RenderVertex,
        c: RenderVertex,
    ) {
        // Interpolate attributes (e.g., depth value, texture coordinates) at the current pixel
        let interpolated_depth = 1.0 / (a.depth_weight() + b.depth_weight() + c.depth_weight());

        // Calculate the pixel's index
        let pixel_index = y * self.screen_width + x;

        // Perform depth testing
        if self.z_buffer.test_and_set(pixel_index, interpolated_depth) {
            // Perform fragment shading (e.g., apply lighting calculations, texture mapping)
            let fragment_color = Color::new(255, 255, 255);

            // Write the fragment color to the frame buffer
            gc::set_pixel(
                fragment_color.to_graphics_params(),
                x as i32,
                (self.screen_height - y) as i32,
            );
        }
    }
}

// Checks if the weights are inside of the triangle
// Via: https://fgiesen.wordpress.com/2013/02/10/optimizing-the-basic-rasterizer/
fn fast_check_weights_inside(wa: f32, wb: f32, wc: f32) -> bool {
    use std::mem::transmute;
    unsafe {
        let wa: i32 = transmute(wa);
        let wb: i32 = transmute(wb);
        let wc: i32 = transmute(wc);
        (wa | wb | wc) >= 0
    }
}

fn edge_function(a: Vec2, b: Vec2, c: Vec2) -> f32 {
    (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)
}

struct RenderVertex {
    position: Vec4,
    weight: f32,
}

impl RenderVertex {
    fn new(position: Vec4, weight: f32) -> Self {
        Self { position, weight }
    }

    fn depth_weight(&self) -> f32 {
        self.position.z / self.weight
    }
}
