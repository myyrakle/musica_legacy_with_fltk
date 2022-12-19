use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileInfo {
    pub filename: String,
    pub filepath: PathBuf,
}
