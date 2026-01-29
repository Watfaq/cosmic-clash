//! Clash Iced - A GUI client for Clash proxy
//!
//! This is the initial GUI implementation using the iced framework.
//! The current version provides the user interface structure for:
//! - Proxy URL configuration
//! - Port configuration
//! - Basic start/stop controls
//! - Config file switching
//!
//! Future enhancements will integrate with the actual Clash proxy service.

use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::{Alignment, Element, Length, Task};

fn main() -> iced::Result {
    iced::application(
        "Clash Iced - Clash Client",
        ClashApp::update,
        ClashApp::view,
    )
    .window_size((800.0, 600.0))
    .run_with(ClashApp::new)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ConfigFile {
    name: String,
    path: String,
}

impl ConfigFile {
    fn new(name: String, path: String) -> Self {
        Self { name, path }
    }
}

impl std::fmt::Display for ConfigFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default)]
struct ClashApp {
    proxy_url: String,
    port: String,
    status: String,
    config_files: Vec<ConfigFile>,
    selected_config: Option<ConfigFile>,
    config_path_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    ProxyUrlChanged(String),
    PortChanged(String),
    StartProxy,
    StopProxy,
    ConfigSelected(ConfigFile),
    ConfigPathChanged(String),
    AddConfigFile,
}

impl ClashApp {
    fn new() -> (Self, Task<Message>) {
        // Initialize with some example config files
        let default_configs = vec![
            ConfigFile::new("Default".to_string(), "/etc/clash/config.yaml".to_string()),
            ConfigFile::new(
                "Home".to_string(),
                "~/.config/clash/config.yaml".to_string(),
            ),
            ConfigFile::new("Custom".to_string(), "./clash-config.yaml".to_string()),
        ];

        (
            Self {
                proxy_url: String::new(),
                port: String::from("7890"),
                status: String::from("Stopped"),
                config_files: default_configs.clone(),
                selected_config: Some(default_configs[0].clone()),
                config_path_input: String::new(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ProxyUrlChanged(value) => {
                // TODO: Add URL validation
                self.proxy_url = value;
            }
            Message::PortChanged(value) => {
                // TODO: Add port number validation (1-65535)
                self.port = value;
            }
            Message::StartProxy => {
                // TODO: Integrate with actual Clash proxy service
                self.status = String::from("Running");
            }
            Message::StopProxy => {
                // TODO: Stop the Clash proxy service
                self.status = String::from("Stopped");
            }
            Message::ConfigSelected(config) => {
                // TODO: Load the selected config file
                self.selected_config = Some(config);
            }
            Message::ConfigPathChanged(value) => {
                self.config_path_input = value;
            }
            Message::AddConfigFile => {
                // Add new config file from the input
                if !self.config_path_input.is_empty() {
                    let name = self
                        .config_path_input
                        .split('/')
                        .next_back()
                        .unwrap_or("New Config")
                        .to_string();
                    let new_config = ConfigFile::new(name, self.config_path_input.clone());
                    self.config_files.push(new_config.clone());
                    self.selected_config = Some(new_config);
                    self.config_path_input.clear();
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("Clash Iced").size(32);

        let status_text = text(format!("Status: {}", self.status)).size(20);

        // Config file selection
        let config_label = text("Config File:").size(14);
        let config_picker = pick_list(
            &self.config_files[..],
            self.selected_config.as_ref(),
            Message::ConfigSelected,
        )
        .placeholder("Select a config file")
        .padding(10);

        // Display current config path
        let current_config_text = if let Some(config) = &self.selected_config {
            text(format!("Path: {}", config.path)).size(12)
        } else {
            text("No config selected").size(12)
        };

        // Add new config file
        let config_path_input = text_input("Enter config file path...", &self.config_path_input)
            .on_input(Message::ConfigPathChanged)
            .padding(10);

        let add_config_button = button(text("Add Config").size(14))
            .on_press(Message::AddConfigFile)
            .padding(8);

        let proxy_input = text_input("Enter proxy URL...", &self.proxy_url)
            .on_input(Message::ProxyUrlChanged)
            .padding(10);

        let port_input = text_input("Port", &self.port)
            .on_input(Message::PortChanged)
            .padding(10);

        let start_button = button(text("Start Proxy").size(16))
            .on_press(Message::StartProxy)
            .padding(10);

        let stop_button = button(text("Stop Proxy").size(16))
            .on_press(Message::StopProxy)
            .padding(10);

        let controls = row![start_button, stop_button].spacing(10);

        let content = column![
            title,
            status_text,
            text("").size(10), // spacer
            config_label,
            config_picker,
            current_config_text,
            text("").size(10), // spacer
            text("Add New Config:").size(14),
            row![config_path_input, add_config_button].spacing(10),
            text("").size(10), // spacer
            text("Proxy URL:").size(14),
            proxy_input,
            text("Port:").size(14),
            port_input,
            controls,
        ]
        .spacing(20)
        .padding(20)
        .align_x(Alignment::Start);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}
