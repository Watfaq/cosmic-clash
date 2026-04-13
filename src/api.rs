// SPDX-License-Identifier: AGPL3.0

use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ClashApi {
	client: reqwest::Client,
	base_url: String,
	secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Version {
	pub meta: Option<bool>,
	pub version: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DelayRecord {
	pub time: String,
	pub delay: u64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ProxySummary {
	pub name: Option<String>,
	#[serde(rename = "type")]
	pub proxy_type: Option<String>,
	pub now: Option<String>,
	pub history: Option<Vec<DelayRecord>>,
	pub all: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ProxiesResponse {
	pub proxies: HashMap<String, ProxySummary>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Traffic {
	pub up: u64,
	pub down: u64,
}

#[derive(Debug, Clone, Serialize)]
struct ConfigReloadRequest<'a> {
	path: &'a str,
}

impl ClashApi {
	pub fn new(base_url: String, secret: Option<String>) -> Self {
		Self {
			client: reqwest::Client::builder()
				.timeout(std::time::Duration::from_secs(5))
				.build()
				.unwrap_or_default(),
			base_url,
			secret,
		}
	}

	fn build_request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
		let url = format!("{}{}", self.base_url, path);
		let mut req = self.client.request(method, &url);
		if let Some(secret) = &self.secret {
			req = req.header("Authorization", format!("Bearer {}", secret));
		}
		req
	}

	pub async fn version(&self) -> eyre::Result<Version> {
		let resp = self
			.build_request(reqwest::Method::GET, "/version")
			.send()
			.await?;
		Ok(resp.json().await?)
	}

	pub async fn proxies(&self) -> eyre::Result<ProxiesResponse> {
		let resp = self
			.build_request(reqwest::Method::GET, "/proxies")
			.send()
			.await?;
		Ok(resp.json().await?)
	}

	pub async fn reload_config(&self, path: &str) -> eyre::Result<()> {
		let body = ConfigReloadRequest { path };
		let resp = self
			.build_request(reqwest::Method::PUT, "/configs")
			.json(&body)
			.send()
			.await?;
		if !resp.status().is_success() {
			let text = resp.text().await.unwrap_or_default();
			return Err(eyre::eyre!("Failed to reload config: {}", text));
		}
		Ok(())
	}

	pub async fn traffic(&self) -> eyre::Result<Traffic> {
		let resp = self
			.build_request(reqwest::Method::GET, "/traffic")
			.send()
			.await?;
		let text = resp.text().await?;
		// Clash traffic endpoint may return SSE stream; extract first data line
		let json_line = text.lines().find(|l| !l.trim().is_empty()).unwrap_or("{}");
		let json_line = json_line.strip_prefix("data:").unwrap_or(json_line).trim();
		Ok(serde_json::from_str(json_line)?)
	}
}
