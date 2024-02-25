use glam::Vec3;

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

pub const CUBE_INDICES: [TriangleIndices; 12] = [
    // Front
    TriangleIndices(0, 1, 2),
    TriangleIndices(2, 1, 3),
    // Right
    TriangleIndices(4, 1 + 4, 2 + 4),
    TriangleIndices(2 + 4, 1 + 4, 3 + 4),
    // Back
    TriangleIndices(8, 1 + 8, 2 + 8),
    TriangleIndices(2 + 8, 1 + 8, 3 + 8),
    // Left
    TriangleIndices(12, 1 + 12, 2 + 12),
    TriangleIndices(2 + 12, 1 + 12, 3 + 12),
    // Top
    TriangleIndices(16, 1 + 16, 2 + 16),
    TriangleIndices(2 + 16, 1 + 16, 3 + 16),
    // Bottom
    TriangleIndices(20, 1 + 20, 2 + 20),
    TriangleIndices(2 + 20, 1 + 20, 3 + 20),
];

// Position, UV, Normals
pub fn cube(side: f32) -> [(Vec3, [f32; 2], [f32; 3], [f32; 3]); 24] {
    let c = cube_simple(side);
    let uvs = CUBE_SIMPLE_UVS;
    // Position, UVs, Normals, Tangent
    [
        // Front
        (c[0], uvs[0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        (c[1], uvs[1], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        (c[2], uvs[2], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        (c[3], uvs[3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
        // Right
        (c[1], uvs[0], [1.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
        (c[5], uvs[1], [1.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
        (c[3], uvs[2], [1.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
        (c[7], uvs[3], [1.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
        // Back
        (c[5], uvs[0], [0.0, 0.0, -1.0], [-1.0, 0.0, 0.0]),
        (c[4], uvs[1], [0.0, 0.0, -1.0], [-1.0, 0.0, 0.0]),
        (c[7], uvs[2], [0.0, 0.0, -1.0], [-1.0, 0.0, 0.0]),
        (c[6], uvs[3], [0.0, 0.0, -1.0], [-1.0, 0.0, 0.0]),
        // Left
        (c[4], uvs[0], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        (c[0], uvs[1], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        (c[6], uvs[2], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        (c[2], uvs[3], [-1.0, 0.0, 0.0], [0.0, 0.0, 1.0]),
        // Top
        (c[2], uvs[0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
        (c[3], uvs[1], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
        (c[6], uvs[2], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
        (c[7], uvs[3], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
        // Bottom
        (c[4], uvs[0], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0]),
        (c[5], uvs[1], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0]),
        (c[0], uvs[2], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0]),
        (c[1], uvs[3], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0]),
    ]
}

pub const CUBE_SIMPLE_INDICES: [TriangleIndices; 12] = [
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

pub const CUBE_COLORS: &[[f32; 3]; 8] = &[
    [0.0, 0.0, 1.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0],
    [1.0, 0.0, 0.0],
    [1.0, 0.0, 1.0],
    [1.0, 1.0, 0.0],
    [1.0, 1.0, 1.0],
    [1.0, 0.0, 1.0],
];

pub const CUBE_SIMPLE_UVS: &[[f32; 2]; 8] = &[
    [0.0, 1.0],
    [1.0, 1.0],
    [0.0, 0.0],
    [1.0, 0.0],
    [1.0, 1.0],
    [0.0, 1.0],
    [1.0, 0.0],
    [0.0, 0.0],
];

pub fn cube_normals() -> [[f32; 3]; 8] {
    [
        Vec3::new(-1.0, -1.0, 1.0).normalize().into(),
        Vec3::new(1.0, -1.0, 1.0).normalize().into(),
        Vec3::new(-1.0, 1.0, 1.0).normalize().into(),
        Vec3::new(1.0, 1.0, 1.0).normalize().into(),
        Vec3::new(-1.0, -1.0, -1.0).normalize().into(),
        Vec3::new(1.0, -1.0, -1.0).normalize().into(),
        Vec3::new(-1.0, 1.0, -1.0).normalize().into(),
        Vec3::new(1.0, 1.0, -1.0).normalize().into(),
    ]
}

pub fn cube_simple(side: f32) -> [Vec3; 8] {
    [
        Vec3::new(-side, -side, side),
        Vec3::new(side, -side, side),
        Vec3::new(-side, side, side),
        Vec3::new(side, side, side),
        Vec3::new(-side, -side, -side),
        Vec3::new(side, -side, -side),
        Vec3::new(-side, side, -side),
        Vec3::new(side, side, -side),
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

pub const PLANE_UVS: &[[f32; 2]; 4] = &[
    [0.0, 1.0], // Bottom Left
    [1.0, 1.0], // Bottom Right
    [0.0, 0.0], // Top Left
    [1.0, 0.0], // Top Right
];

pub const PLANE_INDICES: [TriangleIndices; 2] =
    [TriangleIndices(0, 1, 2), TriangleIndices(2, 1, 3)];

pub const TRI_INDICES: [TriangleIndices; 1] = [TriangleIndices(0, 1, 2)];
