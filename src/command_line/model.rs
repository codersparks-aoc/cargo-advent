use cargo_advent::context::AocData;
use clap::{Args, Parser, Subcommand};
use tracing::debug;

/// cargo-advent
///
/// A command line to help with configuring yearly projects from Advent of Code (aoc) in Rust
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct RootCliArgs {
    /// Set the verbosity of the logging default ERROR, -v INFO, -vv DEBUG
    #[arg(short, long, action=clap::ArgAction::Count)]
    pub verbosity: u8,
    /// Provide the sub commands functionality
    #[command(subcommand)]
    pub mode: ModeSubCommands,
}

#[derive(Debug, Subcommand)]
pub enum ModeSubCommands {
    /// Generate a rust project using the configuration supplied
    #[command(name = "generate")]
    GenerateRustProject {
        #[clap(flatten)]
        data: GenerateRustProjectCliArgs,
    },
}

#[derive(Debug, Clone, Args)]
pub struct GenerateRustProjectCliArgs {
    /// The name to use for the generated project. If not supplied will use name aoc-{aoc_year}-{aoc_day}
    #[arg(short = 'n', long)]
    pub project_name: Option<String>,
    /// The repository to use as a template.
    #[arg(
        short,
        long,
        default_value = "https://github.com/codersparks-aoc/aoc-rust-template.git"
    )]
    pub template_repository: String,
    /// The data required for configuration for advent of code
    #[clap(flatten)]
    pub aoc_cli_args: AocCliArgs,
    /// The path to use for where to create the project. Default ./{project_name}
    #[arg(short, long)]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Args)]
pub struct AocCliArgs {
    /// The base url for the aoc website
    #[arg(short, long)]
    pub base_url: Option<String>,
    /// The year the aoc puzzle was published. If not supplied will use current year
    #[arg(short, long)]
    pub year: Option<i32>,
    /// The day the aoc puzzle was published. If not supplied will use current day
    #[arg(short, long)]
    pub day: Option<u32>,
}

impl From<&AocCliArgs> for AocData {
    fn from(args: &AocCliArgs) -> Self {
        let mut aoc_data = AocData::default();

        if let Some(base_url) = args.base_url.clone() {
            aoc_data.base_url = base_url;
        }

        if let Some(year) = args.year {
            aoc_data.year = year;
        }
        if let Some(day) = args.day {
            aoc_data.day = day;
        }
        debug!("parsed aoc_data: {:#?}", aoc_data);
        aoc_data
    }
}
