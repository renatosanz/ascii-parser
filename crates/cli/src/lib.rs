use std::{fmt, path::PathBuf};

use thiserror::Error;

pub mod args;
pub mod validation;

#[derive(Error)]
pub enum AppError {
    #[error("Invalid File: {0}")]
    InvalidFile(&'static str),

    #[error("Invalid FileType, allowed filetypes: *.jpg, *.png")]
    InvalidFileType,

    #[error("Collision with generated file {0}, use -o/--override to skip this validation")]
    FileCollision(PathBuf),

    #[error("Error on image processing: {0}")]
    ProcessingError(String),
}

impl fmt::Debug for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub type AppResult<T> = Result<T, AppError>;
