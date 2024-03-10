use std::array;
use std::collections::HashMap;

use bytemuck::from_bytes;
use glam::Mat4;
use gltf::Document;
use rkyv::AlignedVec;
use seq_macro::seq;
use shared::skeleton::{Bone, Skeleton};
use shared::{SKELETON_MAX_BONES, SKELETON_MAX_CHILDREN};

use crate::*;
pub struct BoneVec {
    pub children: Vec<u8>,
    pub local_matrix: Mat4,
    pub inverse_bind_matrix: Mat4,
}

impl BoneVec {
    pub fn to_generic<const MAX_CHILDREN: usize>(&self) -> Bone<MAX_CHILDREN> {
        Bone {
            children: array::from_fn(|i| *self.children.get(i).unwrap_or(&0)),
            local_matrix: self.local_matrix,
            inverse_bind_matrix: self.inverse_bind_matrix,
        }
    }
}

//A collection of bones
pub struct SkeletonOutput {
    pub name: String,
    pub bones: Vec<BoneVec>,
}

impl SkeletonOutput {
    fn resize_bones<const MC: usize>(bones: Vec<Bone<MC>>) -> AlignedVec {
        let bone_count = bones.len();

        seq!(BC in 0..64 {
            match bone_count {
                #(BC => rkyv::to_bytes::<_, 256>(&Skeleton::<BC, MC>(array::from_fn(|i| bones[i].clone()))).unwrap(),)*
                too_many_bones => panic!("Too many bones: {too_many_bones}, max is {SKELETON_MAX_BONES}"),
            }
        })
    }

    fn resize(&self, max_children: usize) -> AlignedVec {
        match max_children {
            // This could be a seq! macro, but compile times are already quite long
            c if c > SKELETON_MAX_CHILDREN => {
                panic!("Too many children: {c}, max is {SKELETON_MAX_CHILDREN}")
            }
            0 => Self::resize_bones(self.bones.iter().map(|b| b.to_generic::<0>()).collect()),
            1 => Self::resize_bones(self.bones.iter().map(|b| b.to_generic::<1>()).collect()),
            2 => Self::resize_bones(self.bones.iter().map(|b| b.to_generic::<2>()).collect()),
            3 => Self::resize_bones(self.bones.iter().map(|b| b.to_generic::<3>()).collect()),
            4 => Self::resize_bones(self.bones.iter().map(|b| b.to_generic::<4>()).collect()),
            5 => Self::resize_bones(self.bones.iter().map(|b| b.to_generic::<5>()).collect()),
            6 => Self::resize_bones(self.bones.iter().map(|b| b.to_generic::<6>()).collect()),
            7 => Self::resize_bones(self.bones.iter().map(|b| b.to_generic::<7>()).collect()),
            8 => Self::resize_bones(self.bones.iter().map(|b| b.to_generic::<8>()).collect()),
            _ => unreachable!(),
        }
    }

    pub fn to_output(&self) -> String {
        let filename = format!("{}_{SKELETON_EXTENSION}", self.name);
        let bone_count = self.bones.len();

        // Get the max number of children
        let max_children = self
            .bones
            .iter()
            .map(|bone| {
                println!("ch: {:?}", bone.children);
                bone.children.len()
            })
            .max()
            .unwrap();

        let archive = self.resize(max_children);
        write_file(&filename, &archive);
        let name = filename.to_uppercase();

        format!(
            "pub static {name}: &SkeletonBytes<{bone_count}, {max_children}> = &SkeletonBytes(include_bytes!(\"{filename}\"));\n"
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

        let mut joints = HashMap::<String, u8>::new();
        for (index, bone) in skin.joints().enumerate() {
            let name = bone.name().unwrap().to_string();
            joints.insert(name, index as u8);
        }

        let view = ibm.view().unwrap();
        let data = &buffer[view.offset()..view.offset() + view.length()];

        for matrix in data.chunks_exact(size_of::<Mat4>()) {
            let matrix: &Mat4 = from_bytes(matrix);
            ibms.push(matrix);
        }

        for (index, bone) in skin.joints().enumerate() {
            let mut children = Vec::new();
            for child in bone.children() {
                let child_index = joints.get(child.name().unwrap()).unwrap();
                children.push(*child_index);
            }

            let bone = BoneVec {
                children: children.into(),
                local_matrix: Mat4::from_cols_array_2d(&bone.transform().matrix()),
                inverse_bind_matrix: *ibms[index],
            };
            bones.push(bone);
        }

        let len = bones.len();

        println!("Found a skeleton with {len} bones.");

        bones.iter().for_each(|bone| {
            println!("children: {:?}", bone.children);
        });

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
