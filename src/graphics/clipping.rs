use glam::Vec4;

use crate::camera::NEAR_PLANE;

use super::Triangle;

pub(super) enum ClipResult<const P: usize> {
    Culled,
    One(Triangle<P>),
    Two((Triangle<P>, Triangle<P>)),
}

fn is_in_front_of_near_plane(vertex: &Vec4) -> bool {
    vertex.w < vertex.z
}

pub(super) fn clip_triangle<const P: usize>(triangle: Triangle<P>) -> ClipResult<P> {
    // Geometric Clipping against near plane
    // We are using a reversed Z
    let a_front = is_in_front_of_near_plane(&triangle.positions[0]);
    let b_front = is_in_front_of_near_plane(&triangle.positions[1]);
    let c_front = is_in_front_of_near_plane(&triangle.positions[2]);

    match (a_front, b_front, c_front) {
        // Simple Cases
        (false, false, false) => ClipResult::Culled, // All behind the view, can discard the triangle
        (true, true, true) => ClipResult::One(triangle), // All in front, just pass through

        // Handle One In Front
        (true, false, false) => ClipResult::One(clip_triangle_one_front(triangle, 0)),
        (false, true, false) => ClipResult::One(clip_triangle_one_front(triangle, 1)),
        (false, false, true) => ClipResult::One(clip_triangle_one_front(triangle, 2)),

        // Handle One Behind
        (false, true, true) => ClipResult::Two(clip_triangle_one_behind(triangle, 0)),
        (true, false, true) => ClipResult::Two(clip_triangle_one_behind(triangle, 1)),
        (true, true, false) => ClipResult::Two(clip_triangle_one_behind(triangle, 2)),
    }
}

fn clip_triangle_one_front<const P: usize>(
    triangle: Triangle<P>,
    front_index: usize,
) -> Triangle<P> {
    clip_edges_against_near_plane(triangle, front_index)
}

fn clip_edges_against_near_plane<const P: usize>(
    mut triangle: Triangle<P>,
    origin: usize,
) -> Triangle<P> {
    // Setup
    // Calculate indices for B and C
    let b_index = (origin + 1) % 3;
    let c_index = (origin + 2) % 3;

    // Extract vertices and parameters
    let a = triangle.positions[origin];
    let b = triangle.positions[b_index];
    let c = triangle.positions[c_index];
    let a_params = triangle.parameters[origin];
    let b_params = triangle.parameters[b_index];
    let c_params = triangle.parameters[c_index];
    // End Setup

    // Begin Clipping
    // Get the distance of A from the near plane
    let aw_clipped = a.w - NEAR_PLANE;

    // Vectors from A->B and A->C
    let ab = (b.w - NEAR_PLANE) - aw_clipped;
    let ac = (c.w - NEAR_PLANE) - aw_clipped;

    // Find how much to lerp
    // Negate aw_clipped as we use reversed depth
    let ab_factor = -aw_clipped / ab;
    let ac_factor = -aw_clipped / ac;

    // Lerp all the things
    let b_clipped = a.lerp(b, ab_factor);
    let c_clipped = a.lerp(c, ac_factor);
    let b_clipped_params = a_params.lerp(b_params, ab_factor);
    let c_clipped_params = a_params.lerp(c_params, ac_factor);

    // Set their outs
    triangle.positions[b_index] = b_clipped;
    triangle.positions[c_index] = c_clipped;
    triangle.parameters[b_index] = b_clipped_params;
    triangle.parameters[c_index] = c_clipped_params;

    triangle
}

fn clip_triangle_one_behind<const P: usize>(
    triangle: Triangle<P>,
    back_index: usize,
) -> (Triangle<P>, Triangle<P>) {
    // Index Setup - o0 isn't used as it's the vertex being split
    let o1 = (back_index + 1) % 3;
    let o2 = (back_index + 2) % 3;

    // Clip the triangle
    let behind_triangle = clip_edges_against_near_plane(triangle.clone(), back_index);

    // Verts: o2, b2, b1
    let first = Triangle {
        positions: [
            triangle.positions[o2],
            behind_triangle.positions[o2],
            behind_triangle.positions[o1],
        ],
        parameters: [
            triangle.parameters[o2],
            behind_triangle.parameters[o2],
            behind_triangle.parameters[o1],
        ],
    };

    // Verts: o2, b1, o1
    let second = Triangle {
        positions: [
            triangle.positions[o2],
            behind_triangle.positions[o1],
            triangle.positions[o1],
        ],
        parameters: [
            triangle.parameters[o2],
            behind_triangle.parameters[o1],
            triangle.parameters[o1],
        ],
    };

    (first, second)
}
