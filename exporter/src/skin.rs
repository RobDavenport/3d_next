use std::collections::HashMap;

use rkyv::AlignedVec;
use shared::{
    skin::{Skin, SkinEntry},
    SKIN_MAX_BONE_INFLUENCES,
};

use crate::{asset_list::AssetList, write_file, SKIN_EXTENSION};

// Each of these is guarenteed to have the same length
pub struct SkinEntryVec {
    pub bones_indices: Vec<i8>,
    pub weights: Vec<f32>,
}

impl SkinEntryVec {
    fn to_sized<const MAX_INFLUENCES: usize>(&self) -> SkinEntry<MAX_INFLUENCES> {
        SkinEntry {
            bones_indices: std::array::from_fn(|i| self.bones_indices[i]),
            weights: std::array::from_fn(|i| self.weights[i]),
        }
    }
}

// A collection of bone indices & weights
pub struct SkinOutput {
    pub name: String,
    pub entries: Vec<SkinEntryVec>,
}

impl SkinOutput {
    fn to_sized<const MAX_INFLUENCES: usize>(&self) -> AlignedVec {
        let entries: Vec<SkinEntry<MAX_INFLUENCES>> =
            self.entries.iter().map(|e| e.to_sized()).collect();
        let out = Skin(entries.into_boxed_slice());
        rkyv::to_bytes::<_, 256>(&out).unwrap()
    }

    fn to_archive(&self, max_influences: usize) -> AlignedVec {
        match max_influences {
            0 => self.to_sized::<0>(),
            1 => self.to_sized::<1>(),
            2 => self.to_sized::<2>(),
            3 => self.to_sized::<3>(),
            4 => self.to_sized::<4>(),
            i if i > SKIN_MAX_BONE_INFLUENCES => {
                panic!("too many bone influences {i}, max is {SKIN_MAX_BONE_INFLUENCES}")
            }
            _ => unreachable!(),
        }
    }

    pub fn to_output(&self, config: &AssetList) -> String {
        let filename = format!("{}_{SKIN_EXTENSION}", self.name);

        let max_influences = self
            .entries
            .iter()
            .map(|entry| entry.bones_indices.len())
            .max()
            .unwrap();

        let archive = self.to_archive(max_influences);
        write_file(config, &filename, &archive);
        let name = filename.to_uppercase();

        format!(
            "pub static {name}: &SkinBytes<{max_influences}> = &SkinBytes(include_bytes!(\"{filename}\"));\n"
        )
    }
}

// Maps bone names -> Indices
pub fn get_bone_name_index_maps(skin: &gltf::Skin) -> (HashMap<String, i8>, HashMap<usize, i8>) {
    let mut names = HashMap::<String, i8>::new();
    let mut indices = HashMap::<usize, i8>::new();
    for (bone_index, node) in skin.joints().enumerate() {
        let name = node.name().unwrap().to_string();
        names.insert(name, bone_index as i8);
        indices.insert(node.index(), bone_index as i8);
    }
    (names, indices)
}
