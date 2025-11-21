use evdev::{Device, InputEventKind, Key};
use std::fs;
use std::sync::mpsc::Sender;
use std::thread;

pub enum MonitorEvent {
    Activity,
}

pub fn start_monitor(tx: Sender<MonitorEvent>) {
    thread::spawn(move || {
        // Attempt to use evdev to find all keyboard devices
        let mut devices = Vec::new();

        // Scan /dev/input/event*
        if let Ok(entries) = fs::read_dir("/dev/input") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(fname) = path.file_name().and_then(|s| s.to_str())
                    && fname.starts_with("event")
                        && let Ok(device) = Device::open(&path) {
                            // Check if it has keys (is a keyboard)
                            // This is a heuristic: check if it supports KEY_A or KEY_ENTER
                            if device.supported_keys().is_some_and(|keys| {
                                keys.contains(Key::KEY_ENTER) || keys.contains(Key::KEY_A)
                            }) {
                                devices.push(path);
                            }
                        }
            }
        }

        if devices.is_empty() {
            eprintln!(
                "No keyboard devices found via evdev. Ensure you have permissions (input group)."
            );
            // Fallback to rdev if evdev fails (though rdev on wayland is also flaky)
            start_rdev_monitor(tx.clone());
            return;
        }

        // Spawn a thread for each device
        for path in devices {
            let tx_clone = tx.clone();
            thread::spawn(move || {
                if let Ok(mut device) = Device::open(&path) {
                    loop {
                        match device.fetch_events() {
                            Ok(events) => {
                                for event in events {
                                    if let InputEventKind::Key(_) = event.kind() {
                                        // Only count key presses (value == 1), ignore releases (0) and repeats (2)
                                        if event.value() == 1 {
                                            let _ = tx_clone.send(MonitorEvent::Activity);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error reading device {:?}: {}", path, e);
                                break;
                            }
                        }
                    }
                }
            });
        }
    });
}

fn start_rdev_monitor(tx: Sender<MonitorEvent>) {
    use rdev::{EventType, listen};
    thread::spawn(move || {
        let callback = move |event: rdev::Event| match event.event_type {
            EventType::KeyPress(_) | EventType::MouseMove { .. } | EventType::ButtonPress(_) => {
                let _ = tx.send(MonitorEvent::Activity);
            }
            _ => {}
        };

        if let Err(error) = listen(callback) {
            eprintln!("Input monitoring error (rdev): {:?}", error);
        }
    });
}
