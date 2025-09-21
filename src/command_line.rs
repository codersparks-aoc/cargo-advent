use crate::command_line::model::{GenerateRustProjectCliArgs, ModeSubCommands};
use cargo_advent::context::{AocData, AppAction, AppContext};
use clap::Parser;
use tracing::{debug, Level};

mod model;

pub fn parse_command_line() -> AppContext {
    let cmd_args = model::RootCliArgs::parse();

    let level = match cmd_args.verbosity {
        0 => Level::ERROR,
        1 => Level::INFO,
        2 => Level::DEBUG,
        _ => Level::TRACE,
    };
    tracing_subscriber::fmt().with_max_level(level).init();
    debug!("parsed arguments: {:#?}", cmd_args);
    debug!("log level: {}", level);

    let app_mode = match cmd_args.mode {
        ModeSubCommands::GenerateRustProject { data } => parse_generate_rust_project_args(data),
    };

    AppContext {
        app_action: app_mode,
    }
}

fn parse_generate_rust_project_args(data: GenerateRustProjectCliArgs) -> AppAction {
    let aoc_data = AocData::from(&data.aoc_cli_args);
    debug!("parsed aoc_data: {:#?}", aoc_data);
    let project_name = data
        .project_name
        .unwrap_or_else(|| format!("aoc_{}_{}", aoc_data.year, aoc_data.day));
    debug!("parsed project_name: {}", project_name);
    let path = data.path.unwrap_or_else(|| "./".to_string());
    debug!("parsed path: {}", path);
    AppAction::GenerateRustProject {
        project_name,
        path,
        aoc_data,
        template: data.template,
    }
}

#[cfg(test)]
mod tests {
    use crate::command_line::model::{ModeSubCommands, RootCliArgs};
    use crate::command_line::parse_generate_rust_project_args;
    use cargo_advent::context::{AocData, AppAction};
    use chrono::{Datelike, Utc};
    use clap::Parser;
    use tracing::info;

    #[test_log::test]
    fn test_parse_generate_rust_project_args_no_optional_supplied() {
        let line = "cargo-advent -vvv generate";
        let args = shlex::split(line).unwrap();
        let cli_args = RootCliArgs::try_parse_from(args).unwrap();

        info!("parsed cli_args: {:#?}", cli_args);

        match cli_args.mode {
            ModeSubCommands::GenerateRustProject { data } => {
                let action = parse_generate_rust_project_args(data);

                match action {
                    AppAction::GenerateRustProject {
                        project_name,
                        path,
                        aoc_data,
                        template,
                    } => {
                        let date = Utc::now();
                        let day = date.day();
                        let year = date.year();
                        assert_eq!(project_name, format!("aoc_{}_{}", year, day));
                        assert_eq!(path, "./");
                        assert_eq!(
                            template,
                            "https://github.com/codersparks-aoc/aoc-rust-template.git"
                        );
                        assert_eq!(
                            aoc_data,
                            AocData {
                                year,
                                day,
                                ..AocData::default()
                            }
                        );
                    }
                }
            }
        }
    }

    #[test_log::test]
    fn test_parse_generate_rust_project_args_all_optional_supplied() {
        let line = "cargo-advent -vvv generate -n test-name -d 2 -y 2010 -p \"./random\" -t template.url -b base.url";

        let args = shlex::split(line).unwrap();
        let cli_args = RootCliArgs::try_parse_from(args).unwrap();
        info!("parsed cli_args: {:#?}", cli_args);

        match cli_args.mode {
            ModeSubCommands::GenerateRustProject { data } => {
                let action = parse_generate_rust_project_args(data);

                match action {
                    AppAction::GenerateRustProject {
                        project_name,
                        path,
                        aoc_data,
                        template,
                    } => {
                        assert_eq!(project_name, "test-name");
                        assert_eq!(path, "./random");
                        assert_eq!(
                            aoc_data,
                            AocData {
                                base_url: "base.url".to_string(),
                                year: 2010,
                                day: 2,
                            }
                        );
                        assert_eq!(template, "template.url");
                    }
                }
            }
        }
    }
}
