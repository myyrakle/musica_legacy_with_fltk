use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("Music Directory Not Fount")]
    MusicDirectoryNotFount(#[from] io::Error),
}
