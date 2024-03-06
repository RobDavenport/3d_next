use glam::Mat4;

use crate::*;

// A collection of bone indices & weights
pub struct SkinOutput {
    name: String,
    entries: Vec<SkinEntryOutput>,
    bone_count: usize,
}

struct SkinEntryOutput {
    bone_indices: Vec<u32>,
    weights: Vec<f32>,
    bone_count: usize,
}

//A collection of bones
struct SkeletonOutput {
    name: String,
    bones: Vec<BoneOutput>,
}

struct BoneOutput {
    children: Vec<usize>,
    local_matrix: Mat4,
    inverse_bind_matrix: Mat4,
}

impl SkeletonOutput {
    pub fn to_output(self) -> String {
        let inverse_bind_matrices = format!("{}_{IBM_EXTENSION}", self.name);

        write_file(
            &inverse_bind_matrices,
            cast_slice(&self.inverse_bind_matrices),
        );
        let name = format!("{}_{SKELETON_EXTENSION}", self.name.to_uppercase());

        format!(
            "
        pub static {name}: &SkeletonData = &SkeletonData {{

        }}
            "
        )
    }
}

impl SkinOutput {
    pub fn to_output(&self) -> String {
        let weights = format!("{}_{WEIGHTS_EXTENSION}", self.name);
        let bones = format!("{}_{BONES_EXTENSION}", self.name);

        write_file(&weights, cast_slice(&self.weights));
        write_file(&bones, cast_slice(&self.bones));

        let name = format!("{}_{SKIN_EXTENSION}", self.name.to_uppercase());
        let b = self.bone_count;

        format!(
            "
    pub static {name}: &SkinData<{b}> = &SkinData {{
        weights: include_bytes_aligned!(4, \"generated/{weights}\"),
        bones: include_bytes_aligned!(4, \"generated/{bones}\"),
    }};\n"
        )
    }
}
