use bytemuck::{bytes_of, cast_slice, from_bytes};
use glam::Mat4;
use gltf::Document;

use crate::*;

//A collection of bones
pub struct SkeletonOutput {
    pub name: String,
    pub bones: Vec<BoneOutput>,
}

#[derive(Clone)]
pub struct BoneOutput {
    pub children: Vec<u32>,
    pub local_matrix: Mat4,
    pub inverse_bind_matrix: Mat4,
}

impl BoneOutput {
    fn into_bytes(mut self, max_children: usize) -> Vec<u8> {
        let len_difference = max_children - self.children.len();
        self.children.extend((0..len_difference).map(|_| 0));

        let mut output = cast_slice(&self.children).to_vec();
        output.extend(bytes_of(&self.local_matrix));
        output.extend(bytes_of(&self.inverse_bind_matrix));

        output
    }
}

impl SkeletonOutput {
    pub fn to_output(&self) -> String {
        let bones = format!("{}_{SKELETON_EXTENSION}", self.name);
        let bone_count = self.bones.len();
        let children = format!("{}_{CHILDREN_EXTENSION}", self.name);
        let max_children = self
            .bones
            .iter()
            .map(|bone| bone.children.len())
            .max()
            .unwrap();

        let chidren_vec = self
            .bones
            .iter()
            .flat_map(|bone| bone.children.clone())
            .collect::<Vec<_>>();

        let bones_bytes = self
            .bones
            .iter()
            .flat_map(|bone| bone.clone().into_bytes(max_children))
            .collect::<Vec<_>>();

        write_file(&bones, &bones_bytes);
        write_file(&children, cast_slice(chidren_vec.as_slice()));

        let name = format!("{}_{SKELETON_EXTENSION}", self.name.to_uppercase());

        format!(
            "
    pub static {name}: &SkeletonData<{bone_count}, {max_children}> = &SkeletonData {{
        matrices: include_bytes_aligned!(4, \"generated/{bones}\"),
        children: include_bytes_aligned!(4, \"generated/{children}\")
    }};"
        )
    }
}

pub fn generate_skeleton(
    filename: &str,
    buffer: &[u8],
    document: &Document,
) -> Option<(usize, String)> {
    if let Some(skin) = document.skins().next() {
        let mut ibms = Vec::new();
        let mut bones = Vec::new();

        let ibm = skin.inverse_bind_matrices().unwrap();

        let view = ibm.view().unwrap();
        let data = &buffer[view.offset()..view.offset() + view.length()];

        for matrix in data.chunks_exact(size_of::<Mat4>()) {
            let matrix: &Mat4 = from_bytes(matrix);
            ibms.push(matrix);
        }

        for (index, bone) in skin.joints().enumerate() {
            let mut children = Vec::new();
            for child in bone.children() {
                children.push(child.index() as u32);
            }

            let bone = BoneOutput {
                children,
                local_matrix: Mat4::from_cols_array_2d(&bone.transform().matrix()),
                inverse_bind_matrix: *ibms[index],
            };
            bones.push(bone);
        }

        let len = bones.len();

        println!("Found a skeleton with {len} bones.");

        if bones.len() != ibms.len() {
            panic!(
                "Matrices not equal: Bones: {}, ibms: {}",
                bones.len(),
                ibms.len()
            )
        }

        let skeleton = SkeletonOutput {
            name: filename.to_string(),
            bones,
        };
        Some((len, skeleton.to_output()))
    } else {
        None
    }
}

// A collection of bone indices & weights
pub struct SkinOutput {
    pub name: String,
    pub entries: Vec<SkinEntryOutput>,
}

#[derive(Clone)]
pub struct SkinEntryOutput {
    pub bone_indices: Vec<u32>,
    pub weights: Vec<f32>,
}

impl SkinEntryOutput {
    fn into_bytes(mut self, max_weight_count: usize) -> Vec<u8> {
        let len_difference = max_weight_count - self.bone_indices.len();
        self.bone_indices.extend((0..len_difference).map(|_| 0));

        let mut output = cast_slice(&self.bone_indices).to_vec();
        output.extend(cast_slice(&self.weights));
        output
    }
}

impl SkinOutput {
    pub fn to_output(&self) -> String {
        let skin = format!("{}_{SKIN_EXTENSION}", self.name);

        let max_weight_count = self.entries.iter().map(|e| e.weights.len()).max().unwrap();

        let skin_bytes = self
            .entries
            .iter()
            .flat_map(|bone| bone.clone().into_bytes(max_weight_count))
            .collect::<Vec<_>>();

        write_file(&skin, &skin_bytes);
        let name = format!("{}_{SKIN_EXTENSION}", self.name.to_uppercase());

        format!(
            "
    pub static {name}: &SkinData<{max_weight_count}> = &SkinData(include_bytes_aligned!(4, \"generated/{skin}\"));\n"
        )
    }
}
