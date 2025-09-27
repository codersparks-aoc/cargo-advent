use cargo_advent::AdventResult;
use std::env;
use tracing::{debug, error, info, warn};
use tracing_appender::rolling::{InitError, Rotation};
use tracing_subscriber::EnvFilter;

fn main() -> AdventResult<()> {
    let temp_dir = env::temp_dir();
    let log_dir = temp_dir.join("cargo-advent-log");
    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("cargo-advent")
        .filename_suffix(".log")
        .max_log_files(10)
        .build(log_dir)
        .unwrap();
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();

    //color_eyre::install()?;

    error!("This is an error message");
    warn!("This is a warning");
    info!("This is an info message");
    debug!("This is a debug message - you must not see it!");

    Ok(())
}

fn configure_logging() -> Result<(), InitError> {
    let temp_dir = env::temp_dir();
    let log_dir = temp_dir.join("cargo-advent-log");
    let file_appender = tracing_appender::rolling::RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("cargo-advent")
        .max_log_files(10)
        .build(log_dir)?;
    let (non_blocking, _guart) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .init();
    Ok(())
}
