use crate::journal::{Journal, JournalEntry};
use chrono::{DateTime, Local, Utc};
use std::collections::VecDeque;
use tui_textarea::TextArea;

pub struct App<'a> {
    pub textarea: TextArea<'a>,
    pub logs: Vec<JournalEntry>,
    pub activity_stream: VecDeque<u32>, // Keystrokes per second/tick
    pub focus_level: f64,
    pub last_activity: DateTime<Local>,
    pub alert_active: bool,
    pub should_quit: bool,
    pub journal: Journal,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let mut textarea = TextArea::default();
        textarea.set_placeholder_text("Add log entry...");
        textarea.set_block(
            ratatui::widgets::Block::default()
                .borders(ratatui::widgets::Borders::ALL)
                .title("Input"),
        );

        let journal = Journal::new();
        let mut logs = journal.load_all().unwrap_or_default();
        logs.sort_by_key(|e| e.timestamp);

        Self {
            textarea,
            logs,
            activity_stream: VecDeque::with_capacity(100),
            focus_level: 100.0,
            last_activity: Local::now(),
            alert_active: false,
            should_quit: false,
            journal,
        }
    }

    pub fn on_tick(&mut self) {
        // Update logic here (e.g. decay focus level if no activity)
        let now = Local::now();
        let diff = now.signed_duration_since(self.last_activity).num_seconds();

        if diff > 10 {
            self.focus_level = (self.focus_level - 0.5).max(0.0);
        }

        if diff > 30 {
            self.alert_active = true;
        } else {
            self.alert_active = false;
        }
    }

    pub fn add_log(&mut self, content: String) {
        let entry = JournalEntry {
            timestamp: Utc::now(),
            content,
        };
        if let Err(e) = self.journal.save(&entry) {
            eprintln!("Failed to save journal: {}", e);
        }
        self.logs.push(entry);
    }

    pub fn register_activity(&mut self) {
        self.last_activity = Local::now();
        self.focus_level = (self.focus_level + 1.0).min(100.0);
        self.alert_active = false;
    }
}
