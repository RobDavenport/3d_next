/// Autogenerated file from exporter/main.rs.
/// To regenrate this file, run the exporter again
/// You shouldn't be editing this file.
use crate::assets::{SkinData, SkeletonData};
use shared::{texture::TextureBytes, mesh::MeshRawBytes};

pub mod textures {
    use super::*;
pub static GAMERCADE_TEX: &TextureBytes = &TextureBytes(include_bytes!("gamercade_TEX"));
pub static BRICKWALL_TEX: &TextureBytes = &TextureBytes(include_bytes!("brickwall_TEX"));
pub static BRICKWALL_NORMAL_TEX: &TextureBytes = &TextureBytes(include_bytes!("brickwall_normal_TEX"));
}
pub mod meshes {
    use super::*;
pub static TRIANGLE: &MeshRawBytes<2> = &MeshRawBytes(include_bytes!("Triangle_MESH"));
pub static PLANE: &MeshRawBytes<2> = &MeshRawBytes(include_bytes!("Plane_MESH"));
pub static CUBE: &MeshRawBytes<8> = &MeshRawBytes(include_bytes!("Cube_MESH"));
pub static BOXVERTEXCOLORS: &MeshRawBytes<6> = &MeshRawBytes(include_bytes!("BoxVertexColors_MESH"));

    pub static FOX_SKL: &SkeletonData<24, 4> = &SkeletonData {
        matrices: include_bytes_aligned!(4, "generated/Fox_SKL"),
        children: include_bytes_aligned!(4, "generated/Fox_CHI")
    };pub static FOX: &MeshRawBytes<5> = &MeshRawBytes(include_bytes!("Fox_MESH"));

    pub static FOX_SKN: &SkinData<4> = &SkinData(include_bytes_aligned!(4, "generated/Fox_SKN"));
pub static FOX_0_TEX: &TextureBytes = &TextureBytes(include_bytes!("Fox_0_TEX"));
pub static DUCK: &MeshRawBytes<5> = &MeshRawBytes(include_bytes!("Duck_MESH"));
pub static DUCK_0_TEX: &TextureBytes = &TextureBytes(include_bytes!("Duck_0_TEX"));
pub static DAMAGEDHELMET: &MeshRawBytes<5> = &MeshRawBytes(include_bytes!("DamagedHelmet_MESH"));
pub static DAMAGEDHELMET_0_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_0_TEX"));
pub static DAMAGEDHELMET_1_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_1_TEX"));
pub static DAMAGEDHELMET_2_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_2_TEX"));
pub static DAMAGEDHELMET_3_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_3_TEX"));
pub static DAMAGEDHELMET_4_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_4_TEX"));
}
