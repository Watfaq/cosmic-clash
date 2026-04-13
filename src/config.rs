// SPDX-License-Identifier: AGPL3.0

use std::path::PathBuf;

#[derive(Debug, Clone, Eq, PartialEq)]
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

	pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
		// Simplified: just return Ok for now
		Ok(())
	}

	pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
		// Simplified: return default config
		Ok(Self::default())
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