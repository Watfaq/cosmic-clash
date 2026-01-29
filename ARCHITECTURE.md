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
   - `clash_runtime`: ClashRuntime - Tracks Clash instance state and IPC controller
   - `proxy_info`: String - Display proxy information fetched via IPC

2. **ClashRuntime** - Runtime state wrapper
   - `is_running`: Arc<Mutex<bool>> - Thread-safe flag indicating if Clash is running
   - `controller`: Arc<Mutex<Option<ClashController>>> - IPC controller for communication
   - `process_handle`: Arc<Mutex<Option<u32>>> - Process ID of clash instance

3. **ClashController** - IPC communication layer (in controller.rs module)
   - HTTP/Unix socket client for clash API
   - Methods: get_proxies, set_mode, get_connections, select_proxy, etc.
   - Similar to clash-android controller implementation

4. **ConfigFile** - Configuration file structure
   - `name`: String - Display name of the config file
   - `path`: String - File system path to the config file

5. **Message** - Application messages/events
   - `ProxyUrlChanged(String)` - Fired when proxy URL input changes
   - `PortChanged(String)` - Fired when port input changes
   - `StartProxy` - Fired when Start button is clicked
   - `StopProxy` - Fired when Stop button is clicked
   - `ConfigSelected(ConfigFile)` - Fired when a config file is selected from the dropdown
   - `ConfigPathChanged(String)` - Fired when the config path input changes
   - `AddConfigFile` - Fired when the Add Config button is clicked
   - `ProxyStarted(Result<(), String>)` - Fired when Clash starts (success or error)
   - `ProxyStopped` - Fired when Clash stops
   - `RefreshProxies` - Fired when Refresh button is clicked
   - `ProxiesUpdated(Result<String, String>)` - Fired with proxy info from IPC

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
- **clash-lib** - Clash proxy core library from clash-rs
- **tokio** - Async runtime for IPC operations
- **hyper** / **hyper-util** - HTTP client for IPC communication
- **hyperlocal** - Unix domain socket support for IPC (Unix platforms)
- **serde** / **serde_json** - Serialization for IPC messages

## Key Features Implementation

### Config File Switching

The application now supports switching between different Clash configuration files:

1. **Pick List Widget** - Dropdown menu showing available config files
2. **Dynamic Addition** - Users can add new config files by entering the path
3. **Config Display** - Shows the full path of the currently selected config
4. **Pre-configured Options** - Includes Default, Home, and Custom config paths

When a user selects a different config file from the dropdown, the application updates its state to reflect the new selection.

### Clash Proxy Integration with IPC

The application integrates with clash-lib through IPC (Inter-Process Communication), similar to clash-android architecture:

1. **Separate Process Architecture** - Clash runs in a separate thread with HTTP controller enabled
2. **IPC Controller** - ClashController module handles communication via HTTP/Unix socket
3. **Async Communication** - All IPC calls are asynchronous, non-blocking
4. **API Methods** - get_proxies, set_mode, get_connections, select_proxy, etc.

**Implementation Details:**
- `ClashController` struct wraps HTTP client for IPC
- Supports both Unix domain sockets (Linux/macOS) and HTTP (all platforms)
- `ClashRuntime` struct tracks controller instance and running state
- `start_clash` spawns clash-lib in background thread, creates controller
- `stop_clash` cleans up controller and shuts down clash-lib
- `refresh_proxies` fetches proxy info via IPC and updates UI

**IPC Flow:**
1. User clicks "Start Proxy"
2. App spawns clash-lib in background thread with controller enabled
3. Creates ClashController pointing to clash HTTP API (port 9090)
4. Controller communicates with clash via HTTP requests
5. UI updates with information fetched through IPC
6. "Refresh" button fetches latest proxy info via IPC

## Future Enhancements

Potential features to add:
- Actual Clash proxy integration
- Configuration file loading/saving
- System tray integration
- Proxy rules management
- Connection statistics
- Log viewing
- Multiple profile support
