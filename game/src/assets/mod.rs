mod generated;
pub use generated::*;

use crate::{
    animation::{Skeleton, Skin},
    //graphics::{IndexList, StaticMesh, VertexList, VertexParametersList},
};

use bytemuck::cast_slice;

// pub struct StaticMeshData<const P: usize> {
//     pub vertices: &'static [u8],
//     pub indices: &'static [u8],
//     pub parameters: &'static [u8],
// }

// impl<const P: usize> StaticMeshData<P> {
//     pub fn as_mesh(&self) -> StaticMesh<P> {
//         let vertices = VertexList(cast_slice(self.vertices));
//         let indices = IndexList(cast_slice(self.indices));
//         let parameters = VertexParametersList(cast_slice(self.parameters));

//         StaticMesh {
//             vertices,
//             indices,
//             parameters,
//         }
//     }
// }

pub struct SkeletonData<const BONE_COUNT: usize, const MAX_CHILDREN: usize> {
    pub children: &'static [u8],
    pub matrices: &'static [u8],
}

impl<const BONE_COUNT: usize, const MAX_CHILDREN: usize> SkeletonData<BONE_COUNT, MAX_CHILDREN> {
    pub fn as_skeleton(&self) -> Skeleton<BONE_COUNT, MAX_CHILDREN> {
        Skeleton {
            matrices: cast_slice(self.matrices),
            children: cast_slice(self.children),
        }
    }
}

pub struct SkinData<const MAX_BONE_COUNT: usize>(pub &'static [u8]);

impl<const MAX_BONE_COUNT: usize> SkinData<MAX_BONE_COUNT> {
    pub fn as_skin(&self) -> Skin<MAX_BONE_COUNT> {
        Skin(cast_slice(self.0))
    }
}
