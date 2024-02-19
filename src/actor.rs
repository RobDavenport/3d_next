use glam::Mat4;

use crate::graphics::MeshIndex;

pub struct Actor<PSIN> {
    pub mesh_id: MeshIndex<PSIN>,
    pub transform: Mat4,
}
