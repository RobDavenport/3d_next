use gamercade_rs::{api::text::console_log, prelude as gc};
use glam::Vec4;

use crate::{camera::NEAR_PLANE, shaders::PixelShaderInput};

use super::Triangle;

pub(super) enum ClipResult<P> {
    Culled,
    One(Triangle<P>),
    Two((Triangle<P>, Triangle<P>)),
}

fn is_in_front_of_near_plane(vertex: &Vec4) -> bool {
    vertex.z >= vertex.w
}

pub(super) fn clip_triangle<P: PixelShaderInput>(triangle: Triangle<P>) -> ClipResult<P> {
    // Geometric Clipping against near plane
    // We are using a reversed Z
    let a_front = is_in_front_of_near_plane(&triangle.positions[0]);
    let b_front = is_in_front_of_near_plane(&triangle.positions[1]);
    let c_front = is_in_front_of_near_plane(&triangle.positions[2]);
    console_log(&format!("b: {}", triangle.positions[1]));
    match (a_front, b_front, c_front) {
        // Simple Cases
        (false, false, false) => return ClipResult::Culled, // All behind the view, can discard the triangle
        (true, true, true) => return ClipResult::One(triangle), // All in front, just pass through

        // Handle One In Front
        (true, false, false) => ClipResult::One(clip_triangle_one_front(triangle, 0)),
        (false, true, false) => ClipResult::One(clip_triangle_one_front(triangle, 1)),
        (false, false, true) => ClipResult::One(clip_triangle_one_front(triangle, 2)),

        _ => return ClipResult::Culled,
        // // Handle One Behind
        // (false, true, true) => ClipResult::Two(clip_triangle_one_behind(triangle, 0)),
        // (true, false, true) => ClipResult::Two(clip_triangle_one_behind(triangle, 1)),
        // (true, true, false) => ClipResult::Two(clip_triangle_one_behind(triangle, 2)),
    }
}

fn clip_triangle_one_front<P: PixelShaderInput>(
    mut triangle: Triangle<P>,
    front_index: usize,
) -> Triangle<P> {
    let b_index = (front_index + 1) % 3;
    let c_index = (front_index + 2) % 3;
    let a = triangle.positions[front_index];
    let b = triangle.positions[b_index];
    let c = triangle.positions[c_index];

    gc::console_log("------------");

    gc::console_log(&format!("front: {}", triangle.positions[front_index]));
    gc::console_log(&format!("vertb: {}", triangle.positions[b_index]));
    // gc::console_log(&format!("vertc: {}", triangle.positions[c_index]));

    // TODO: Finish this clipping stuff

    // Vectors from A->B and A->C
    let ab = b.w - a.w;
    let ac = c.w - a.w;

    gc::console_log(&format!("ab: {ab}"));

    // Find how much to lerp
    let ab_factor = a.w / ab;
    let ac_factor = a.w / ac;

    gc::console_log(&format!("ab factor: {ab_factor}"));

    // Lerp all the things
    let b_clipped = a.lerp(b, ab_factor);
    let c_clipped = a.lerp(c, ac_factor);

    // Set the out values
    triangle.positions[b_index] = b_clipped;
    triangle.positions[c_index] = c_clipped;

    gc::console_log(&format!("Clipped b: {}", triangle.positions[b_index]));
    // gc::console_log(&format!("Clipped c: {}", triangle.positions[c_index]));

    triangle
}

fn clip_triangle_one_behind<P>(
    triangle: Triangle<P>,
    behind_index: usize,
) -> (Triangle<P>, Triangle<P>) {
    let a = triangle.positions[behind_index];
    let b = triangle.positions[(behind_index + 1) % 3];
    let c = triangle.positions[(behind_index + 2) % 3];

    todo!()
}
