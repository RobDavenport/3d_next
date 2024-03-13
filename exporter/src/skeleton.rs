use std::array;
use std::collections::HashMap;

use bytemuck::cast_slice;
use glam::{Mat4, Vec4};
use gltf::Document;
use rkyv::AlignedVec;
use seq_macro::seq;
use shared::skeleton::{Bone, BoneTrs, Skeleton};
use shared::SKELETON_MAX_BONES;

use crate::skin::get_bone_name_index_maps;
use crate::*;

pub struct BoneVec {
    pub children: Vec<i8>,
    pub local_matrix: BoneTrs,
    pub inverse_bind_matrix: Mat4,
}

pub struct SkeletonMetaData {
    pub bone_count: u8,
    pub named_bones: HashMap<String, i8>,
    pub node_to_index: HashMap<usize, i8>,
    pub root_transform: Mat4,
}

// A collection of bones
pub struct SkeletonOutput {
    pub name: String,
    pub bones: Vec<Bone>,
}

impl SkeletonOutput {
    fn resize(bones: Vec<Bone>) -> AlignedVec {
        let bone_count = bones.len();

        seq!(BC in 0..128 {
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
    blob: &[u8],
) -> Option<(SkeletonMetaData, String)> {
    let skin_count = document.skins().count();
    if skin_count > 1 {
        println!("Skin count > 1 {skin_count}. May not be exported correctly...")
    }

    if let Some(skin) = document.skins().next() {
        let mut bones = Vec::new();

        let (named_joints, indexed_joints) = get_bone_name_index_maps(&skin);

        let ibm_accessor = skin.inverse_bind_matrices().unwrap();
        let ibm_view = ibm_accessor.view().unwrap();

        let ibm_start = ibm_accessor.offset() + ibm_view.offset();
        let ibm_end = ibm_start + (ibm_accessor.count() * ibm_accessor.size());

        let bytes = &blob[ibm_start..ibm_end];
        let bytes: &[f32] = cast_slice(bytes);

        let mut ibms = Vec::new();

        for mat in bytes.chunks_exact(ibm_accessor.dimensions().multiplicity()) {
            ibms.push(Mat4::from_cols_slice(mat))
        }

        for (index, bone) in skin.joints().enumerate() {
            let mut children = Vec::new();
            for child in bone.children() {
                let child_index = named_joints
                    .get(child.name().unwrap())
                    .expect("Bone name not found");
                children.push(*child_index);
            }

            let (translation, rotation, scale) = bone.transform().decomposed();
            let local_matrix = BoneTrs {
                translation: Vec3::from_slice(&translation),
                rotation: Vec4::from_slice(&rotation),
                scale: Vec3::from_slice(&scale),
            };

            let inverse_bind_matrix = ibms[index];

            let bone = BoneVec {
                children,
                local_matrix,
                inverse_bind_matrix,
            };
            bones.push(bone);
        }

        let len = bones.len();

        if len != ibms.len() {
            panic!("ibm.len() != len")
        };

        println!("Found a skeleton with {len} bones.");

        // Invert the Bone -> [Children] Relationship
        let mut inverted_bones = Vec::new();

        struct WorkingBone {
            parent: i8,
            local_matrix: BoneTrs,
            inverse_bind_matrix: Mat4,
        }

        bones.iter().for_each(|bone| {
            inverted_bones.push(WorkingBone {
                parent: -1,
                local_matrix: bone.local_matrix,
                inverse_bind_matrix: bone.inverse_bind_matrix,
            })
        });

        // Set the Parents
        bones.iter().enumerate().for_each(|(parent_index, bone)| {
            let parent_index = parent_index as i8;
            bone.children.iter().for_each(|child| {
                // Check if the bone already has a parent:
                let prev_parent = inverted_bones[*child as usize].parent;

                if prev_parent.is_positive() && prev_parent != parent_index {
                    panic!("Bone has multiple parents, which isn't supported");
                }
                inverted_bones[*child as usize].parent = parent_index
            });
        });

        inverted_bones
            .iter()
            .enumerate()
            .for_each(|(bone_index, bone)| {
                if bone.parent > bone_index as i8 {
                    panic!("Bone parent > bone index!")
                }
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
            named_bones: named_joints,
            node_to_index: indexed_joints,
            root_transform: Mat4::IDENTITY,
        };
        Some((metadata, skeleton.to_output()))
    } else {
        None
    }
}
