/// Autogenerated file from exporter/main.rs.
/// To regenrate this file, run the exporter again
/// You shouldn't be editing this file.
use shared::{animation::AnimationBytes, texture::TextureBytes, mesh::MeshBytes, skeleton::SkeletonBytes, skin::SkinBytes};

pub mod textures {
    use super::*;
pub static GAMERCADE_TEX: &TextureBytes = &TextureBytes(include_bytes!("gamercade_TEX"));
pub static BRICKWALL_TEX: &TextureBytes = &TextureBytes(include_bytes!("brickwall_TEX"));
pub static BRICKWALL_NORMAL_TEX: &TextureBytes = &TextureBytes(include_bytes!("brickwall_normal_TEX"));
}
pub mod meshes {
    use super::*;
pub static TRIANGLE: &MeshBytes<2> = &MeshBytes(include_bytes!("Triangle_MESH"));
pub static PLANE: &MeshBytes<2> = &MeshBytes(include_bytes!("Plane_MESH"));
pub static CUBE: &MeshBytes<8> = &MeshBytes(include_bytes!("Cube_MESH"));
pub static BOXVERTEXCOLORS: &MeshBytes<6> = &MeshBytes(include_bytes!("BoxVertexColors_MESH"));
pub static FOX_SKL: &SkeletonBytes<24> = &SkeletonBytes(include_bytes!("Fox_SKL"));
pub static FOX_SURVEY_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("Fox_Survey_ANM"));
pub static FOX_WALK_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("Fox_Walk_ANM"));
pub static FOX_RUN_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("Fox_Run_ANM"));
pub static FOX: &MeshBytes<5> = &MeshBytes(include_bytes!("Fox_MESH"));
pub static FOX_SKN: &SkinBytes<4> = &SkinBytes(include_bytes!("Fox_SKN"));
pub static FOX_0_TEX: &TextureBytes = &TextureBytes(include_bytes!("Fox_0_TEX"));
pub static DUCK: &MeshBytes<5> = &MeshBytes(include_bytes!("Duck_MESH"));
pub static DUCK_0_TEX: &TextureBytes = &TextureBytes(include_bytes!("Duck_0_TEX"));
pub static DAMAGEDHELMET: &MeshBytes<5> = &MeshBytes(include_bytes!("DamagedHelmet_MESH"));
pub static DAMAGEDHELMET_0_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_0_TEX"));
pub static DAMAGEDHELMET_1_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_1_TEX"));
pub static DAMAGEDHELMET_2_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_2_TEX"));
pub static DAMAGEDHELMET_3_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_3_TEX"));
pub static DAMAGEDHELMET_4_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_4_TEX"));
pub static RIGGEDSIMPLE_SKL: &SkeletonBytes<2> = &SkeletonBytes(include_bytes!("RiggedSimple_SKL"));
pub static RIGGEDSIMPLE_UNNAMED_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("RiggedSimple_Unnamed_ANM"));
pub static RIGGEDSIMPLE: &MeshBytes<3> = &MeshBytes(include_bytes!("RiggedSimple_MESH"));
pub static RIGGEDSIMPLE_SKN: &SkinBytes<4> = &SkinBytes(include_bytes!("RiggedSimple_SKN"));
pub static RIGGEDFIGURE_SKL: &SkeletonBytes<19> = &SkeletonBytes(include_bytes!("RiggedFigure_SKL"));
pub static RIGGEDFIGURE_UNNAMED_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("RiggedFigure_Unnamed_ANM"));
pub static RIGGEDFIGURE: &MeshBytes<3> = &MeshBytes(include_bytes!("RiggedFigure_MESH"));
pub static RIGGEDFIGURE_SKN: &SkinBytes<4> = &SkinBytes(include_bytes!("RiggedFigure_SKN"));
}
