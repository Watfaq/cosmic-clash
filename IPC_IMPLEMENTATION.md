# IPC Implementation Summary

## Overview
Implemented IPC (Inter-Process Communication) between the UI and clash-lib, following the architecture pattern from clash-android (https://github.com/Watfaq/clash-android/blob/main/uniffi/clash-android-ffi/src/controller.rs).

## Architecture

### Separation of Concerns
```
┌─────────────────┐         IPC          ┌──────────────────┐
│   UI (iced)     │ ◄──────────────────► │   clash-lib      │
│                 │    HTTP/Unix Socket   │   (background)   │
│  ClashApp       │                       │                  │
│  controller.rs  │                       │   HTTP Server    │
└─────────────────┘                       └──────────────────┘
```

### Components

#### 1. ClashController (controller.rs)
HTTP/Unix socket client for communicating with clash API:
- **Methods**:
  - `new_http(url)` - Create HTTP-based controller
  - `new_unix(socket_path)` - Create Unix socket-based controller (Unix only)
  - `get_proxies()` - Fetch all proxy groups and nodes
  - `select_proxy(group, proxy)` - Select proxy in a group
  - `get_configs()` - Get current configuration
  - `set_mode(mode)` - Set proxy mode (rule/global/direct)
  - `get_mode()` - Get current proxy mode
  - `get_connections()` - Get active connections

- **Data Structures**:
  - `Proxy` - Proxy group/node information
  - `Mode` - Enum for proxy modes
  - `Connection` - Connection information
  - `ConfigResponse` - Configuration response

#### 2. ClashRuntime (main.rs)
Extended runtime state tracking:
```rust
struct ClashRuntime {
    is_running: Arc<Mutex<bool>>,
    controller: Arc<Mutex<Option<ClashController>>>,
    process_handle: Arc<Mutex<Option<u32>>>,
}
```

#### 3. Updated Message Flow
- `StartProxy` → spawns clash-lib → creates controller → `ProxyStarted`
- `RefreshProxies` → controller.get_proxies() → `ProxiesUpdated`
- `StopProxy` → clears controller → shuts down clash → `ProxyStopped`

## Implementation Details

### Starting Clash with IPC
1. User clicks "Start Proxy"
2. App spawns clash-lib in background thread with controller enabled
3. Waits 500ms for clash to initialize
4. Creates `ClashController` pointing to HTTP API (default: http://127.0.0.1:9090)
5. Tests connection by fetching config
6. Stores controller in ClashRuntime
7. Auto-refreshes proxy info

### Communication Flow
```
UI Action → Message → async Task → ClashController → HTTP/Socket
                                         ↓
                                    clash-lib API
                                         ↓
Response ← Message ← Result ← HTTP Response
```

### Supported Platforms
- **Linux/macOS**: Both HTTP and Unix domain sockets
- **Windows**: HTTP only

## Dependencies Added
- `hyper` - HTTP client/server framework
- `hyper-util` - HTTP utilities
- `http-body-util` - HTTP body utilities
- `hyperlocal` - Unix domain socket support (Unix only)
- `serde` / `serde_json` - Serialization
- `urlencoding` - URL encoding

## Benefits of IPC Architecture

1. **Separation**: UI and proxy logic are decoupled
2. **Stability**: Crash in one component doesn't affect the other
3. **Flexibility**: Can restart/reconfigure clash without restarting UI
4. **Compatibility**: Matches clash-android architecture pattern
5. **Scalability**: Easy to add more IPC methods as needed

## Similar to clash-android
The implementation closely follows the clash-android pattern:
- Uses HTTP/Unix socket for IPC
- Similar data structures (Proxy, Mode, Connection, etc.)
- Same API endpoints (/proxies, /configs, /connections, etc.)
- Async communication pattern
- Separate controller module

## Testing Notes
To test the IPC implementation:
1. Ensure clash config has controller enabled (port 9090)
2. Start clash via the UI
3. Click "Refresh" to fetch proxy info
4. Check that proxy names and types are displayed
5. Verify that stopping clash clears the info

## Future Enhancements
- Add more IPC methods (logs, rules, etc.)
- Display connections in real-time
- Allow proxy selection via UI
- Add mode switching UI controls
- Add delay testing for proxies
