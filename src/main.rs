use tracing::level_filters::LevelFilter;

mod app;
mod config;
mod i18n;
mod log;
mod pages;
mod sidecar;

fn main() -> eyre::Result<()> {
	// Get the system's preferred languages.
	let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

	// Enable localizations to be applied.
	i18n::init(&requested_languages);

	crate::log::init(LevelFilter::DEBUG)?;

	// Settings for configuring the application window and iced runtime.
	let settings = cosmic::app::Settings::default().size_limits(cosmic::iced::Limits::NONE.min_width(360.0).min_height(180.0));

	// Starts the application's event loop with `()` as the application's flags.
	cosmic::app::run::<app::AppModel>(settings, ())?;

	Ok(())
}
