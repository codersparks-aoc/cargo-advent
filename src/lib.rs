use crate::functions::generate_project::GenerateProjectError;
use color_eyre::Report;
use thiserror::Error;

pub mod context;
pub mod functions;

pub type AdventResult<T> = Result<T, AdventError>;

#[derive(Debug, Error)]
pub enum AdventError {
    #[error(transparent)]
    GenerateProjectError(#[from] GenerateProjectError),
    #[error(transparent)]
    ColorEyreReport(#[from] Report),
}
