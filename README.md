# clash-iced

A GUI client for clash-rs built with the iced framework in Rust.

## Features

- Cross-platform GUI built with [iced](https://github.com/iced-rs/iced)
- Proxy configuration interface
- Start/Stop proxy controls
- Port configuration
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

1. Enter your proxy URL in the "Proxy URL" field
2. Configure the port (default: 7890)
3. Click "Start Proxy" to enable the proxy
4. Click "Stop Proxy" to disable the proxy

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

