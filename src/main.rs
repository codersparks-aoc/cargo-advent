use tracing::debug;

mod command_line;

#[cfg(test)]
mod test_utils;

fn main() {
    let app_context = command_line::parse_command_line();
    debug!("{:#?}", app_context);
    println!("Hello, world!");
}
