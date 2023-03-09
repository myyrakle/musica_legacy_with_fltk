use std::{fs, path::Path};

use crate::{errors::CommonError, types::file_info::FileInfo};

pub fn read_file_list(path: &Path) -> Result<Vec<FileInfo>, CommonError> {
    let read_dir_result = fs::read_dir(path)?;

    let mut file_list = vec![];

    for read_dir_entry in read_dir_result {
        if let Ok(read_dir_entry) = read_dir_entry {
            if let Ok(metadata) = read_dir_entry.metadata() {
                if metadata.is_file() {
                    let filepath = read_dir_entry.path();
                    let filename = read_dir_entry
                        .file_name()
                        .to_str()
                        .unwrap_or("error")
                        .to_owned();
                    file_list.push(FileInfo { filename, filepath })
                }
            }
        }
    }

    Ok(file_list)
}
