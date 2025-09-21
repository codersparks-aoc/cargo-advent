use crate::AdventResult;
use cargo_generate::{generate, GenerateArgs, TemplatePath, Vcs};
use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;
use thiserror::Error;
use tracing::debug;

#[derive(Debug, Error)]
pub enum GenerateProjectError {
    #[error("Target directory ({0}) does not exist.")]
    TargetDirectoryNotExists(String),
    #[error(
        "Project name must match contain only letters, numbers, underscore and dash, supplied: {0}"
    )]
    InvalidProjectName(String),
    #[error(transparent)]
    GenerateError(#[from] anyhow::Error),
}

lazy_static! {
    pub static ref PROJECT_NAME_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
}

pub fn generate_project(project_name: String, path: String, template: String) -> AdventResult<()> {
    let path_buf = PathBuf::from(&path);
    if !path_buf.is_dir() {
        return Err(GenerateProjectError::TargetDirectoryNotExists(path).into());
    };

    debug!(path = path, "Checked path to be valid");

    if !PROJECT_NAME_REGEX.is_match(&project_name) {
        return Err(GenerateProjectError::InvalidProjectName(project_name).into());
    }

    debug!(
        project_name = project_name.as_str(),
        "Project name is valid"
    );

    let generate_args = GenerateArgs {
        name: Some(project_name.clone()),
        vcs: Some(Vcs::Git),
        template_path: TemplatePath {
            git: Some(template),
            ..TemplatePath::default()
        },
        destination: Some(path_buf),
        ..GenerateArgs::default()
    };

    debug!(args=?generate_args, "Generated args");

    let path = generate(generate_args).map_err(GenerateProjectError::GenerateError)?;

    debug!(project_path=?path, "Generated project");
    Ok(())
}
