use tracing_subscriber::{fmt::time::ChronoUtc, prelude::*, util::SubscriberInitExt};

pub async fn init_tracing() {
    let appender = tracing_appender::rolling::daily("./logs", "rakuen.log");

    let timer = ChronoUtc::new("%F %T".to_string());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_ansi(false)
                .with_target(true)
                .with_timer(timer.clone())
                .with_writer(appender),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_ansi(true)
                .with_target(false)
                .with_timer(timer),
        )
        .init();
}
