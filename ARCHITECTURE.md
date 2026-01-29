# Architecture

## Overview

clash-iced is a GUI application built with the iced framework for managing Clash proxy configurations.

## Application Structure

### Main Components

1. **ClashApp** - The main application state
   - `proxy_url`: String - Stores the proxy URL configuration
   - `port`: String - Stores the port configuration (default: 7890)
   - `status`: String - Current proxy status (Running/Stopped)

2. **Message** - Application messages/events
   - `ProxyUrlChanged(String)` - Fired when proxy URL input changes
   - `PortChanged(String)` - Fired when port input changes
   - `StartProxy` - Fired when Start button is clicked
   - `StopProxy` - Fired when Stop button is clicked

### UI Layout

```
┌────────────────────────────────────────┐
│        Clash Iced                      │
│                                        │
│  Status: Stopped                       │
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
  - iced_widget - UI widgets (buttons, text inputs, etc.)
  - iced_winit - Window creation and event handling
  - iced_wgpu - Hardware-accelerated rendering backend

## Future Enhancements

Potential features to add:
- Actual Clash proxy integration
- Configuration file loading/saving
- System tray integration
- Proxy rules management
- Connection statistics
- Log viewing
- Multiple profile support
