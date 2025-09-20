use tracing::Level;

pub fn init_test_logging() {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();
}