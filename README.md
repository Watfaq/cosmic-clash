# clash-iced

A GUI client for clash-rs built with the iced framework in Rust.

## Features

- Cross-platform GUI built with [iced](https://github.com/iced-rs/iced)
- **Config file switching** - Select and switch between different Clash configuration files
- **Clash proxy integration** - Uses clash-lib to run actual Clash proxy instances
- **IPC Communication** - UI and clash-lib communicate via IPC (similar to clash-android)
- **Proxy information display** - View proxy groups and nodes
- **Real-time refresh** - Fetch latest proxy status via IPC
- Proxy configuration interface
- Start/Stop proxy controls
- Port configuration
- Add custom config files dynamically
- Clean and modern user interface

## Building

### Prerequisites

- Rust nightly toolchain (automatically configured via rust-toolchain.toml)
- System dependencies for iced:
  - Linux: `libxkbcommon-x11-0`, `libvulkan1` or `mesa-vulkan-drivers`, `protobuf-compiler`
  - Windows: No additional dependencies (protobuf-compiler for building)
  - macOS: No additional dependencies (protobuf-compiler for building)

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/Watfaq/clash-iced.git
cd clash-iced

# Build the project
cargo build --release

# Run the application
cargo run --release
```

## Usage

1. **Select a config file** from the dropdown menu or add a new one:
   - Choose from pre-configured options (Default, Home, Custom)
   - Or enter a custom config file path and click "Add Config"
   - Make sure the config file exists and is a valid Clash configuration
2. The current config file path is displayed below the selector
3. Enter your proxy URL in the "Proxy URL" field (optional)
4. Configure the port (default: 7890, optional)
5. Click "Start Proxy" to enable the proxy
   - The application will start a Clash proxy instance using the selected config
   - Status will change to "Running" when successful
   - Proxy information will automatically be fetched via IPC
6. Click "Refresh" to update proxy information from the running instance
7. Click "Stop Proxy" to disable the proxy
   - The Clash instance will be shut down
   - Status will change to "Stopped"

**Note**: The application uses IPC to communicate with the clash instance, similar to clash-android architecture.

## Development

```bash
# Build in debug mode
cargo build

# Run in debug mode
cargo run

# Check the code
cargo check

# Format the code
cargo fmt

# Run lints
cargo clippy
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

