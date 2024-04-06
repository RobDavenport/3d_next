/// Autogenerated file from exporter.
/// To regenrate this file, run the exporter again
/// You shouldn't be editing this file.
use shared::bytes::*;

pub mod textures {
    use super::*;
pub static GAMERCADE_TEX: &TextureBytes = &TextureBytes(include_bytes!("gamercade_TEX"));
pub static BRICKWALL_TEX: &TextureBytes = &TextureBytes(include_bytes!("brickwall_TEX"));
pub static BRICKWALL_NORMAL_TEX: &TextureBytes = &TextureBytes(include_bytes!("brickwall_normal_TEX"));
pub static TESTCHARTEXTURE_TEX: &TextureBytes = &TextureBytes(include_bytes!("testCharTexture_TEX"));
pub static ENDESGA32_TEX: &TextureBytes = &TextureBytes(include_bytes!("endesga32_TEX"));
}
pub mod meshes {
    use super::*;
pub static TRIANGLE: &MeshBytes<2> = &MeshBytes(include_bytes!("Triangle_MESH"));
pub static PLANE: &MeshBytes<2> = &MeshBytes(include_bytes!("Plane_MESH"));
pub static CUBE: &MeshBytes<8> = &MeshBytes(include_bytes!("Cube_MESH"));
pub static BOXVERTEXCOLORS: &MeshBytes<6> = &MeshBytes(include_bytes!("BoxVertexColors_MESH"));
pub static FOX: &MeshBytes<5> = &MeshBytes(include_bytes!("Fox_MESH"));
pub static FOX_SKL: &SkeletonBytes<24> = &SkeletonBytes(include_bytes!("Fox_SKL"));
pub static FOX_SURVEY_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("Fox_Survey_ANM"));
pub static FOX_WALK_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("Fox_Walk_ANM"));
pub static FOX_RUN_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("Fox_Run_ANM"));
pub static FOX_SKN: &SkinBytes<4> = &SkinBytes(include_bytes!("Fox_SKN"));
pub static FOX_0_TEX: &TextureBytes = &TextureBytes(include_bytes!("Fox_0_TEX"));
pub static DAMAGEDHELMET: &MeshBytes<5> = &MeshBytes(include_bytes!("DamagedHelmet_MESH"));
pub static DAMAGEDHELMET_0_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_0_TEX"));
pub static DAMAGEDHELMET_1_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_1_TEX"));
pub static DAMAGEDHELMET_2_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_2_TEX"));
pub static DAMAGEDHELMET_3_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_3_TEX"));
pub static DAMAGEDHELMET_4_TEX: &TextureBytes = &TextureBytes(include_bytes!("DamagedHelmet_4_TEX"));
pub static CHARTEST: &MeshBytes<5> = &MeshBytes(include_bytes!("charTest_MESH"));
pub static CHARTEST_SKL: &SkeletonBytes<33> = &SkeletonBytes(include_bytes!("charTest_SKL"));
pub static CHARTEST_IDLE_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("charTest_idle_ANM"));
pub static CHARTEST_SKN: &SkinBytes<4> = &SkinBytes(include_bytes!("charTest_SKN"));
pub static VSBACKGROUND: &MeshBytes<5> = &MeshBytes(include_bytes!("vsBackground_MESH"));
pub static VSBACKGROUND_0_TEX: &TextureBytes = &TextureBytes(include_bytes!("vsBackground_0_TEX"));
pub static MECH: &MeshBytes<5> = &MeshBytes(include_bytes!("mech_MESH"));
pub static MECH_SKL: &SkeletonBytes<19> = &SkeletonBytes(include_bytes!("mech_SKL"));
pub static MECH__DEFAULTPOSE_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("mech__DefaultPose_ANM"));
pub static MECH_IDLE_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("mech_idle_ANM"));
pub static MECH_IDLEPOSE_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("mech_idlePose_ANM"));
pub static MECH_SQUAT_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("mech_Squat_ANM"));
pub static MECH_SKN: &SkinBytes<4> = &SkinBytes(include_bytes!("mech_SKN"));
pub static MULTIMESH: &MeshBytes<5> = &MeshBytes(include_bytes!("multimesh_MESH"));
pub static MULTIMESH_SKL: &SkeletonBytes<2> = &SkeletonBytes(include_bytes!("multimesh_SKL"));
pub static MULTIMESH_ARMATUREACTION_ANM: &AnimationBytes = &AnimationBytes(include_bytes!("multimesh_ArmatureAction_ANM"));
pub static MULTIMESH_SKN: &SkinBytes<4> = &SkinBytes(include_bytes!("multimesh_SKN"));
}
