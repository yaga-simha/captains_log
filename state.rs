pub struct AppState {
    /// Current active screen or panel.
    pub screen: AppScreen,
    /// List of journal entries.
    pub entries: Vec<JournalEntry>,
}

/// Represents the top‑level UI screens.
#[derive(Debug, PartialEq, Eq)]
pub enum AppScreen {
    /// Editing a journal entry.
    Editor,
    /// List of entries.
    List,
    /// View full entry.
    Detail,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            screen: AppScreen::Editor,
            entries: Vec::new(),
        }
    }
}
