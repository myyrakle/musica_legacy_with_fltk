use std::{error::Error, fs, path::Path};
use thiserror::Error;

use crate::errors::CommonError;

pub struct FileInfo {
    pub filename: String,
}

pub fn read_file_list(path: &Path) -> Result<Vec<FileInfo>, CommonError> {
    let read_dir_result = fs::read_dir(path)?;

    let mut file_list = vec![];

    for read_dir_entry in read_dir_result {
        if let Ok(read_dir_entry) = read_dir_entry {
            if let Ok(metadata) = read_dir_entry.metadata() {
                if metadata.is_file() {
                    //println!("{:?}", metadata)
                    //file_list.push(FileInfo {filename: metadata.})
                }
            }
        }
    }

    Ok(file_list)
}
