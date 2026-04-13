//! Cosmic Clash - GUI for clash-rs

pub mod api;
pub mod app;
pub mod config;
pub mod i18n;
pub mod log;
pub mod pages;
pub mod sidecar;

// Re-exports
pub use app::{AppModel, Message, SettingField};
pub use config::Config;
