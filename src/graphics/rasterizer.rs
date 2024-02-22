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
    // TODO: Incorporate a better boundingbox traversal algorithm
    pub(super) fn rasterize_triangle<PS, const PSIN: usize>(
        &mut self,
        pixel_shader: &PS,
        triangle: Triangle<PSIN>,
    ) where
        PS: PixelShader<PSIN>,
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
        let double_triangle_area = double_triangle_area(a.xy(), b.xy(), c.xy());
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
                    let wa = wa / double_triangle_area;
                    let wb = wb / double_triangle_area;
                    let wc = wc / double_triangle_area;

                    self.render_pixels(
                        pixel_shader,
                        x,
                        y,
                        RenderTriangle {
                            vertices: [
                                RenderVertex::new(a, wa, triangle.parameters[0]),
                                RenderVertex::new(b, wb, triangle.parameters[1]),
                                RenderVertex::new(c, wc, triangle.parameters[2]),
                            ],
                        },
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

    fn render_pixels<PS, const PSIN: usize>(
        &mut self,
        pixel_shader: &PS,
        x: usize,
        y: usize,
        triangle: RenderTriangle<PSIN>,
        mask: f32x4,
    ) where
        PS: PixelShader<PSIN>,
    {
        let [a, b, c] = triangle.vertices;

        // Interpolate depth values for the pixels
        let interpolated_depths = ((a.z * a.weight) + (b.z * b.weight) + (c.z * c.weight)).recip();

        // Calculate the pixel's index
        let pixel_index = y * self.screen_width + x;

        // Perform depth testing
        let mask = self
            .z_buffer
            .test_and_set(pixel_index, interpolated_depths, mask);

        // Continue if any pass the depth test
        if mask > 0 {
            let weights_a = a.weight.as_array_ref();
            let weights_b = b.weight.as_array_ref();
            let weights_c = c.weight.as_array_ref();

            for bit in 0..4 {
                if (mask & 1 << bit) != 0 {
                    let x = x as i32 + X_OFFSETS[bit];
                    let y = y as i32 + Y_OFFSETS[bit];

                    // Interpolate attributes for rendering, perspective correct
                    let a_weight = a.z * weights_a[bit];
                    let b_weight = b.z * weights_b[bit];
                    let c_weight = c.z * weights_c[bit];

                    // Calculate the reciprocal of the sum of weights
                    let weight_recip = (a_weight + b_weight + c_weight).recip();

                    // Sum the Parameres to complete interpolation
                    let ps_params = (a.parameters * a_weight
                        + b.parameters * b_weight
                        + c.parameters * c_weight)
                        * weight_recip;

                    // Perform fragment shading (e.g., apply lighting calculations, texture mapping)
                    let fragment_color = pixel_shader.run(ps_params.0);

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

struct RenderTriangle<const P: usize> {
    vertices: [RenderVertex<P>; 3],
}

struct RenderVertex<const P: usize> {
    z: f32,
    weight: f32x4,
    parameters: PixelShaderInput<P>,
}

impl<const P: usize> RenderVertex<P> {
    fn new(position: Vec4, weight: f32x4, parameters: PixelShaderInput<P>) -> Self {
        Self {
            z: position.z,
            weight,
            parameters,
        }
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

fn double_triangle_area(v0: Vec2, v1: Vec2, v2: Vec2) -> f32 {
    (v1.x - v0.x) * (v2.y - v0.y) - (v1.y - v0.y) * (v2.x - v0.x)
}

// TODO: Fill Rules
// fn is_top_or_left_edge(v0: Vec2, v1: Vec2) -> bool {
//     is_top_edge(v0, v1) | is_left_edge(v0, v1)
// }

// fn is_top_edge(v0: Vec2, v1: Vec2) -> bool {
//     // Check if the edge is horizontal
//     let horizontal = v0.y == v1.y;

//     // Check if the edge is going left (negative X axis)
//     let going_left = v1.x < v0.x;

//     horizontal & going_left
// }

// fn is_left_edge(v0: Vec2, v1: Vec2) -> bool {
//     // Check if the edge is going down
//     v1.y < v0.y
// }
