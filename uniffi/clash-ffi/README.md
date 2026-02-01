# Clash FFI

Foreign Function Interface (FFI) bindings for clash-rs core management functionality. This crate provides a C-compatible API that can be used to integrate clash-rs into various applications.

## Overview

This FFI implementation is based on the [clash-android-ffi](https://github.com/Watfaq/clash-android/tree/main/uniffi/clash-android-ffi) reference implementation and provides the following functionality:

- **Core Management**: Start, stop, and configure clash-rs instances
- **Controller API**: Interact with running clash instances via HTTP API
  - Get/set proxy modes (rule, global, direct)
  - Manage proxy selections
  - Query proxy delays and connections
  - Monitor memory usage
- **Utilities**: Download files with progress callbacks
- **Logging**: Configurable logging infrastructure

## Building

This crate requires Rust nightly due to clash-lib dependencies:

```bash
rustup install nightly
cd uniffi/clash-ffi
rustup override set nightly
cargo build --release
```

## Language Bindings

The FFI uses [UniFFI](https://mozilla.github.io/uniffi-rs/) to generate bindings for multiple languages:

### Kotlin
```bash
cargo uniffi-bindgen generate src/lib.rs --language kotlin
```

### Swift  
```bash
cargo uniffi-bindgen generate src/lib.rs --language swift
```

### Python
```bash
cargo uniffi-bindgen generate src/lib.rs --language python
```

## Core Functions

### Initialization
```rust
// Initialize the clash library with logging level
init_clash(log_level: String)
```

### Configuration
```rust
// Verify a configuration file is valid
verify_config(config_path: &str) -> Result<String, ClashError>
```

### Running Clash
```rust
// Start clash with configuration
run_clash(
    config_path: String,
    work_dir: String, 
    over: ProfileOverride
) -> Result<FinalProfile, ClashError>

// Shutdown clash
shutdown()
```

### Controller API

Create a controller to interact with a running clash instance:

```rust
let controller = ClashController::new("/path/to/clash.sock");

// Get all proxies
let proxies = controller.get_proxies().await?;

// Select a proxy
controller.select_proxy("ProxyGroup", "ProxyName").await?;

// Get proxy delay
let delay = controller.get_proxy_delay("ProxyName", None, None).await?;

// Get connections
let connections = controller.get_connections().await?;

// Set mode
controller.set_mode(Mode::Global).await?;
```

### File Downloads

Download files with optional progress tracking:

```rust
// Simple download
download_file(
    url: String,
    output_path: String,
    user_agent: Option<String>,
    proxy_url: Option<String>
) -> Result<DownloadResult, ClashError>

// Download with progress callback
download_file_with_progress(
    url: String,
    output_path: String,
    user_agent: Option<String>,
    proxy_url: Option<String>,
    progress_callback: Option<Box<dyn DownloadProgressCallback>>
) -> Result<DownloadResult, ClashError>
```

## Configuration

The `ProfileOverride` struct allows customizing clash behavior:

```rust
ProfileOverride {
    allow_lan: bool,          // Allow LAN connections (default: false)
    mixed_port: u16,          // Mixed proxy port (default: 7890)
    http_port: Option<u16>,   // HTTP proxy port (optional)
    socks_port: Option<u16>,  // SOCKS proxy port (optional)
    fake_ip: bool,            // Enable fake IP mode (default: false)
    fake_ip_range: String,    // Fake IP range (default: "198.18.0.2/16")
    ipv6: bool,               // Enable IPv6 (default: true)
}
```

## Error Handling

All errors are represented by the `ClashError` enum:

- `ConfigError`: Configuration-related errors
- `IoError`: I/O operation errors
- `ParseError`: Parsing errors
- `RuntimeError`: Runtime errors

## Platform Support

- **Linux**: Full support (uses Unix domain sockets)
- **macOS**: Full support (uses Unix domain sockets)
- **Windows**: Partial support (Unix domain socket features not available)

## Dependencies

Key dependencies include:
- `clash-lib`: Core clash-rs functionality
- `uniffi`: FFI binding generation
- `tokio`: Async runtime
- `hyper`: HTTP client
- `serde`: Serialization

## License

AGPL-3.0

## Credits

Based on the implementation from [clash-android](https://github.com/Watfaq/clash-android).
