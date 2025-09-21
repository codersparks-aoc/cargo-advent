use cargo_advent::AdventResult;
use tracing::debug;

mod command_line;

fn main() -> AdventResult<()> {
    let app_context = command_line::parse_command_line();
    debug!("{:#?}", app_context);
    app_context.run_action()
}
