use rkyv::{Archive, Deserialize, Serialize};

use crate::{vertex_parameters::VertexParametersList, IndexList, VertexList};

#[derive(Serialize, Deserialize, Archive)]
pub struct Mesh<const PARAMETER_COUNT: usize> {
    pub vertices: VertexList,
    pub indices: IndexList,
    pub parameters: VertexParametersList<PARAMETER_COUNT>,
}

pub struct MeshBytes<const PARAMETER_COUNT: usize>(pub &'static [u8]);

impl<const PARAMETER_COUNT: usize> MeshBytes<PARAMETER_COUNT> {
    pub fn as_mesh(&self) -> &ArchivedMesh<PARAMETER_COUNT> {
        unsafe { rkyv::archived_root::<Mesh<PARAMETER_COUNT>>(self.0) }
    }
}
