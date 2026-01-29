# UI Preview

Since the application requires a graphical environment to run, here's what the interface looks like:

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃  Clash Iced - Clash Client                               ┃
┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
┃                                                           ┃
┃    Clash Iced                                            ┃
┃                                                           ┃
┃    Status: Stopped                                       ┃
┃                                                           ┃
┃    Proxy URL:                                            ┃
┃    ┌─────────────────────────────────────────────────┐  ┃
┃    │ Enter proxy URL...                              │  ┃
┃    └─────────────────────────────────────────────────┘  ┃
┃                                                           ┃
┃    Port:                                                 ┃
┃    ┌─────────────────────────────────────────────────┐  ┃
┃    │ 7890                                            │  ┃
┃    └─────────────────────────────────────────────────┘  ┃
┃                                                           ┃
┃    ┌──────────────┐  ┌──────────────┐                   ┃
┃    │ Start Proxy  │  │  Stop Proxy  │                   ┃
┃    └──────────────┘  └──────────────┘                   ┃
┃                                                           ┃
┃                                                           ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

## Window Details

- **Window Size**: 800x600 pixels
- **Title**: "Clash Iced - Clash Client"
- **Components**:
  - Large title text (32pt): "Clash Iced"
  - Status display (20pt): Shows current proxy status
  - Text input for Proxy URL with placeholder text
  - Text input for Port (pre-filled with "7890")
  - Two buttons for starting and stopping the proxy

## Interaction Flow

1. User enters a proxy URL in the first text field
2. User can modify the port number (default: 7890)
3. User clicks "Start Proxy" to begin proxy service (currently updates status to "Running")
4. User clicks "Stop Proxy" to stop proxy service (currently updates status to "Stopped")

## Current Implementation Status

This is a UI prototype. The following features are planned for future implementation:
- [ ] Actual Clash proxy integration
- [ ] URL validation
- [ ] Port number validation
- [ ] Connection status monitoring
- [ ] Configuration persistence
- [ ] Error handling and user feedback
