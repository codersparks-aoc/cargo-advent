use cargo_advent::{AdventError, AdventResult};
use tracing::debug;

mod command_line;

fn main() -> AdventResult<()> {
    let load_env_success = loadenv::load().map_err(AdventError::EnvLoadError)?;
    let app_context = command_line::parse_command_line();
    // We have to delay the output for the env file loading so that verbosity can be configured
    debug!(".env was file found and loaded: {:#?}", load_env_success);
    debug!("{:#?}", app_context);
    app_context.run_action()
}
