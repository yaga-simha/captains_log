# Captain's Log - Focus Lock-In TUI

A developer productivity suite and TUI app that tracks keyboard activity, allows journaling, and keeps you focused.

## Features

-   **Keyboard Activity Stream (Braille-Gantt)**: Visualizes your typing intensity in real-time using Braille characters.
-   **Focus Lock-In**: Tracks your focus level based on activity. Alerts you if you slack off.
-   **Journal Logs**: Persistent journaling to keep track of your work sessions.
-   **Cyberpunk Aesthetics**: Designed for advanced terminals (Alacritty, Ghostty).
-   **Global Input Tracking**: Uses `evdev` to track keyboard activity across the entire system (works on Wayland/Hyprland).

## Requirements

-   Linux.
-   **Permissions**: You must be in the `input` group to read keyboard events, or run with `sudo`.
    ```bash
    sudo usermod -aG input $USER
    # Log out and log back in for changes to take effect
    ```

## Usage

```bash
cargo run
# If input tracking doesn't work, try running as root (not recommended for daily use, but verifies functionality):
# sudo -E cargo run
```

-   **Type** to increase focus level.
-   **Enter** to save a journal entry.
-   **F10** to exit.

## Troubleshooting

-   **No Activity Detected**: Ensure you have read permissions for `/dev/input/event*`. Check by running `ls -l /dev/input/event*`. They should be owned by `root:input`.
-   **Wayland Support**: This app uses `evdev` directly, so it works on Wayland compositors like Hyprland without issues, provided permissions are correct.
