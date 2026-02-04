use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::time::LocalTime, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init(level: LevelFilter) -> eyre::Result<()> {
	let filter = tracing_subscriber::filter::Targets::new()
		.with_targets(vec![("cosmic_clash", level), ("clash_lib", level)])
		.with_default(LevelFilter::WARN);
	let registry = tracing_subscriber::registry();
	registry
		.with(filter)
		.with(
			tracing_subscriber::fmt::layer()
				.with_target(true)
				.with_timer(LocalTime::new(time::macros::format_description!(
					"[year repr:last_two]-[month]-[day] [hour]:[minute]:[second]"
				))),
		)
		.try_init()?;
	Ok(())
}
