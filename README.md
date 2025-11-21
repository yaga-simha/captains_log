# Captain's Log - Lockin Wesley

![Captain's Log Logo](/captains_log.png)


A developer productivity suite and TUI app that tracks keyboard activity, allows journaling, and keeps you focused.

## Features

-   **Global Input Tracking**: Uses `evdev` to monitor keyboard activity across the entire system (works seamlessly on Wayland/Hyprland/X11).
-   **Activity Visualization**:
    -   **Symmetrical Waveform**: A real-time, horizontally symmetrical "audio visualizer" style chart.
    -   **Heatmap Coloring**: Dynamic color coding based on typing intensity:
        -   **Blue**: Slow (< 120 LPM)
        -   **Green**: Medium (120 - 300 LPM)
        -   **Yellow**: Fast (300 - 480 LPM)
        -   **Red**: Very Fast (> 480 LPM)
    -   **Rolling Smoothing**: Uses a 1-second rolling window for smooth visual transitions.
-   **Real-Time Metrics**:
    -   **WPM (Words Per Minute)** & **LPM (Letters Per Minute)** calculated continuously.
    -   **Focus Level**: A percentage gauge that fills as you type and decays when idle.
-   **Inactivity Alerts**: A visual "ALERT" popup warns you if your activity drops too low, helping you stay "locked in".
-   **Journaling System**:
    -   Write and save log entries directly within the TUI.
    -   **Persistence**: Entries are saved as timestamped JSON files in the `journals/` directory.
    -   **Log Viewer**: Scrollable list of past entries with local timestamps.
-   **Cyberpunk Aesthetics**: Neon borders, bold text, and a terminal-centric design optimized for Alacritty/Ghostty.

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

**TL;DR:**
-   You can do whatever you want with this code.
-   Just remember my name (**yaga-simha**) every once in a while.
-   **Disclaimer**: I have no responsibility for what this app does to you or your system. Use at your own risk.
