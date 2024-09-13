use crate::configuration::{self, Environment};
use tracing_appender::non_blocking;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

/// Initialize tracing subscriber
/// ### On Local Environment:
/// Write traces to `stdout`
/// ### On Production Environment:
/// Log to files rotated daily
#[must_use = "the guard must be assigned to a local variable"]
pub fn init_subscriber() -> tracing_appender::non_blocking::WorkerGuard {
    let env = configuration::get_environment();
    let (writer, guard) = match env {
        Environment::Local => non_blocking(std::io::stdout()),
        Environment::Production => non_blocking(
            RollingFileAppender::builder()
                .rotation(Rotation::DAILY)
                .filename_prefix("blindchannel")
                .filename_suffix("log")
                .build("logs")
                .expect("failed to initialize rolling file appender"),
        ),
    };
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "blindchannel=debug,tower_http=debug,axum::rejection=trace".into());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(env.is_local())
        .with_writer(writer)
        .init();
    guard
}
pub fn init_test_subscriber() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::OFF.into())
        .with_env_var("TEST_LOG")
        .from_env_lossy();
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}
