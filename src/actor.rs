use ultraviolet::{Mat4, Vec3};

use crate::graphics::MeshIndex;

pub struct Actor {
    pub mesh_id: MeshIndex<Vec3>,
    pub transform: Mat4,
}
