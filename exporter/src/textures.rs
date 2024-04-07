use crate::*;
use bytemuck::from_bytes;
use gltf::image::Data;
use shared::texture::Texture;

pub struct TextureOutput {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub image_data: Vec<u8>,
}

impl TextureOutput {
    pub fn to_output(&self, config: &AssetList) -> String {
        // Write the struct as Rust code
        let filename = format!("{}_{TEXTURES_EXTENSION}", self.name);
        let width = self.width;
        let height = self.height;

        let out = Texture {
            width: width as u16,
            height: height as u16,
            data: self.image_data.clone(),
        };

        let archive = rkyv::to_bytes::<_, 256>(&out).unwrap();
        write_file(config, &filename, &archive);
        let name = filename.to_uppercase();

        format!(
            "pub const {name}: &TextureBytes = &TextureBytes(include_bytes!(\"{filename}\"));\n"
        )
    }
}

pub fn generate_textures(config: &AssetList) -> String {
    let input_dir = &config.in_dir;

    let mut out = String::from(
        "pub mod textures {
    use super::*;\n",
    );

    config.images.iter().for_each(|filename| {
        // Try each of the valid image file formats
        let mut bytes = None;

        // Iterate each extension supported
        for extension in crate::SUPPORTED_IMAGE_EXTENSIONS.iter() {
            // Read in the image file
            let read_path = format!("{input_dir}/{filename}.{extension}");
            // Convert it to a vec of bytes
            if let Ok(data) = fs::read(read_path) {
                bytes = Some(data);
                break;
            }
        }

        if bytes.is_none() {
            println!("Couldn't find file: {filename}");
            return;
        }
        let bytes = bytes.unwrap();

        let image = image::load_from_memory(&bytes).unwrap();
        let image_data = image
            .pixels()
            .flat_map(|(_x, _y, pixel)| [pixel.0[0], pixel.0[1], pixel.0[2]])
            .collect::<Vec<u8>>();

        let texture = TextureOutput {
            name: filename.to_string(),
            width: image.width(),
            height: image.height(),
            image_data,
        };

        // Append the output String
        out.push_str(&texture.to_output(config));
    });

    out.push_str("}\n");

    out
}

pub fn handle_glb_images(
    images: Vec<Data>,
    output: &mut String,
    config: &AssetList,
    filename: &str,
) {
    for (index, image) in images.iter().enumerate() {
        let (size, alpha) = match image.format {
            gltf::image::Format::R8G8B8 => (1, false),
            gltf::image::Format::R8G8B8A8 => (1, true),
            gltf::image::Format::R16G16B16 => (2, false),
            gltf::image::Format::R16G16B16A16 => (2, true),
            gltf::image::Format::R32G32B32FLOAT => (4, false),
            gltf::image::Format::R32G32B32A32FLOAT => (4, true),
            // gltf::image::Format::R8 => todo!(),
            // gltf::image::Format::R8G8 => todo!(),
            // gltf::image::Format::R16 => todo!(),
            // gltf::image::Format::R16G16 => todo!(),
            _ => {
                println!("Unsupported texture format: {:?}", image.format);
                continue;
            }
        };

        let mut image_data = Vec::with_capacity((image.width * image.height) as usize);

        let chunks = (3 * size) + if alpha { size } else { 0 };

        for pixel in image.pixels.chunks_exact(chunks) {
            let (r, g, b) = match size {
                1 => (pixel[0], pixel[1], pixel[2]),
                2 => {
                    let r = *from_bytes::<u16>(&pixel[0..2]) as f32 / u16::MAX as f32;
                    let g = *from_bytes::<u16>(&pixel[2..4]) as f32 / u16::MAX as f32;
                    let b = *from_bytes::<u16>(&pixel[4..6]) as f32 / u16::MAX as f32;

                    let r = (r * u8::MAX as f32) / u8::MAX as f32;
                    let g = (g * u8::MAX as f32) / u8::MAX as f32;
                    let b = (b * u8::MAX as f32) / u8::MAX as f32;

                    (r as u8, g as u8, b as u8)
                }
                4 => {
                    let r = *from_bytes::<f32>(&pixel[0..4]) * u8::MAX as f32;
                    let g = *from_bytes::<f32>(&pixel[4..8]) * u8::MAX as f32;
                    let b = *from_bytes::<f32>(&pixel[8..12]) * u8::MAX as f32;

                    (r as u8, g as u8, b as u8)
                }
                _ => unreachable!(),
            };

            image_data.push(r);
            image_data.push(g);
            image_data.push(b);
        }

        let texture = TextureOutput {
            name: format!("{filename}_{index}"),
            width: image.width,
            height: image.height,
            image_data,
        };

        output.push_str(&texture.to_output(config));
    }
}
