use std::{
    fs::{self},
    io::Write,
};

use image::GenericImageView;

const INPUT_DIR: &str = "./assets";
const OUTPUT_DIR: &str = "./src/assets/generated";

const IMAGES: &[[&str; 2]] = &[
    ["gamercade", "png"],
    ["brickwall", "jpg"],
    ["brickwall_normal", "jpg"],
];

fn main() {
    let modfile_path = format!("{OUTPUT_DIR}/mod.rs");

    let mut output = String::from("// Autogenerated file from build.rs\n\n");

    output.push_str(&generate_images());

    let mut output_file = fs::File::create(&modfile_path).unwrap();

    output_file
        .write_all(output.as_bytes())
        .expect("Failed to write output file");
}

fn generate_images() -> String {
    let mut out = String::from(
        "pub mod textures {
    use crate::assets::Texture;\n",
    );

    IMAGES.iter().for_each(|[filename, extension]| {
        // Read in the image file
        let read_path = format!("{INPUT_DIR}/{filename}.{extension}");

        // Convert it to a vec of bytes
        let bytes = fs::read(read_path).unwrap();
        let image = image::load_from_memory(&bytes).unwrap();
        let image_bytes = image
            .pixels()
            .into_iter()
            .flat_map(|(_x, _y, pixel)| [pixel.0[0], pixel.0[1], pixel.0[2]])
            .collect::<Vec<u8>>();

        // Write out the bytes of the image
        let write_path = format!("{OUTPUT_DIR}/{filename}");
        let mut file_out = fs::File::create(&write_path).unwrap();
        file_out.write(&image_bytes).unwrap();

        // Write the struct as Rust code
        let filename = filename.to_uppercase();
        let width = image.width();
        let height = image.height();
        let append = format!(
            "
    pub const {filename}: &Texture = &Texture {{
        width: {width},
        height: {height},
        data: include_bytes!(\"{filename}\")
    }};\n"
        );

        // Append the output String
        out.push_str(&append);
    });

    out.push_str("}");

    out
}
