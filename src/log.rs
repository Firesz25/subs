use tracing_subscriber::fmt::writer::MakeWriterExt;

pub fn setup() {
    let logfile = tracing_appender::rolling::daily("./logs", "subs");
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);
    tracing_subscriber::fmt()
        .with_writer(stdout.and(logfile))
        .init();
}
