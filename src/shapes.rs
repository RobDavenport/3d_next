use glam::Vec3;

use crate::{graphics::TriangleIndices, shaders::VertexParameters};

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
pub const SIDE: f32 = 1.0;
pub const CUBE: &[Vec3; 24] = &[
    // Front
    CUBE_SIMPLE[0],
    CUBE_SIMPLE[1],
    CUBE_SIMPLE[2],
    CUBE_SIMPLE[3],
    // Right
    CUBE_SIMPLE[1],
    CUBE_SIMPLE[5],
    CUBE_SIMPLE[3],
    CUBE_SIMPLE[7],
    // Back
    CUBE_SIMPLE[5],
    CUBE_SIMPLE[4],
    CUBE_SIMPLE[7],
    CUBE_SIMPLE[6],
    // Left
    CUBE_SIMPLE[4],
    CUBE_SIMPLE[0],
    CUBE_SIMPLE[6],
    CUBE_SIMPLE[2],
    // Top
    CUBE_SIMPLE[2],
    CUBE_SIMPLE[3],
    CUBE_SIMPLE[6],
    CUBE_SIMPLE[7],
    // Bottom
    CUBE_SIMPLE[4],
    CUBE_SIMPLE[5],
    CUBE_SIMPLE[0],
    CUBE_SIMPLE[1],
];

pub const CUBE_PARAMETERS: &[VertexParameters<8>; 24] = &[
    // UVs, Normals, Tangent
    // Front
    VertexParameters([
        CUBE_SIMPLE_UVS[0].0[0],
        CUBE_SIMPLE_UVS[0].0[1],
        0.0,
        0.0,
        1.0,
        1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[1].0[0],
        CUBE_SIMPLE_UVS[1].0[1],
        0.0,
        0.0,
        1.0,
        1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[2].0[0],
        CUBE_SIMPLE_UVS[2].0[1],
        0.0,
        0.0,
        1.0,
        1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[3].0[0],
        CUBE_SIMPLE_UVS[3].0[1],
        0.0,
        0.0,
        1.0,
        1.0,
        0.0,
        0.0,
    ]),
    // Right
    VertexParameters([
        CUBE_SIMPLE_UVS[1].0[0],
        CUBE_SIMPLE_UVS[0].0[1],
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        -1.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[5].0[0],
        CUBE_SIMPLE_UVS[1].0[1],
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        -1.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[3].0[0],
        CUBE_SIMPLE_UVS[2].0[1],
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        -1.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[7].0[0],
        CUBE_SIMPLE_UVS[3].0[1],
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        -1.0,
    ]),
    // Back
    VertexParameters([
        CUBE_SIMPLE_UVS[5].0[0],
        CUBE_SIMPLE_UVS[0].0[1],
        0.0,
        0.0,
        -1.0,
        -1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[4].0[0],
        CUBE_SIMPLE_UVS[1].0[1],
        0.0,
        0.0,
        -1.0,
        -1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[7].0[0],
        CUBE_SIMPLE_UVS[2].0[1],
        0.0,
        0.0,
        -1.0,
        -1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[6].0[0],
        CUBE_SIMPLE_UVS[3].0[1],
        0.0,
        0.0,
        -1.0,
        -1.0,
        0.0,
        0.0,
    ]),
    // Left
    VertexParameters([
        CUBE_SIMPLE_UVS[4].0[0],
        CUBE_SIMPLE_UVS[0].0[1],
        -1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[0].0[0],
        CUBE_SIMPLE_UVS[1].0[1],
        -1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[6].0[0],
        CUBE_SIMPLE_UVS[2].0[1],
        -1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[2].0[0],
        CUBE_SIMPLE_UVS[3].0[1],
        -1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
    ]),
    // Top
    VertexParameters([
        CUBE_SIMPLE_UVS[2].0[0],
        CUBE_SIMPLE_UVS[0].0[1],
        0.0,
        1.0,
        0.0,
        1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[3].0[0],
        CUBE_SIMPLE_UVS[1].0[1],
        0.0,
        1.0,
        0.0,
        1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[6].0[0],
        CUBE_SIMPLE_UVS[2].0[1],
        0.0,
        1.0,
        0.0,
        1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[7].0[0],
        CUBE_SIMPLE_UVS[3].0[1],
        0.0,
        1.0,
        0.0,
        1.0,
        0.0,
        0.0,
    ]),
    // Bottom
    VertexParameters([
        CUBE_SIMPLE_UVS[4].0[0],
        CUBE_SIMPLE_UVS[0].0[1],
        0.0,
        -1.0,
        0.0,
        1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[5].0[0],
        CUBE_SIMPLE_UVS[1].0[1],
        0.0,
        -1.0,
        0.0,
        1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[0].0[0],
        CUBE_SIMPLE_UVS[2].0[1],
        0.0,
        -1.0,
        0.0,
        1.0,
        0.0,
        0.0,
    ]),
    VertexParameters([
        CUBE_SIMPLE_UVS[1].0[0],
        CUBE_SIMPLE_UVS[3].0[1],
        0.0,
        -1.0,
        0.0,
        1.0,
        0.0,
        0.0,
    ]),
];

pub const CUBE_SIMPLE_INDICES: &[TriangleIndices; 12] = &[
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

pub const CUBE_COLORS: &[VertexParameters<3>] = &[
    VertexParameters([0.0, 0.0, 1.0]),
    VertexParameters([0.0, 1.0, 0.0]),
    VertexParameters([0.0, 1.0, 1.0]),
    VertexParameters([1.0, 0.0, 0.0]),
    VertexParameters([1.0, 0.0, 1.0]),
    VertexParameters([1.0, 1.0, 0.0]),
    VertexParameters([1.0, 1.0, 1.0]),
    VertexParameters([1.0, 0.0, 1.0]),
];

pub const CUBE_SIMPLE_UVS: &[VertexParameters<2>; 8] = &[
    VertexParameters([0.0, 1.0]),
    VertexParameters([1.0, 1.0]),
    VertexParameters([0.0, 0.0]),
    VertexParameters([1.0, 0.0]),
    VertexParameters([1.0, 1.0]),
    VertexParameters([0.0, 1.0]),
    VertexParameters([1.0, 0.0]),
    VertexParameters([0.0, 0.0]),
];

pub const CUBE_SIMPLE: &[Vec3] = &[
    Vec3::new(-SIDE, -SIDE, SIDE),
    Vec3::new(SIDE, -SIDE, SIDE),
    Vec3::new(-SIDE, SIDE, SIDE),
    Vec3::new(SIDE, SIDE, SIDE),
    Vec3::new(-SIDE, -SIDE, -SIDE),
    Vec3::new(SIDE, -SIDE, -SIDE),
    Vec3::new(-SIDE, SIDE, -SIDE),
    Vec3::new(SIDE, SIDE, -SIDE),
];

pub const PLANE: &[Vec3] = &[
    Vec3::new(-SIDE, -SIDE, 0.0), //Bottom Left
    Vec3::new(SIDE, -SIDE, 0.0),  // Bottom Right
    Vec3::new(-SIDE, SIDE, 0.0),  // Top Left
    Vec3::new(SIDE, SIDE, 0.0),   // Top Right
];

pub const PLANE_UVS: &[VertexParameters<2>; 4] = &[
    VertexParameters([0.0, 1.0]), // Bottom Left
    VertexParameters([1.0, 1.0]), // Bottom Right
    VertexParameters([0.0, 0.0]), // Top Left
    VertexParameters([1.0, 0.0]), // Top Right
];

pub const PLANE_INDICES: &[TriangleIndices; 2] =
    &[TriangleIndices(0, 1, 2), TriangleIndices(2, 1, 3)];

pub const TRI_INDICES: &[TriangleIndices; 1] = &[TriangleIndices(0, 1, 2)];
