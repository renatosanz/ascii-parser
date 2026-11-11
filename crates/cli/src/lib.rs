use thiserror::Error;

pub mod args;
pub mod validation;

#[derive(Error, Debug)]
pub enum AppError {}
pub type AppResult<T> = Result<T, AppError>;
