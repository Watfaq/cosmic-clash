use tracing_error::ErrorLayer;
use tracing_subscriber::EnvFilter;
#[allow(unused_imports)]
use tracing_subscriber::{filter::LevelFilter, fmt::format::FmtSpan, prelude::*};

pub(crate) fn init_logger(level: LevelFilter) {
    let filter = EnvFilter::from_default_env()
        .add_directive(format!("clash={}", level).parse().unwrap())
        .add_directive(format!("clash_lib={}", level).parse().unwrap())
        .add_directive(format!("clash_ffi={}", level).parse().unwrap())
        .add_directive("warn".parse().unwrap());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_thread_names(true)
        .with_filter(LevelFilter::TRACE);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}
