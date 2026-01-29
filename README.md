# clash-iced

A GUI client for clash-rs built with the iced framework in Rust.

## Features

- Cross-platform GUI built with [iced](https://github.com/iced-rs/iced)
- **Config file switching** - Select and switch between different Clash configuration files
- Proxy configuration interface
- Start/Stop proxy controls
- Port configuration
- Add custom config files dynamically
- Clean and modern user interface

## Building

### Prerequisites

- Rust toolchain (1.75 or later)
- System dependencies for iced:
  - Linux: `libxkbcommon-x11-0`, `libvulkan1` or `mesa-vulkan-drivers`
  - Windows: No additional dependencies
  - macOS: No additional dependencies

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
2. The current config file path is displayed below the selector
3. Enter your proxy URL in the "Proxy URL" field
4. Configure the port (default: 7890)
5. Click "Start Proxy" to enable the proxy
6. Click "Stop Proxy" to disable the proxy

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

