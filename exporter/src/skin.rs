use shared::skin::{SkinEntryRaw, SkinRaw};

use crate::{write_file, SKIN_EXTENSION};

// A collection of bone indices & weights
pub struct SkinOutput {
    pub name: String,
    pub entries: Vec<SkinEntryRaw>,
}

impl SkinOutput {
    pub fn to_output(&self) -> String {
        let filename = format!("{}_{SKIN_EXTENSION}", self.name);

        let max_weight_count = self.entries.iter().map(|e| e.weights.len()).max().unwrap();

        let out = SkinRaw(self.entries.clone().into_boxed_slice());

        let archive = rkyv::to_bytes::<_, 256>(&out).unwrap();
        write_file(&filename, &archive);
        let name = filename.to_uppercase();

        format!(
            "pub static {name}: &SkinRawBytes<{max_weight_count}> = &SkinRawBytes(include_bytes!(\"{filename}\"));\n"
        )
    }
}
