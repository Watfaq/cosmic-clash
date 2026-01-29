# Architecture

## Overview

clash-iced is a GUI application built with the iced framework for managing Clash proxy configurations.

## Application Structure

### Main Components

1. **ClashApp** - The main application state
   - `proxy_url`: String - Stores the proxy URL configuration
   - `port`: String - Stores the port configuration (default: 7890)
   - `status`: String - Current proxy status (Running/Stopped/Starting/Stopping/Error)
   - `config_files`: Vec<ConfigFile> - List of available configuration files
   - `selected_config`: Option<ConfigFile> - Currently selected config file
   - `config_path_input`: String - Input field for adding new config paths
   - `clash_runtime`: ClashRuntime - Tracks Clash instance state

2. **ClashRuntime** - Runtime state wrapper
   - `is_running`: Arc<Mutex<bool>> - Thread-safe flag indicating if Clash is running

3. **ConfigFile** - Configuration file structure
   - `name`: String - Display name of the config file
   - `path`: String - File system path to the config file

4. **Message** - Application messages/events
   - `ProxyUrlChanged(String)` - Fired when proxy URL input changes
   - `PortChanged(String)` - Fired when port input changes
   - `StartProxy` - Fired when Start button is clicked
   - `StopProxy` - Fired when Stop button is clicked
   - `ConfigSelected(ConfigFile)` - Fired when a config file is selected from the dropdown
   - `ConfigPathChanged(String)` - Fired when the config path input changes
   - `AddConfigFile` - Fired when the Add Config button is clicked
   - `ProxyStarted(Result<(), String>)` - Fired when Clash starts (success or error)
   - `ProxyStopped` - Fired when Clash stops

### UI Layout

```
┌────────────────────────────────────────┐
│        Clash Iced                      │
│                                        │
│  Status: Stopped                       │
│                                        │
│  Config File:                          │
│  ┌────────────────────────────────┐   │
│  │ Default                    ▼   │   │
│  └────────────────────────────────┘   │
│  Path: /etc/clash/config.yaml         │
│                                        │
│  Add New Config:                       │
│  ┌──────────────────┐ ┌──────────┐   │
│  │ Enter path...    │ │Add Config│   │
│  └──────────────────┘ └──────────┘   │
│                                        │
│  Proxy URL:                            │
│  ┌────────────────────────────────┐   │
│  │ Enter proxy URL...             │   │
│  └────────────────────────────────┘   │
│                                        │
│  Port:                                 │
│  ┌────────────────────────────────┐   │
│  │ 7890                           │   │
│  └────────────────────────────────┘   │
│                                        │
│  ┌──────────┐  ┌──────────┐          │
│  │  Start   │  │  Stop    │          │
│  │  Proxy   │  │  Proxy   │          │
│  └──────────┘  └──────────┘          │
│                                        │
└────────────────────────────────────────┘
```

## Technologies Used

- **Rust** - Systems programming language
- **iced** (v0.13) - Cross-platform GUI framework
  - iced_core - Core functionality
  - iced_widget - UI widgets (buttons, text inputs, pick lists, etc.)
  - iced_winit - Window creation and event handling
  - iced_wgpu - Hardware-accelerated rendering backend

## Key Features Implementation

### Config File Switching

The application now supports switching between different Clash configuration files:

1. **Pick List Widget** - Dropdown menu showing available config files
2. **Dynamic Addition** - Users can add new config files by entering the path
3. **Config Display** - Shows the full path of the currently selected config
4. **Pre-configured Options** - Includes Default, Home, and Custom config paths

When a user selects a different config file from the dropdown, the application updates its state to reflect the new selection. The actual loading of config file contents would be implemented in the future proxy integration.

### Clash Proxy Integration

The application integrates with clash-lib (from https://github.com/Watfaq/clash-rs) to run actual Clash proxy instances:

1. **clash-lib Dependency** - Uses clash-lib as a git dependency
2. **Async Runtime** - Uses Tokio for async operations
3. **Start/Stop Controls** - Actually starts and stops Clash proxy instances
4. **Runtime State** - Tracks whether Clash is running

**Implementation Details:**
- `ClashRuntime` struct wraps the runtime state in an Arc<Mutex<bool>>
- `start_clash` function uses `clash_lib::start_scaffold` to start Clash in a blocking task
- `stop_clash` function calls `clash_lib::shutdown` to stop the running instance
- Uses iced's `Task::future` for async message handling

## Future Enhancements

Potential features to add:
- Actual Clash proxy integration
- Configuration file loading/saving
- System tray integration
- Proxy rules management
- Connection statistics
- Log viewing
- Multiple profile support
