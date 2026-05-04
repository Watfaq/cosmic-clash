// SPDX-License-Identifier: AGPL3.0

use std::{
	path::PathBuf,
	process::{Child, Command},
};

use tracing;

#[derive(Debug)]
pub struct SidecarManager {
	child: Option<Child>,
	binary_path: PathBuf,
	work_dir: PathBuf,
	config_path: PathBuf,
}

impl SidecarManager {
	pub fn new(binary_path: PathBuf, work_dir: PathBuf, config_path: PathBuf) -> Self {
		Self {
			child: None,
			binary_path,
			work_dir,
			config_path,
		}
	}

	pub fn start(&mut self) -> eyre::Result<()> {
		if self.child.is_some() {
			tracing::warn!("Clash sidecar is already running");
			return Ok(());
		}

		tracing::info!(
			"Starting clash sidecar: binary={:?}, work_dir={:?}, config={:?}",
			self.binary_path,
			self.work_dir,
			self.config_path
		);

		let mut cmd = Command::new(&self.binary_path);
		cmd.arg("-d")
			.arg(&self.work_dir)
			.arg("-c")
			.arg(&self.config_path)
			.current_dir(&self.work_dir);

		match cmd.spawn() {
			Ok(child) => {
				tracing::info!("Clash sidecar started with pid {:?}", child.id());
				self.child = Some(child);
				Ok(())
			}
			Err(err) => {
				tracing::error!("Failed to start clash sidecar: {}", err);
				Err(err.into())
			}
		}
	}

	pub fn stop(&mut self) -> eyre::Result<()> {
		if let Some(mut child) = self.child.take() {
			match child.kill() {
				Ok(_) => {
					tracing::info!("Clash sidecar killed successfully");
				}
				Err(err) => {
					tracing::error!("Failed to kill clash sidecar: {}", err);
					return Err(err.into());
				}
			}
		}
		Ok(())
	}

	pub fn is_running(&self) -> bool {
		self.child.as_ref().map(|c| c.id()).is_some()
	}
}
