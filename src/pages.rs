pub mod home;
pub mod profile;
pub mod settings;

pub enum Page {
	PageHome,
	PageProfile,
	PageSettings,
}

// pub fn make_page(page: Page) -> Box<dyn cosmic::app::Page> {
// 	match page {
// 		Page::PageHome => Box::new(crate::pages::home::HomePage::new()),
// 		Page::PageProfile => Box::new(crate::pages::profile::ProfilePage::new()),
// 		Page::PageSettings => Box::new(crate::pages::settings::SettingsPage::new()),
// 	}
// }
