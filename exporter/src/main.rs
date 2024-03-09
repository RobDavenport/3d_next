use std::{fs, io::Write, mem::size_of};

use glam::{Mat4, Vec2, Vec3};
use image::GenericImageView;

use crate::{meshes::generate_meshes, textures::generate_textures};

pub const INPUT_DIR: &str = "./assets";
const CODE_OUTPUT_DIR: &str = "./game/src/generated/mod.rs";
pub const ASSET_OUTPUT_DIR: &str = "./game/src/generated/";

const TEXTURES: &[[&str; 2]] = &[
    ["gamercade", "png"],
    ["brickwall", "jpg"],
    ["brickwall_normal", "jpg"],
];

const MESHES: &[[&str; 2]] = &[
    ["BoxVertexColors", "glb"],
    ["Fox", "glb"],
    ["Duck", "glb"],
    ["DamagedHelmet", "glb"],
];

mod animations;
mod meshes;
mod proc_meshes;
mod skeleton;
mod textures;

// For output files
pub const TEXTURES_EXTENSION: &str = "TEX";
pub const MESH_EXTENSION: &str = "MESH";

pub const ANIMATION_EXTENSION: &str = "ANI";
pub const SKELETON_EXTENSION: &str = "SKL";
pub const SKIN_EXTENSION: &str = "SKN";

// Writes the bytes of the file
pub fn write_file(filename: &str, data: &[u8]) {
    let write_path = format!("{ASSET_OUTPUT_DIR}/{filename}");
    let mut file_out = fs::File::create(write_path).unwrap();
    file_out.write_all(data).unwrap();
}

fn main() {
    println!("Executing custom export script...");

    let mut output = String::from(
        "/// Autogenerated file from exporter/main.rs.
/// To regenrate this file, run the exporter again
/// You shouldn't be editing this file.
use shared::{texture::TextureBytes, mesh::MeshRawBytes};\n\n",
    );

    output.push_str(&generate_textures());
    output.push_str(&generate_meshes());

    let mut output_file = fs::File::create(CODE_OUTPUT_DIR).unwrap();

    output_file
        .write_all(output.as_bytes())
        .expect("Failed to write output file");
}
