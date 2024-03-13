use shared::shapes;

use crate::meshes::MeshOutput;

pub fn cube() -> MeshOutput {
    MeshOutput {
        name: "Cube".to_string(),
        vertices: shapes::CUBE.to_vec(),
        indices: shapes::CUBE_INDICES.to_vec(),
        parameters: shapes::CUBE_PARAMETERS
            .iter()
            .flat_map(|p| Vec::from_iter(p.0))
            .collect(),
        attribute_count: 8,
    }
}

pub fn triangle() -> MeshOutput {
    MeshOutput {
        name: "Triangle".to_string(),
        vertices: shapes::TRIANGLE.to_vec(),
        indices: shapes::TRI_INDICES.to_vec(),
        parameters: shapes::PLANE_UVS
            .iter()
            .flat_map(|p| Vec::from_iter(p.0))
            .collect(),
        attribute_count: 2,
    }
}

pub fn plane() -> MeshOutput {
    MeshOutput {
        name: "Plane".to_string(),
        vertices: shapes::PLANE.to_vec(),
        indices: shapes::PLANE_INDICES.to_vec(),
        parameters: shapes::PLANE_UVS
            .iter()
            .flat_map(|p| Vec::from_iter(p.0))
            .collect(),
        attribute_count: 2,
    }
}
