use std::path::PathBuf;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub directory_path: PathBuf,
}
