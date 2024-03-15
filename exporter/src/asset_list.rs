use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssetList {
    pub in_dir: String,
    pub out_dir: String,
    pub out_file: Option<String>,
    pub meshes: Vec<String>,
    pub images: Vec<String>,
}
