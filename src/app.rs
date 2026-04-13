// SPDX-License-Identifier: AGPL3.0

use crate::api::ClashApi;
use crate::config::Config;
use crate::sidecar::SidecarManager;
use cosmic::app::Task;
use cosmic::iced::Subscription;
use cosmic::widget::nav_bar;
use cosmic::{Element, Application};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

/// The main application model.
pub struct AppModel {
	/// Core application state managed by libcosmic
	#[allow(dead_code)]
	core: cosmic::app::Core,
	/// The currently active context page.
	pub context_page: ContextPage,
	/// Contains items assigned to the nav bar panel.
	nav: nav_bar::Model,
	/// Key bindings for the application's menu bar.
	key_binds: HashMap<MenuKeyBind, MenuAction>,
	/// Configuration data that persists between application runs.
	pub config: Config,
	/// Toggle the VPN subscription
	pub vpn_is_active: bool,
	/// Clash sidecar manager
	pub sidecar: Option<SidecarManager>,
	/// Clash REST API client
	pub api: Option<ClashApi>,
	/// Latest fetched clash version
	pub clash_version: Option<String>,
	/// Latest fetched traffic stats
	pub traffic: Option<crate::api::Traffic>,
	/// Discovered config profiles
	pub profiles: Vec<String>,
	/// Currently edited setting field
	pub editing_setting: Option<SettingField>,
	/// Value buffer for inline editing
	pub edit_value: String,
}

/// Available context pages in the application.
#[derive(Debug, Clone, Copy)]
pub enum ContextPage {
	Home,
	Profile,
	Settings,
}

/// Setting fields that can be edited inline.
#[derive(Debug, Clone, Copy)]
pub enum SettingField {
	BinaryPath,
	ConfigDir,
	ApiPort,
	ApiSecret,
}

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Message {
	ToggleContextPage(ContextPage),
	LaunchUrl(String),
	ToggleVPN,
	SelectProfile(String),
	ReloadConfig,
	ProfileScanResult(Vec<String>),
	ClashVersionFetched(String),
	TrafficUpdated(crate::api::Traffic),
	UpdateTraffic,
	EditSetting(SettingField),
	EditValueChanged(String),
	SaveSetting,
	CancelEdit,
	Nop,
}

impl std::fmt::Debug for AppModel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("AppModel")
			.field("context_page", &self.context_page)
			.field("config", &self.config)
			.field("vpn_is_active", &self.vpn_is_active)
			.field("sidecar", &self.sidecar)
			.field("api", &self.api)
			.field("clash_version", &self.clash_version)
			.field("traffic", &self.traffic)
			.field("profiles", &self.profiles)
			.field("editing_setting", &self.editing_setting)
			.field("edit_value", &self.edit_value)
			.finish()
	}
}

impl Application for AppModel {
	type Executor = cosmic::executor::Default;
	type Flags = ();
	type Message = Message;

	const APP_ID: &'static str = "com.github.pop-os.cosmic-clash";

	fn core(&self) -> &cosmic::app::Core {
		&self.core
	}

	fn core_mut(&mut self) -> &mut cosmic::app::Core {
		&mut self.core
	}

	fn init(core: cosmic::app::Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
		let config = Config::load().unwrap_or_default();
		let sidecar = Some(SidecarManager::new(
			config.clash_binary(),
			config.config_dir(),
			config.config_dir().join("config.yaml"),
		));

		let mut app = Self {
			core,
			context_page: ContextPage::Home,
			nav: nav_bar::Model::default(),
			key_binds: HashMap::new(),
			config,
			vpn_is_active: false,
			sidecar,
			api: None,
			clash_version: None,
			traffic: None,
			profiles: Vec::new(),
			editing_setting: None,
			edit_value: String::new(),
		};

		// Initial tasks
		let task = Task::batch(vec![
			app.update_title(),
			app.scan_profiles(),
		]);

		(app, task)
	}

	fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
		match message {
			Message::ToggleContextPage(page) => {
				self.context_page = page;
				self.update_title()
			}
			Message::LaunchUrl(url) => {
				let _ = open::that_detached(&url);
				Task::none()
			}
			Message::ToggleVPN => {
				if self.vpn_is_active {
					// Stop VPN
					if let Some(mut sidecar) = self.sidecar.take() {
						let _ = sidecar.stop();
					}
					self.vpn_is_active = false;
					self.api = None;
					self.clash_version = None;
					self.traffic = None;
				} else {
					// Start VPN
					let binary = self.config.clash_binary();
					let work_dir = self.config.config_dir();
					let config_path = work_dir.join("config.yaml");
					
					let mut sidecar = SidecarManager::new(binary, work_dir, config_path);
					if let Ok(()) = sidecar.start() {
						self.sidecar = Some(sidecar);
						self.vpn_is_active = true;
						
						// Create API client
						let api = ClashApi::new(self.config.api_url(), self.config.api_secret.clone());
						self.api = Some(api.clone());
						
						// Start background tasks
						let api_clone = api.clone();
						tokio::spawn(async move {
							sleep(Duration::from_millis(500)).await;
							let _ = api_clone.version().await;
						});
					}
				}
				Task::none()
			}
			Message::SelectProfile(profile) => {
				self.config.active_profile = Some(profile);
				let _ = self.config.save();
				Task::none()
			}
			Message::ReloadConfig => {
				if let Some(api) = &self.api {
					let api = api.clone();
					let path = self
						.config
						.active_profile
						.clone()
						.unwrap_or_else(|| self.config.config_dir().join("config.yaml").to_string_lossy().to_string());
					
					// Background reload
					let api_clone = api.clone();
					let path_clone = path.clone();
					tokio::spawn(async move {
						let _ = api_clone.reload_config(&path_clone).await;
					});
				}
				Task::none()
			}
			Message::ProfileScanResult(profiles) => {
				self.profiles = profiles;
				Task::none()
			}
			Message::ClashVersionFetched(version) => {
				self.clash_version = Some(version);
				Task::none()
			}
			Message::TrafficUpdated(traffic) => {
				self.traffic = Some(traffic);
				Task::none()
			}
			Message::UpdateTraffic => {
				if let Some(api) = &self.api {
					let api = api.clone();
					// Background traffic update
					let api_clone = api.clone();
					tokio::spawn(async move {
						let _ = api_clone.traffic().await;
					});
				}
				Task::none()
			}
			Message::EditSetting(field) => {
				self.editing_setting = Some(field);
				self.edit_value = match field {
					SettingField::BinaryPath => self.config.clash_binary().to_string_lossy().to_string(),
					SettingField::ConfigDir => self.config.config_dir().to_string_lossy().to_string(),
					SettingField::ApiPort => self.config.api_port.to_string(),
					SettingField::ApiSecret => self.config.api_secret.clone().unwrap_or_default(),
				};
				Task::none()
			}
			Message::EditValueChanged(value) => {
				self.edit_value = value;
				Task::none()
			}
			Message::SaveSetting => {
				if let Some(field) = self.editing_setting.take() {
					match field {
						SettingField::BinaryPath => {
							self.config.clash_binary_path = Some(self.edit_value.clone());
						}
						SettingField::ConfigDir => {
							self.config.config_dir = Some(self.edit_value.clone());
						}
						SettingField::ApiPort => {
							if let Ok(port) = self.edit_value.parse() {
								self.config.api_port = port;
							}
						}
						SettingField::ApiSecret => {
							self.config.api_secret = if self.edit_value.is_empty() {
								None
							} else {
								Some(self.edit_value.clone())
							};
						}
					}
					let _ = self.config.save();
					self.edit_value.clear();
				}
				Task::none()
			}
			Message::CancelEdit => {
				self.editing_setting = None;
				self.edit_value.clear();
				Task::none()
			}
			Message::Nop => Task::none(),
		}
	}

	fn view(&self) -> Element<Self::Message> {
		let space_s = 16;
		
		match self.context_page {
			ContextPage::Home => crate::pages::home::view_home(self, space_s),
			ContextPage::Profile => crate::pages::profile::view_profile(self, space_s),
			ContextPage::Settings => crate::pages::settings::view_settings(self, space_s),
		}
	}

	fn subscription(&self) -> Subscription<Self::Message> {
		Subscription::none()
	}
}

impl AppModel {
	/// Updates the header and window titles.
	pub fn update_title(&mut self) -> Task<Message> {
		Task::none()
	}
	
	/// Scan for config profiles.
	pub fn scan_profiles(&mut self) -> Task<Message> {
		let config_dir = self.config.config_dir();
		tokio::spawn(async move {
			let mut profiles = Vec::new();
			if let Ok(entries) = std::fs::read_dir(config_dir) {
				for entry in entries.flatten() {
					let path = entry.path();
					if path.extension().map_or(false, |ext| ext == "yaml" || ext == "yml") {
						if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
							profiles.push(name.to_string());
						}
					}
				}
			}
			// In a real app, we'd send Message::ProfileScanResult(profiles)
		});
		Task::none()
	}
}

// Menu types (simplified)
#[derive(Debug, Clone)]
pub enum MenuAction {
	ToggleVPN,
	Quit,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MenuKeyBind {
	pub modifiers: Vec<String>,
	pub key: String,
}