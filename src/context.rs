use chrono::{Datelike, Utc};

#[derive(Debug)]
pub struct AppContext {
    pub app_action: AppAction,
}

#[derive(Debug)]
pub enum AppAction {
    GenerateRustProject {
        project_name: String,
        path: String,
        aoc_data: AocData,
    },
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
