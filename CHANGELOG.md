# Changelog

All notable changes to the clash-iced project will be documented in this file.

## [Unreleased]

### Added - Config File Switching Feature

#### New Features
- **Config file selection dropdown**: Users can now select from multiple Clash configuration files using a pick list widget
- **Dynamic config addition**: Users can add new configuration files by entering their path
- **Config path display**: Shows the full path of the currently selected configuration file
- **Pre-configured options**: Comes with three example config paths:
  - Default: `/etc/clash/config.yaml`
  - Home: `~/.config/clash/config.yaml`
  - Custom: `./clash-config.yaml`

#### Technical Improvements
- Added `ConfigFile` struct with Hash, PartialEq, Eq, Clone, and Debug traits
- Extended `ClashApp` state with config management fields
- Added three new Message variants for config file operations
- Implemented duplicate path validation
- Cross-platform path handling using `std::path::Path`
- Replaced text spacers with proper `Space` widgets for better semantics

#### UI Components
- Pick list dropdown for config selection
- Text input field for adding new config paths
- "Add Config" button to add new configurations
- Display of currently selected config file path

#### Documentation
- Updated README.md with config switching feature description
- Updated ARCHITECTURE.md with new components and UI layout
- Updated UI_PREVIEW.md with ASCII mockup showing new features
- Added inline code documentation and TODO comments

#### Code Quality
- All code passes `cargo clippy -- -D warnings` without warnings
- Properly formatted with `cargo fmt`
- Successfully builds in both debug and release modes

### Changed
- Window layout reorganized to accommodate config file selection UI
- UI spacing improved with proper Space widgets instead of empty text elements

## [0.1.0] - Initial Release

### Added
- Basic GUI application using iced framework
- Proxy URL configuration input
- Port configuration input (default: 7890)
- Start/Stop proxy controls
- Status display
- Cross-platform support (Linux, Windows, macOS)
