use crate::*;
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
            "pub static {name}: &TextureBytes = &TextureBytes(include_bytes!(\"{filename}\"));\n"
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
