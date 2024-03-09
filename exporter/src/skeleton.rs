use bytemuck::from_bytes;
use glam::Mat4;
use gltf::Document;
use shared::skeleton::{BoneRaw, SkeletonRaw};

use crate::*;

//A collection of bones
pub struct SkeletonOutput {
    pub name: String,
    pub bones: Vec<BoneRaw>,
}

impl SkeletonOutput {
    pub fn to_output(&self) -> String {
        let filename = format!("{}_{SKELETON_EXTENSION}", self.name);
        let bone_count = self.bones.len();

        // Get the max number of children
        let max_children = self
            .bones
            .iter()
            .map(|bone| bone.children.len())
            .max()
            .unwrap();

        // Pad any vecs which have less than max children
        let bones = self
            .bones
            .iter()
            .map(|bone| {
                // let len_difference = max_children - bone.children.len();
                // bone.children.extend((0..len_difference).map(|_| 0));
                let mut out = vec![0; max_children];

                bone.children
                    .iter()
                    .enumerate()
                    .for_each(|(index, bone)| out[index] = *bone);
                let children = out.into_boxed_slice();

                BoneRaw {
                    children,
                    local_matrix: bone.local_matrix,
                    inverse_bind_matrix: bone.inverse_bind_matrix,
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let out = SkeletonRaw(bones);

        let archive = rkyv::to_bytes::<_, 256>(&out).unwrap();
        write_file(&filename, &archive);
        let name = filename.to_uppercase();

        format!(
            "pub static {name}: &SkeletonRawBytes<{bone_count}, {max_children}> = &SkeletonRawBytes(include_bytes!(\"{filename}\"));\n"
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
                children.push(child.index() as u8);
            }

            let bone = BoneRaw {
                children: children.into(),
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
