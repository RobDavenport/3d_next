use rkyv::{Archive, Deserialize, Serialize};

use crate::{vertex_parameters::VertexParametersList, IndexList, VertexList};

#[derive(Serialize, Deserialize, Archive)]
pub struct Mesh<const P: usize> {
    pub vertices: VertexList,
    pub indices: IndexList,
    pub parameters: VertexParametersList<P>,
}

pub struct MeshBytes<const P: usize>(pub &'static [u8]);

impl<const P: usize> MeshBytes<P> {
    pub fn as_mesh(&self) -> &ArchivedMesh<P> {
        unsafe { rkyv::archived_root::<Mesh<P>>(self.0) }
    }
}
