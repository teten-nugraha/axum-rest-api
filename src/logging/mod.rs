use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling;
use tracing_appender::non_blocking::WorkerGuard;

pub fn init() -> WorkerGuard {

    let file_appender = rolling::daily("logs", "app.log");

    let (non_blocking, guard) =
        tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    guard
}