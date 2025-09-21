use crate::{functions, AdventResult};
use chrono::{Datelike, Utc};

#[derive(Debug)]
pub struct AppContext {
    pub app_action: AppAction,
}

impl AppContext {
    pub fn run_action(&self) -> AdventResult<()> {
        self.app_action.run()
    }
}

#[derive(Debug)]
pub enum AppAction {
    GenerateRustProject {
        project_name: String,
        path: String,
        aoc_data: AocData,
        template: String,
    },
}

impl AppAction {
    pub fn run(&self) -> AdventResult<()> {
        match self {
            AppAction::GenerateRustProject {
                project_name,
                path,
                aoc_data: _,
                template,
            } => functions::generate_project::generate_project(
                project_name.clone(),
                path.clone(),
                template.clone(),
            ),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AocData {
    /// The base url for the puzzle
    pub base_url: String,
    /// The year that the puzzle was published
    pub year: i32,
    /// The day that the puzzle was published
    pub day: u32,
}

impl Default for AocData {
    fn default() -> Self {
        let date = Utc::now();
        let year = date.year();
        let day = date.day();

        Self {
            base_url: "https://adventofcode.com/".to_string(),
            year,
            day,
        }
    }
}
