use crate::functions::generate_project::GenerateProjectError;
use thiserror::Error;

pub mod context;
pub mod functions;

pub type AdventResult<T> = Result<T, AdventError>;

#[derive(Debug, Error)]
pub enum AdventError {
    #[error(transparent)]
    GenerateProjectError(#[from] GenerateProjectError),
    #[error(".env file exists but could not be read: {0}")]
    EnvLoadError(String),
}
