use crate::*;
use shared::texture::Texture;

pub struct TextureOutput {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub image_data: Vec<u8>,
}

impl TextureOutput {
    pub fn to_output(&self) -> String {
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
        write_file(&filename, &archive);
        let name = filename.to_uppercase();

        format!(
            "pub static {name}: &TextureBytes = &TextureBytes(include_bytes!(\"{filename}\"));\n"
        )
    }
}

pub fn generate_textures() -> String {
    let mut out = String::from(
        "pub mod textures {
    use super::*;\n",
    );

    TEXTURES.iter().for_each(|[filename, extension]| {
        // Read in the image file
        let read_path = format!("{INPUT_DIR}/{filename}.{extension}");

        // Convert it to a vec of bytes
        let bytes = fs::read(read_path).unwrap();
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
        out.push_str(&texture.to_output());
    });

    out.push_str("}\n");

    out
}
