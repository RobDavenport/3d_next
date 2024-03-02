use arrayvec::ArrayVec;
use glam::Vec4;

use crate::camera::NEAR_PLANE;

use super::{gpu::CLIPPING_MAX_OUTPUT, Gpu, Triangle};

#[derive(Clone, Copy)]
pub(crate) enum ClippingPlane {
    Near,
    Left,
    Right,
    Top,
    Bottom,
}

impl ClippingPlane {
    fn first() -> Option<Self> {
        Some(Self::Near)
    }

    fn next(self) -> Option<ClippingPlane> {
        match self {
            Self::Near => Some(Self::Left),
            Self::Left => Some(Self::Right),
            Self::Right => Some(Self::Top),
            Self::Top => Some(Self::Bottom),
            Self::Bottom => None,
        }
    }

    pub(crate) fn point_front_of_plane(&self, vertex: &Vec4) -> bool {
        match self {
            Self::Near => vertex.z > vertex.w,
            Self::Left => vertex.x < -vertex.w,
            Self::Right => vertex.x > vertex.w,
            Self::Top => vertex.y < -vertex.w,
            Self::Bottom => vertex.y > vertex.w,
        }
    }
}

pub(super) enum ClipResult<const P: usize> {
    Culled,
    One(Triangle<P>),
    Two((Triangle<P>, Triangle<P>)),
}

pub(super) fn clip_triangle<const P: usize>(
    plane: ClippingPlane,
    triangle: Triangle<P>,
) -> ClipResult<P> {
    // Geometric Clipping against frustum
    let a_front = plane.point_front_of_plane(&triangle.positions[0]);
    let b_front = plane.point_front_of_plane(&triangle.positions[1]);
    let c_front = plane.point_front_of_plane(&triangle.positions[2]);

    match (a_front, b_front, c_front) {
        // Simple Cases
        (false, false, false) => ClipResult::Culled, // All behind the view, can discard the triangle
        (true, true, true) => ClipResult::One(triangle), // All in front, just pass through

        // Handle One In Front
        (true, false, false) => ClipResult::One(clip_triangle_one_front(plane, triangle, 0)),
        (false, true, false) => ClipResult::One(clip_triangle_one_front(plane, triangle, 1)),
        (false, false, true) => ClipResult::One(clip_triangle_one_front(plane, triangle, 2)),

        // Handle One Behind
        (false, true, true) => ClipResult::Two(clip_triangle_one_behind(plane, triangle, 0)),
        (true, false, true) => ClipResult::Two(clip_triangle_one_behind(plane, triangle, 1)),
        (true, true, false) => ClipResult::Two(clip_triangle_one_behind(plane, triangle, 2)),
    }
}

fn clip_triangle_one_front<const P: usize>(
    plane: ClippingPlane,
    triangle: Triangle<P>,
    front_index: usize,
) -> Triangle<P> {
    clip_edges_against_plane(plane, triangle, front_index)
}

fn get_clip_factors<const P: usize>(
    plane: ClippingPlane,
    triangle: &Triangle<P>,
    origin: usize,
) -> (f32, f32) {
    let b_index = (origin + 1) % 3;
    let c_index = (origin + 2) % 3;

    let a = triangle.positions[origin];
    let b = triangle.positions[b_index];
    let c = triangle.positions[c_index];

    let (a_clipped, ab, ac) = match plane {
        ClippingPlane::Near => {
            // Get the distance of A from the near plane
            let aw_clipped = a.w - NEAR_PLANE;

            // Vectors from A->B and A->C
            let ab = a.w - b.w;
            let ac = a.w - c.w;

            (aw_clipped, ab, ac)
        }
        ClippingPlane::Left => {
            let ax_clipped = a.x + a.w;

            let ab = ax_clipped - (b.x + b.w);
            let ac = ax_clipped - (c.x + c.w);

            (ax_clipped, ab, ac)
        }
        ClippingPlane::Right => {
            let ax_clipped = a.x - a.w;

            let ab = ax_clipped - (b.x - b.w);
            let ac = ax_clipped - (c.x - c.w);

            (ax_clipped, ab, ac)
        }
        ClippingPlane::Top => {
            let ay_clipped = a.y + a.w;

            let ab = ay_clipped - (b.y + b.w);
            let ac = ay_clipped - (c.y + c.w);

            (ay_clipped, ab, ac)
        }
        ClippingPlane::Bottom => {
            let ay_clipped = a.y - a.w;

            let ab = ay_clipped - (b.y - b.w);
            let ac = ay_clipped - (c.y - c.w);

            (ay_clipped, ab, ac)
        }
    };

    // Find how much to lerp
    let ab_factor = a_clipped / ab;
    let ac_factor = a_clipped / ac;
    (ab_factor, ac_factor)
}

fn clip_edges_against_plane<const P: usize>(
    plane: ClippingPlane,
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
    let (ab_factor, ac_factor) = get_clip_factors(plane, &triangle, origin);

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
    plane: ClippingPlane,
    triangle: Triangle<P>,
    back_index: usize,
) -> (Triangle<P>, Triangle<P>) {
    // Index Setup - o0 isn't used as it's the vertex being split
    let o1 = (back_index + 1) % 3;
    let o2 = (back_index + 2) % 3;

    // Clip the triangle
    let behind_triangle = clip_edges_against_plane(plane, triangle.clone(), back_index);

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

impl Gpu {
    pub(crate) fn clip_stage<const P: usize>(
        &mut self,
        triangle: Triangle<P>,
    ) -> ArrayVec<Triangle<P>, CLIPPING_MAX_OUTPUT> {
        // Clip triangles, and whatever remains, rasterize them
        let mut plane_iter = ClippingPlane::first();
        let mut input;
        let mut output = ArrayVec::<_, 13>::new();

        output.push(triangle);
        while let Some(clip) = plane_iter {
            // Take the previous output and run it through clipping
            input = output;
            output = ArrayVec::new();

            // Clip all of the triangles against the plane
            input.drain(..).for_each(|triangle| {
                let clip_result = clip_triangle(clip, triangle);
                match clip_result {
                    ClipResult::Culled => (),
                    ClipResult::One(triangle) => output.push(triangle),
                    ClipResult::Two((first, second)) => {
                        output.push(first);
                        output.push(second)
                    }
                }
            });

            // Step to the next plane
            plane_iter = clip.next();
        }

        output
    }
}
