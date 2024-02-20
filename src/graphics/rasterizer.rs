use gamercade_rs::prelude as gc;
use glam::{Vec2, Vec4, Vec4Swizzles};
use wide::{f32x4, i32x4, CmpGt};

use crate::shaders::{PixelShader, PixelShaderInput};

use super::{Gpu, Triangle};

// TODO: Consider using a 2x2 tiled approach
pub(super) const X_STEP_SIZE: usize = 4;
pub(super) const Y_STEP_SIZE: usize = 1;

const X_OFFSETS: [i32; 4] = [0, 1, 2, 3];
const Y_OFFSETS: [i32; 4] = [0, 0, 0, 0];

impl Gpu {
    // TODO: Consider using Fixed Point math
    // TODO: Incorporate a better boundingbox traversal algorithm
    pub(super) fn rasterize_triangle<PS, PSIN>(&mut self, triangle: Triangle<PSIN>)
    where
        PS: PixelShader<PSIN>,
        PSIN: PixelShaderInput,
    {
        let a = triangle.positions[0];
        let b = triangle.positions[1];
        let c = triangle.positions[2];

        // Determine the bounding box of the triangle in screen space
        let min_x = a.x.min(b.x).min(c.x).max(0.0) as usize;
        let min_y = a.y.min(b.y).min(c.y).max(0.0) as usize;
        let max_x = (a.x.max(b.x).max(c.x).min((self.screen_width - 1) as f32)) as usize;
        let max_y = (a.y.max(b.y).max(c.y).min((self.screen_height - 1) as f32)) as usize;

        // Triangle Setup
        let triangle_area = triangle_area(a.xy(), b.xy(), c.xy());
        let top_left = Vec2::new(min_x as f32, min_y as f32);
        let (a_edge, mut wa_row) = EdgeStepper::initialize(b.xy(), c.xy(), top_left);
        let (b_edge, mut wb_row) = EdgeStepper::initialize(c.xy(), a.xy(), top_left);
        let (c_edge, mut wc_row) = EdgeStepper::initialize(a.xy(), b.xy(), top_left);

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
                    // Normalize the weights
                    let wa = wa / triangle_area;
                    let wb = wb / triangle_area;
                    let wc = wc / triangle_area;

                    self.render_pixels::<PS, PSIN>(
                        x,
                        y,
                        RenderVertex::new(a, wa, triangle.parameters[0]),
                        RenderVertex::new(b, wb, triangle.parameters[1]),
                        RenderVertex::new(c, wc, triangle.parameters[2]),
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

    fn render_pixels<PS, PSIN>(
        &mut self,
        x: usize,
        y: usize,
        a: RenderVertex<PSIN>,
        b: RenderVertex<PSIN>,
        c: RenderVertex<PSIN>,
        mask: f32x4,
    ) where
        PS: PixelShader<PSIN>,
        PSIN: PixelShaderInput,
    {
        // Interpolate depth values for the pixels
        let interpolated_depths = 1.0 / (a.depth_weight() + b.depth_weight() + c.depth_weight());

        // Calculate the pixel's index
        let pixel_index = y * self.screen_width + x;

        // Perform depth testing
        let mask = self
            .z_buffer
            .test_and_set(pixel_index, interpolated_depths, mask);

        if mask > 0 {
            let weights_a = a.weight.as_array_ref();
            let weights_b = b.weight.as_array_ref();
            let weights_c = c.weight.as_array_ref();

            for bit in 0..4 {
                if (mask & 1 << bit) != 0 {
                    let x = x as i32 + X_OFFSETS[bit];
                    let y = y as i32 + Y_OFFSETS[bit];
                    // Interpolate attributes for rendering
                    let a = a.parameters * weights_a[bit];
                    let b = b.parameters * weights_b[bit];
                    let c = c.parameters * weights_c[bit];
                    let ps_params = a + b + c;

                    // Perform fragment shading (e.g., apply lighting calculations, texture mapping)
                    let fragment_color = PS::run(ps_params);

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

struct RenderVertex<P> {
    position: Vec4,
    weight: f32x4,
    parameters: P,
}

impl<P> RenderVertex<P> {
    fn new(position: Vec4, weight: f32x4, parameters: P) -> Self {
        Self {
            position,
            weight,
            parameters,
        }
    }

    fn depth_weight(&self) -> f32x4 {
        self.position.z * self.weight
    }
}

struct EdgeStepper {
    step_x: f32x4,
    step_y: f32x4,
}

impl EdgeStepper {
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
        let x: f32x4 = f32x4::splat(origin.x) + i32x4::from(X_OFFSETS).round_float();
        let y: f32x4 = f32x4::splat(origin.y) + i32x4::from(Y_OFFSETS).round_float();

        // Edge function weights at origin
        let weight = f32x4::splat(a) * x + f32x4::splat(b) * y + f32x4::splat(c);
        (out, weight)
    }
}

fn triangle_area(v0: Vec2, v1: Vec2, v2: Vec2) -> f32 {
    let area = (v1.x - v0.x) * (v2.y - v0.y) - (v1.y - v0.y) * (v2.x - v0.x);
    area / 2.0
}
