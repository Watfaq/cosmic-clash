//! Clash Iced - A GUI client for Clash proxy
//!
//! This is the initial GUI implementation using the iced framework.
//! The current version provides the user interface structure for:
//! - Proxy URL configuration
//! - Port configuration
//! - Basic start/stop controls
//! - Config file switching
//! - Integration with clash-lib via IPC for actual proxy functionality
//!
//! Future enhancements will add more advanced features.

use iced::widget::{button, column, container, pick_list, row, text, text_input, Space};
use iced::{Alignment, Element, Length, Task};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::process::{Child, Command};

mod controller;
use controller::ClashController;

fn main() -> iced::Result {
    iced::application(
        "Clash Iced - Clash Client",
        ClashApp::update,
        ClashApp::view,
    )
    .window_size((800.0, 600.0))
    .run_with(ClashApp::new)
}

// Holds the clash runtime state
#[derive(Clone)]
struct ClashRuntime {
    is_running: Arc<Mutex<bool>>,
    controller: Arc<Mutex<Option<ClashController>>>,
    process_handle: Arc<Mutex<Option<u32>>>, // Store PID instead of Child
}

impl Default for ClashRuntime {
    fn default() -> Self {
        Self {
            is_running: Arc::new(Mutex::new(false)),
            controller: Arc::new(Mutex::new(None)),
            process_handle: Arc::new(Mutex::new(None)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

struct ClashApp {
    proxy_url: String,
    port: String,
    status: String,
    config_files: Vec<ConfigFile>,
    selected_config: Option<ConfigFile>,
    config_path_input: String,
    clash_runtime: ClashRuntime,
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
    ProxyStarted(Result<(), String>),
    ProxyStopped,
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
                clash_runtime: ClashRuntime::default(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ProxyUrlChanged(value) => {
                // TODO: Add URL validation
                self.proxy_url = value;
                Task::none()
            }
            Message::PortChanged(value) => {
                // TODO: Add port number validation (1-65535)
                self.port = value;
                Task::none()
            }
            Message::StartProxy => {
                // Start the clash proxy service
                if let Some(config) = &self.selected_config {
                    self.status = String::from("Starting...");
                    let config_path = config.path.clone();
                    let runtime = self.clash_runtime.clone();
                    
                    Task::future(async move {
                        start_clash(config_path, runtime).await
                    })
                } else {
                    self.status = String::from("Error: No config selected");
                    Task::none()
                }
            }
            Message::StopProxy => {
                // Stop the clash proxy service
                self.status = String::from("Stopping...");
                let runtime = self.clash_runtime.clone();
                
                Task::future(async move {
                    stop_clash(runtime).await;
                    Message::ProxyStopped
                })
            }
            Message::ProxyStarted(result) => {
                match result {
                    Ok(_) => {
                        self.status = String::from("Running");
                    }
                    Err(e) => {
                        self.status = format!("Error: {}", e);
                    }
                }
                Task::none()
            }
            Message::ProxyStopped => {
                self.status = String::from("Stopped");
                Task::none()
            }
            Message::ConfigSelected(config) => {
                // TODO: Load the selected config file
                self.selected_config = Some(config);
                Task::none()
            }
            Message::ConfigPathChanged(value) => {
                self.config_path_input = value;
                Task::none()
            }
            Message::AddConfigFile => {
                // Add new config file from the input
                if !self.config_path_input.is_empty() {
                    // Check if config with same path already exists
                    let path_exists = self
                        .config_files
                        .iter()
                        .any(|c| c.path == self.config_path_input);

                    if !path_exists {
                        // Extract filename using cross-platform Path API
                        let path = Path::new(&self.config_path_input);
                        let name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("New Config")
                            .to_string();

                        let new_config = ConfigFile::new(name, self.config_path_input.clone());
                        self.config_files.push(new_config.clone());
                        self.selected_config = Some(new_config);
                        self.config_path_input.clear();
                    }
                }
                Task::none()
            }
        }
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
            Space::with_height(10.0),
            config_label,
            config_picker,
            current_config_text,
            Space::with_height(10.0),
            text("Add New Config:").size(14),
            row![config_path_input, add_config_button].spacing(10),
            Space::with_height(10.0),
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

// Helper function to start clash
async fn start_clash(config_path: String, runtime: ClashRuntime) -> Message {
    // Check if already running
    let mut is_running = runtime.is_running.lock().await;
    if *is_running {
        return Message::ProxyStarted(Err("Clash is already running".to_string()));
    }

    // Start clash in a separate thread with HTTP controller enabled
    let config_path_clone = config_path.clone();
    let runtime_clone = runtime.clone();
    
    let result = tokio::task::spawn(async move {
        // Start clash in a background thread
        let handle = tokio::task::spawn_blocking(move || {
            // Use clash-lib to start the clash instance
            let opts = clash_lib::Options {
                config: clash_lib::Config::File(config_path_clone),
                cwd: None,
                rt: Some(clash_lib::TokioRuntime::MultiThread),
                log_file: None,
            };
            
            match clash_lib::start_scaffold(opts) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to start clash: {}", e)),
            }
        });

        // Wait a moment for clash to start
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        // Create HTTP controller to communicate with clash
        // Assuming clash is running on default port 9090
        let controller = ClashController::new_http("http://127.0.0.1:9090".to_string());
        
        // Test connection by trying to get config
        match controller.get_configs().await {
            Ok(_) => {
                // Store controller for future use
                let mut ctrl = runtime_clone.controller.lock().await;
                *ctrl = Some(controller);
                Ok(())
            }
            Err(e) => {
                // If connection fails, clash might not have started properly
                Err(format!("Failed to connect to clash controller: {}", e))
            }
        }
    })
    .await;

    match result {
        Ok(Ok(_)) => {
            *is_running = true;
            Message::ProxyStarted(Ok(()))
        }
        Ok(Err(e)) => Message::ProxyStarted(Err(e)),
        Err(e) => Message::ProxyStarted(Err(format!("Task error: {}", e))),
    }
}

// Helper function to stop clash
async fn stop_clash(runtime: ClashRuntime) {
    let mut is_running = runtime.is_running.lock().await;
    if *is_running {
        // Clear controller
        let mut ctrl = runtime.controller.lock().await;
        *ctrl = None;
        
        // Call clash-lib shutdown
        clash_lib::shutdown();
        *is_running = false;
    }
}
