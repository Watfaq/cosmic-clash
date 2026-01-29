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
┃    Config File:                                          ┃
┃    ┌─────────────────────────────────────────────────┐  ┃
┃    │ Default                                      ▼  │  ┃
┃    └─────────────────────────────────────────────────┘  ┃
┃    Path: /etc/clash/config.yaml                         ┃
┃                                                           ┃
┃    Add New Config:                                       ┃
┃    ┌───────────────────────────┐ ┌──────────────┐      ┃
┃    │ Enter config file path... │ │  Add Config  │      ┃
┃    └───────────────────────────┘ └──────────────┘      ┃
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
  - **Config file dropdown**: Pick list to select from available config files
  - **Config path display**: Shows the full path of the selected config file
  - **Add config section**: Text input and button to add new config files
  - Text input for Proxy URL with placeholder text
  - Text input for Port (pre-filled with "7890")
  - Two buttons for starting and stopping the proxy

## Interaction Flow

### Config File Management

1. User can select from pre-configured config files via the dropdown:
   - **Default**: /etc/clash/config.yaml
   - **Home**: ~/.config/clash/config.yaml
   - **Custom**: ./clash-config.yaml
2. User can add a new config file:
   - Enter the full path to a config file in the text input
   - Click "Add Config" button
   - The new config is added to the list and automatically selected
   - The file name is extracted from the path as the display name
3. The current config file path is displayed below the picker

### Proxy Control

1. User enters a proxy URL in the first text field
2. User can modify the port number (default: 7890)
3. User clicks "Start Proxy" to begin proxy service (currently updates status to "Running")
4. User clicks "Stop Proxy" to stop proxy service (currently updates status to "Stopped")

## Current Implementation Status

This is a UI prototype with config file switching functionality. The following features are planned for future implementation:
- [ ] Actual Clash proxy integration
- [ ] Load and parse config file contents
- [ ] URL validation
- [ ] Port number validation
- [ ] Configuration persistence
- [ ] Error handling and user feedback
- [x] **Config file selection and switching**
- [x] **Dynamic addition of config files**
