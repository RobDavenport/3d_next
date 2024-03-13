use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssetList {
    pub meshes: Vec<String>,
    pub images: Vec<String>,
}
