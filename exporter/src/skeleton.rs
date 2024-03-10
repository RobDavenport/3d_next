use std::array;
use std::collections::HashMap;

use glam::{Mat4, Quat, Vec4};
use gltf::Document;
use rkyv::AlignedVec;
use seq_macro::seq;
use shared::skeleton::{Bone, Skeleton};
use shared::SKELETON_MAX_BONES;

use crate::skin::get_bone_name_index_map;
use crate::*;

pub struct BoneVec {
    pub children: Vec<u8>,
    pub local_matrix: Mat4,
    pub inverse_bind_matrix: Mat4,
}

pub struct SkeletonMetaData {
    pub bone_count: u8,
    pub named_bones: HashMap<String, u8>,
}

//A collection of bones
pub struct SkeletonOutput {
    pub name: String,
    pub bones: Vec<Bone>,
}

impl SkeletonOutput {
    fn resize(bones: Vec<Bone>) -> AlignedVec {
        let bone_count = bones.len();

        seq!(BC in 0..64 {
            match bone_count {
                #(BC => rkyv::to_bytes::<_, 256>(&Skeleton::<BC>(array::from_fn(|i| bones[i].clone()))).unwrap(),)*
                too_many_bones => panic!("Too many bones: {too_many_bones}, max is {SKELETON_MAX_BONES}"),
            }
        })
    }

    pub fn to_output(&self) -> String {
        let filename = format!("{}_{SKELETON_EXTENSION}", self.name);
        let bone_count = self.bones.len();

        let archive = Self::resize(self.bones.clone());
        write_file(&filename, &archive);
        let name = filename.to_uppercase();

        format!(
            "pub static {name}: &SkeletonBytes<{bone_count}> = &SkeletonBytes(include_bytes!(\"{filename}\"));\n"
        )
    }
}

pub fn generate_skeleton(
    filename: &str,
    document: &Document,
) -> Option<(SkeletonMetaData, String)> {
    if let Some(skin) = document.skins().next() {
        let mut bones = Vec::new();

        let joints = get_bone_name_index_map(&skin);
        for bone in skin.joints() {
            let mut children = Vec::new();
            for child in bone.children() {
                let child_index = joints.get(child.name().unwrap()).unwrap();
                children.push(*child_index);
            }

            let (translation, rotation, scale) = bone.transform().decomposed();
            let translation = Vec3::from_slice(&translation);
            let rotation = Quat::from_vec4(Vec4::from_slice(&rotation));
            let scale = Vec3::from_slice(&scale);

            let local_matrix = Mat4::from_scale_rotation_translation(scale, rotation, translation);

            let bone = BoneVec {
                children: children.into(),
                local_matrix,
                inverse_bind_matrix: local_matrix.inverse(),
            };
            bones.push(bone);
        }

        let len = bones.len();

        println!("Found a skeleton with {len} bones.");

        // Invert the Bone -> [Children] Relationship
        let mut inverted_bones = Vec::new();

        struct WorkingBone {
            parent: u8,
            local_matrix: Mat4,
            inverse_bind_matrix: Mat4,
        }

        // Set the Children
        bones.iter().for_each(|bone| {
            inverted_bones.push(WorkingBone {
                parent: 0,
                local_matrix: bone.local_matrix,
                inverse_bind_matrix: bone.inverse_bind_matrix,
            })
        });

        // Set the Parents
        bones.iter().enumerate().for_each(|(parent_index, bone)| {
            bone.children.iter().for_each(|child| {
                // Check if the bone already has a parent:
                let prev_parent = inverted_bones[*child as usize].parent;
                if prev_parent != 0 && prev_parent as usize != parent_index {
                    panic!("Bone has multiple parents, which isn't supported");
                }
                inverted_bones[*child as usize].parent = parent_index as u8
            });
        });

        // Populate the output
        let bones = inverted_bones
            .into_iter()
            .map(|bone| Bone {
                parent_index: bone.parent,
                local_matrix: bone.local_matrix,
                inverse_bind_matrix: bone.inverse_bind_matrix,
            })
            .collect::<Vec<_>>();

        let skeleton = SkeletonOutput {
            name: filename.to_string(),
            bones,
        };

        let metadata = SkeletonMetaData {
            bone_count: len as u8,
            named_bones: joints,
        };
        Some((metadata, skeleton.to_output()))
    } else {
        None
    }
}
