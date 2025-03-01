use tracing_subscriber::fmt::time::LocalTime;

pub fn local_time_tracer() {
    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .init();
}