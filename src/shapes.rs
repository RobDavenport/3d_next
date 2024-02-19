use glam::{Vec2, Vec3};

use crate::graphics::TriangleIndices;

// RH Coordinate System
// +X is Right
// +Y is Up
// +Z is Out of the screen (towards viewer)
//    6--------7
//   /|       /|
//  / |      / |
// 2--|-----3  |
// |  |     |  |
// |  4-----|--5
// | /      | /
// |/       |/
// 0--------1

pub const CUBE_INDICES: &[TriangleIndices; 12] = &[
    TriangleIndices(0, 1, 2), // Front
    TriangleIndices(2, 1, 3), // Front
    TriangleIndices(1, 5, 3), // Right
    TriangleIndices(3, 5, 7), // Right
    TriangleIndices(2, 3, 6), // Top
    TriangleIndices(3, 7, 6), // Top
    TriangleIndices(4, 7, 5), // Back
    TriangleIndices(4, 6, 7), // Back
    TriangleIndices(0, 2, 4), // Left
    TriangleIndices(2, 6, 4), // Left
    TriangleIndices(0, 4, 1), // Bottom
    TriangleIndices(1, 4, 5), // Bottom
];

pub const CUBE_COLORS: &[Vec3; 8] = &[
    Vec3::new(0.0, 0.0, 1.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.0, 1.0, 1.0),
    Vec3::new(1.0, 0.0, 0.0),
    Vec3::new(1.0, 0.0, 1.0),
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(1.0, 1.0, 1.0),
    Vec3::new(1.0, 0.0, 1.0),
];

pub const CUBE_UVS: &[Vec2; 8] = &[
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 1.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
];

pub fn cube(side: f32) -> [Vec3; 8] {
    [
        Vec3::new(-side, -side, -side),
        Vec3::new(side, -side, -side),
        Vec3::new(-side, side, -side),
        Vec3::new(side, side, -side),
        Vec3::new(-side, -side, side),
        Vec3::new(side, -side, side),
        Vec3::new(-side, side, side),
        Vec3::new(side, side, side),
    ]
}

pub fn plane(side: f32) -> [Vec3; 4] {
    [
        Vec3::new(-side, -side, 0.0), //Bottom Left
        Vec3::new(side, -side, 0.0),  // Bottom Right
        Vec3::new(-side, side, 0.0),  // Top Left
        Vec3::new(side, side, 0.0),   // Top Right
    ]
}

pub const PLANE_UVS: &[Vec2; 4] = &[
    Vec2::new(0.0, 0.0), // Bottom Left
    Vec2::new(1.0, 0.0), // Bottom Right
    Vec2::new(0.0, 1.0), // Top Left
    Vec2::new(1.0, 1.0), // Top Right
];

pub const PLANE_INDICES: [TriangleIndices; 1] = [TriangleIndices(0, 1, 2)]; //, TriangleIndices(2, 1, 3)];
