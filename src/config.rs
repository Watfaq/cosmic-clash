// SPDX-License-Identifier: AGPL3.0

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
	pub clash_binary_path: Option<String>,
	pub config_dir: Option<String>,
	pub active_profile: Option<String>,
	pub api_port: u16,
	pub api_secret: Option<String>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			clash_binary_path: None,
			config_dir: None,
			active_profile: None,
			api_port: 9090,
			api_secret: None,
		}
	}
}

impl Config {
	pub fn config_dir(&self) -> PathBuf {
		self.config_dir
			.as_ref()
			.map(PathBuf::from)
			.unwrap_or_else(|| dirs::config_dir().unwrap_or_else(|| PathBuf::from(".")).join("clash"))
	}

	pub fn clash_binary(&self) -> PathBuf {
		self.clash_binary_path
			.as_ref()
			.map(PathBuf::from)
			.unwrap_or_else(detect_default_binary)
	}

	pub fn api_url(&self) -> String {
		format!("http://127.0.0.1:{}", self.api_port)
	}

	fn config_path() -> PathBuf {
		dirs::config_dir()
			.unwrap_or_else(|| PathBuf::from("."))
			.join("cosmic-clash")
			.join("config.json")
	}

	pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
		let path = Self::config_path();
		if let Some(parent) = path.parent() {
			std::fs::create_dir_all(parent)?;
		}
		let json = serde_json::to_string_pretty(self)?;
		std::fs::write(path, json)?;
		Ok(())
	}

	pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
		let path = Self::config_path();
		if !path.exists() {
			return Ok(Self::default());
		}
		let json = std::fs::read_to_string(path)?;
		let config = serde_json::from_str(&json)?;
		Ok(config)
	}
}

fn detect_default_binary() -> PathBuf {
	// Try to find clash binary in common locations
	let candidates = [
		"/usr/local/bin/clash",
		"/usr/bin/clash",
		"clash", // Try PATH
	];

	for candidate in &candidates {
		if std::path::Path::new(candidate).exists() {
			return PathBuf::from(candidate);
		}
	}

	// Fallback
	PathBuf::from("clash")
}
