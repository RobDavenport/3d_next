use glam::{Vec2, Vec3, Vec3Swizzles, Vec4Swizzles};
use wide::{f32x4, i32x4, CmpGt, CmpLt};

use crate::shaders::{PixelShader, VertexParameters, VertexParametersSimd};

use super::{Gpu, Triangle};

// TODO: Consider using a 2x2 tiled approach
pub(super) const X_STEP_SIZE: usize = 4;
pub(super) const Y_STEP_SIZE: usize = 1;

const X_OFFSETS: [i32; 4] = [0, 1, 2, 3];
const Y_OFFSETS: [i32; 4] = [0, 0, 0, 0];

impl Gpu {
    // TODO: Consider a better traversal algorithm (Zig Zag)
    pub(super) fn rasterize_triangle<PS, const PSIN: usize>(
        &mut self,
        mut triangle: RenderTriangle<PSIN>,
        ps: PS,
    ) where
        PS: PixelShader<PSIN>,
    {
        let a = triangle.a.xy();
        let b = triangle.b;
        let c = triangle.c;

        // Determine the bounding box of the triangle in screen space
        let min_x = triangle.min_x.max(0.0) as usize;
        let min_y = triangle.min_y.max(0.0) as usize;
        let max_x = triangle.max_x.min((self.screen_width - 1) as f32) as usize;
        let max_y = triangle.max_y.min((self.screen_height - 1) as f32) as usize;

        // Triangle Setup
        let one_over_triangle_2a = double_triangle_area(a, b, c).recip();
        let top_left = Vec2::new(min_x as f32, min_y as f32);
        let (a_edge, mut wa_row) = EdgeStepper::initialize(b, c, top_left);
        let (b_edge, mut wb_row) = EdgeStepper::initialize(c, a, top_left);
        let (c_edge, mut wc_row) = EdgeStepper::initialize(a, b, top_left);

        // Iterate over each pixel in the bounding box
        for y in (min_y..=max_y).step_by(Y_STEP_SIZE) {
            // Barycentric coordinates at start of row
            let mut wa = wa_row;
            let mut wb = wb_row;
            let mut wc = wc_row;

            for x in (min_x..=max_x).step_by(X_STEP_SIZE) {
                // If the pixel is inside the triangle (barycentric coordinates are non-negative)
                let zero = f32x4::ZERO;
                let wa_mask = wa.cmp_gt(zero);
                let wb_mask = wb.cmp_gt(zero);
                let wc_mask = wc.cmp_gt(zero);
                let mask = wa_mask & wb_mask & wc_mask;

                if mask.any() {
                    // Normalize the weights
                    // a's weight is skipped
                    triangle.b_sub_a.weight = wb * one_over_triangle_2a;
                    triangle.c_sub_a.weight = wc * one_over_triangle_2a;

                    // See if any pixels extend out of the bb
                    // and update mask accordingly
                    let pixel_indices = i32x4::splat(x as i32) + i32x4::new(X_OFFSETS);
                    let bb_valid_mask = pixel_indices.cmp_lt(i32x4::splat(max_x as i32 + 1));
                    let mask = mask & bytemuck::cast::<_, f32x4>(bb_valid_mask);

                    self.render_pixels(ps, x, y, &triangle, mask);
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
        _ps: PS,
        x: usize,
        y: usize,
        triangle: &RenderTriangle<PSIN>,
        mask: f32x4,
    ) where
        PS: PixelShader<PSIN>,
    {
        let RenderTriangle {
            a,
            a_params,
            b_sub_a,
            c_sub_a,
            ..
        } = triangle;

        // Interpolate depth for depth testing, using simplified formula
        let interpolated_depths =
            (a.z) + (b_sub_a.z * b_sub_a.weight) + (c_sub_a.z * c_sub_a.weight);

        // Calculate the pixel's index
        let pixel_index = y * self.screen_width + x;

        // Perform depth testing
        let mask = self
            .z_buffer
            .test_and_set(pixel_index, interpolated_depths, mask);

        // Continue if any pass the depth test
        if mask > 0 {
            // Sum the Parameres to complete interpolation, using simplified formula
            let ps_params = (a_params.clone()
                + (b_sub_a.parameters.clone() * b_sub_a.weight)
                + (c_sub_a.parameters.clone() * c_sub_a.weight))
                * interpolated_depths.recip();

            for bit in 0..4 {
                if (mask & 1 << bit) != 0 {
                    let x = x as i32 + X_OFFSETS[bit];
                    let y = y as i32 + Y_OFFSETS[bit];

                    // Pun the pixel shader
                    let params = ps_params.extract(bit);
                    let fragment_color = PS::run(&self.uniforms, params);

                    // Write the fragment color to the frame buffer
                    let y = (self.screen_height - y as usize) - 1;
                    self.frame_buffer[x as usize + (y * self.screen_width)] =
                        fragment_color.to_graphics_params();
                }
            }
        }
    }
}

pub(crate) struct RenderTriangle<const P: usize> {
    a: Vec3,
    b: Vec2,
    c: Vec2,
    a_params: VertexParametersSimd<P>,
    b_sub_a: RenderVertex<P>,
    c_sub_a: RenderVertex<P>,
    pub(crate) min_x: f32,
    pub(crate) max_x: f32,
    pub(crate) min_y: f32,
    pub(crate) max_y: f32,
}

impl<const P: usize> RenderTriangle<P> {
    pub(crate) fn setup(triangle: Triangle<P>) -> Self {
        let a = triangle.positions[0];
        let b = triangle.positions[1];
        let c = triangle.positions[2];

        let min_x = a.x.min(b.x).min(c.x);
        let max_x = a.x.max(b.x).max(c.x);

        let min_y = a.y.min(b.y).min(c.y);
        let max_y = a.y.max(b.y).max(c.y);

        let a_params = triangle.parameters[0] * a.z;
        let b_params = (triangle.parameters[1] * b.z) - a_params;
        let c_params = (triangle.parameters[2] * c.z) - a_params;

        Self {
            a: a.xyz(),
            b: b.xy(),
            c: c.xy(),
            a_params: a_params.splat(),
            b_sub_a: RenderVertex::new(b.z - a.z, b_params),
            c_sub_a: RenderVertex::new(c.z - a.z, c_params),
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}

struct RenderVertex<const P: usize> {
    z: f32,
    weight: f32x4,
    parameters: VertexParametersSimd<P>,
}

impl<const P: usize> RenderVertex<P> {
    fn new(z: f32, parameters: VertexParameters<P>) -> Self {
        Self {
            z,
            weight: f32x4::default(),
            parameters: parameters.splat(),
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
