# Changelog

All notable changes to the clash-iced project will be documented in this file.

## [Unreleased]

### Added - IPC Communication Layer

#### New Features
- **IPC Architecture**: Separated UI and clash-lib communication through IPC
- **ClashController Module**: HTTP/Unix socket client for communicating with clash API
- **Proxy Information Display**: UI now shows proxy information fetched via IPC
- **Refresh Button**: Manual refresh of proxy information from running clash instance
- **Automatic Refresh**: Proxy info automatically refreshes when clash starts

#### Technical Implementation
- Added `controller.rs` module with ClashController implementation
- Similar to clash-android controller architecture
- Supports both HTTP (all platforms) and Unix domain sockets (Unix platforms)
- IPC methods: get_proxies, get_configs, set_mode, select_proxy, get_connections
- Modified ClashRuntime to include controller instance
- Start/stop now communicate with clash via IPC instead of direct calls

#### Dependencies
- Added hyper, hyper-util for HTTP client
- Added http-body-util for request/response handling
- Added hyperlocal for Unix domain socket support (Unix only)
- Added serde, serde_json for message serialization
- Added urlencoding for URL encoding

### Added - Clash Proxy Integration

#### New Features
- **Clash-lib Integration**: Integrated clash-lib from https://github.com/Watfaq/clash-rs as a git dependency
- **Actual Proxy Functionality**: Start and stop buttons now actually start and stop Clash proxy instances
- **Async Runtime Support**: Added Tokio async runtime for handling proxy operations
- **Runtime State Tracking**: Track whether Clash is running or stopped
- **Error Handling**: Display errors if Clash fails to start

#### Technical Implementation
- Added clash-lib as git dependency
- Added tokio and tracing dependencies
- Requires Rust nightly toolchain (configured via rust-toolchain.toml)
- Uses `clash_lib::start_scaffold` to start Clash instances
- Uses `clash_lib::shutdown` to stop running instances
- Implements async message handling with iced's `Task::future`
- Added `ClashRuntime` struct to track state
- Added `ProxyStarted` and `ProxyStopped` messages

#### Configuration
- Requires protobuf-compiler for building
- Added rust-toolchain.toml to specify nightly Rust
- Updated .gitignore to exclude runtime files (cache.db, config files)

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
