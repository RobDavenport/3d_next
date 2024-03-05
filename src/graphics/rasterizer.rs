use glam::{Vec2, Vec3, Vec3Swizzles, Vec4Swizzles};
use wide::{f32x4, i32x4, CmpGt, CmpLt};

use crate::shaders::{PixelShader, VertexParameters, VertexParametersSimd};

use super::{render_tile::RenderTile, Triangle, Uniforms};

// TODO: Consider using a 2x2 tiled approach
pub(super) const X_STEP_SIZE: usize = 4;
pub(super) const Y_STEP_SIZE: usize = 1;

const X_STAMP_OFFSETS: [i32; 4] = [0, 1, 2, 3];
const Y_STAMP_OFFSETS: [i32; 4] = [0, 0, 0, 0];

pub struct EdgeStepperCombined {
    one_over_triangle_2a: f32,

    a_edge: EdgeStepper,
    b_edge: EdgeStepper,
    c_edge: EdgeStepper,

    wa_row: f32x4,
    wb_row: f32x4,
    wc_row: f32x4,

    wa: f32x4,
    wb: f32x4,
    wc: f32x4,
}

impl EdgeStepperCombined {
    pub fn new<const P: usize>(
        triangle: &RenderTriangle<P>,
        top_left: Vec2,
        initial_x_stamp: &[i32; 4],
        initial_y_stamp: &[i32; 4],
        x_step: i32,
        y_step: i32,
    ) -> Self {
        // Local Setup
        let a = triangle.a.xy();
        let b = triangle.b;
        let c = triangle.c;

        // Initialize steppers
        let one_over_triangle_2a = double_triangle_area(a, b, c).recip();
        let (a_edge, wa_row) = EdgeStepper::initialize(
            b,
            c,
            top_left,
            initial_x_stamp,
            initial_y_stamp,
            x_step,
            y_step,
        );
        let (b_edge, wb_row) = EdgeStepper::initialize(
            c,
            a,
            top_left,
            initial_x_stamp,
            initial_y_stamp,
            x_step,
            y_step,
        );
        let (c_edge, wc_row) = EdgeStepper::initialize(
            a,
            b,
            top_left,
            initial_x_stamp,
            initial_y_stamp,
            x_step,
            y_step,
        );

        Self {
            one_over_triangle_2a,
            a_edge,
            b_edge,
            c_edge,
            wa_row,
            wb_row,
            wc_row,
            wa: wa_row,
            wb: wa_row,
            wc: wa_row,
        }
    }

    pub fn step_x(&mut self) {
        // Increment weights one step to the right
        self.wa += self.a_edge.step_x;
        self.wb += self.b_edge.step_x;
        self.wc += self.c_edge.step_x;
    }

    pub fn reset_row(&mut self) {
        self.wa = self.wa_row;
        self.wb = self.wb_row;
        self.wc = self.wc_row;
    }

    pub fn step_y(&mut self) {
        // Increment weights one step down
        self.wa_row += self.a_edge.step_y;
        self.wb_row += self.b_edge.step_y;
        self.wc_row += self.c_edge.step_y;
    }

    pub fn points_inside_triangle_mask(&self) -> f32x4 {
        // If the pixel is inside the triangle (barycentric coordinates are non-negative)
        let wa_mask = self.wa.cmp_gt(f32x4::ZERO);
        let wb_mask = self.wb.cmp_gt(f32x4::ZERO);
        let wc_mask = self.wc.cmp_gt(f32x4::ZERO);
        wa_mask & wb_mask & wc_mask
    }
}

impl<const W: usize, const H: usize> RenderTile<W, H> {
    pub(super) fn trivial_rasterize_triangle<PS, const PSIN: usize>(
        &mut self,
        uniforms: &Uniforms,
        mut triangle: RenderTriangle<PSIN>,
        ps: PS,
    ) where
        PS: PixelShader<PSIN>,
    {
        // Triangle Setup
        let top_left = Vec2::new(self.x as f32, self.y as f32);
        let mut stepper = EdgeStepperCombined::new(
            &triangle,
            top_left,
            &X_STAMP_OFFSETS,
            &Y_STAMP_OFFSETS,
            X_STEP_SIZE as i32,
            Y_STEP_SIZE as i32,
        );

        // Iterate over each pixel in the bounding box
        for y in (0..H).step_by(Y_STEP_SIZE) {
            // Reset to start of row.
            stepper.reset_row();
            for x in (0..W).step_by(X_STEP_SIZE) {
                // Normalize the weights
                triangle.b_sub_a.weight = stepper.wb * stepper.one_over_triangle_2a;
                triangle.c_sub_a.weight = stepper.wc * stepper.one_over_triangle_2a;

                // See if any pixels extend out of the bb
                // and update mask accordingly
                let pixel_indices = i32x4::splat(x as i32) + i32x4::new(X_STAMP_OFFSETS);
                let bb_valid_mask = pixel_indices.cmp_lt(i32x4::splat(W as i32));
                let mask = bytemuck::cast::<_, f32x4>(bb_valid_mask);

                self.render_pixels(ps, uniforms, x, y, &triangle, mask);

                // One step right
                stepper.step_x();
            }
            // One step down
            stepper.step_y();
        }
    }

    pub(super) fn rasterize_triangle<PS, const PSIN: usize>(
        &mut self,
        uniforms: &Uniforms,
        mut triangle: RenderTriangle<PSIN>,
        ps: PS,
    ) where
        PS: PixelShader<PSIN>,
    {
        // Determine the bounding box of the triangle in tile space
        let min_x = triangle.min_x.max(self.x as f32) as usize;
        let min_y = triangle.min_y.max(self.y as f32) as usize;
        let max_x = triangle.max_x.min((self.x + W - 1) as f32) as usize;
        let max_y = triangle.max_y.min((self.y + H - 1) as f32) as usize;

        // Triangle Setup
        let top_left = Vec2::new(min_x as f32, min_y as f32);
        let mut stepper = EdgeStepperCombined::new(
            &triangle,
            top_left,
            &X_STAMP_OFFSETS,
            &Y_STAMP_OFFSETS,
            X_STEP_SIZE as i32,
            Y_STEP_SIZE as i32,
        );

        // Iterate over each pixel in the bounding box
        for y in (min_y..=max_y).step_by(Y_STEP_SIZE) {
            stepper.reset_row();
            for x in (min_x..=max_x).step_by(X_STEP_SIZE) {
                // If the pixel is inside the triangle (barycentric coordinates are non-negative)
                let mask = stepper.points_inside_triangle_mask();

                if mask.any() {
                    // Normalize the weights
                    // a's weight is skipped
                    triangle.b_sub_a.weight = stepper.wb * stepper.one_over_triangle_2a;
                    triangle.c_sub_a.weight = stepper.wc * stepper.one_over_triangle_2a;

                    // See if any pixels extend out of the bb
                    // and update mask accordingly
                    let pixel_indices = i32x4::splat(x as i32) + i32x4::new(X_STAMP_OFFSETS);
                    let bb_valid_mask = pixel_indices.cmp_lt(i32x4::splat(max_x as i32 + 1));
                    let mask = mask & bytemuck::cast::<_, f32x4>(bb_valid_mask);

                    self.render_pixels(ps, uniforms, x - self.x, y - self.y, &triangle, mask);
                }

                // One step right
                stepper.step_x();
            }

            // One step down
            // Also resets the X's to start of row values
            stepper.step_y()
        }
    }

    fn render_pixels<PS, const PSIN: usize>(
        &mut self,
        _ps: PS,
        uniforms: &Uniforms,
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
        let pixel_index = (y * W) + x;

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
                    let x = x as i32 + X_STAMP_OFFSETS[bit];
                    let y = y as i32 + Y_STAMP_OFFSETS[bit];

                    // Pun the pixel shader
                    let params = ps_params.extract(bit);
                    let fragment_color = PS::run(uniforms, params);

                    // Write the fragment color to the frame buffer
                    self.frame_buffer[x as usize + (y as usize * W)] =
                        fragment_color.to_graphics_params();
                }
            }
        }
    }
}

#[derive(Clone)]
pub(super) struct RenderTriangle<const P: usize> {
    pub(super) a: Vec3,
    pub(super) b: Vec2,
    pub(super) c: Vec2,
    a_params: VertexParametersSimd<P>,
    b_sub_a: RenderVertex<P>,
    c_sub_a: RenderVertex<P>,
    pub(super) min_x: f32,
    pub(super) max_x: f32,
    pub(super) min_y: f32,
    pub(super) max_y: f32,
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

        let a_params = triangle.parameters[0] * a.w;
        let b_params = (triangle.parameters[1] * b.w) - a_params;
        let c_params = (triangle.parameters[2] * c.w) - a_params;

        Self {
            a: a.xyw(),
            b: b.xy(),
            c: c.xy(),
            a_params: a_params.splat(),
            b_sub_a: RenderVertex::new(b.w - a.w, b_params),
            c_sub_a: RenderVertex::new(c.w - a.w, c_params),
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}

#[derive(Clone)]
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
    fn initialize(
        v0: Vec2,
        v1: Vec2,
        origin: Vec2,
        initial_x_stamp: &[i32; 4],
        initial_y_stamp: &[i32; 4],
        x_step: i32,
        y_step: i32,
    ) -> (Self, f32x4) {
        // Edge setup
        let a = v0.y - v1.y;
        let b = v1.x - v0.x;
        let c = v0.x * v1.y - v0.y * v1.x;

        // Step Deltas
        let step_x = f32x4::splat(a * x_step as f32);
        let step_y = f32x4::splat(b * y_step as f32);

        let out = Self { step_x, step_y };

        // x/y values for initial pixel block
        let x: f32x4 = f32x4::splat(origin.x) + i32x4::new(*initial_x_stamp).round_float();
        let y: f32x4 = f32x4::splat(origin.y) + i32x4::new(*initial_y_stamp).round_float();

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
