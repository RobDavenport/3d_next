use shared::skin::{Skin, SkinEntry};

use crate::{write_file, SKIN_EXTENSION};

// A collection of bone indices & weights
pub struct SkinOutput {
    pub name: String,
    pub entries: Vec<SkinEntry>,
}

impl SkinOutput {
    pub fn to_output(&self) -> String {
        let filename = format!("{}_{SKIN_EXTENSION}", self.name);

        let out = Skin(self.entries.clone().into_boxed_slice());

        let archive = rkyv::to_bytes::<_, 256>(&out).unwrap();
        write_file(&filename, &archive);
        let name = filename.to_uppercase();

        format!(
            "pub static {name}: &SkinBytes = &SkinBytes(include_bytes!(\"{filename}\"));\n"
        )
    }
}
