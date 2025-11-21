use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JournalEntry {
    pub timestamp: DateTime<Utc>,
    pub content: String,
}

pub struct Journal {
    pub path: PathBuf,
}

impl Journal {
    pub fn new() -> Self {
        Journal { path: PathBuf::from("journals") }
    }

    pub fn save(&self, entry: &JournalEntry) -> std::io::Result<()> {
        use std::fs::{create_dir_all, File, OpenOptions};
        use std::io::Write;
        use serde_json::json;

        create_dir_all(&self.path)?;
        let filename = self.path.join(format!("{}.json", Utc::now().format("%Y-%m-%d-%H-%M-%S")));
        let mut file = File::create(filename)?;
        let json = serde_json::to_string(entry)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn load_all(&self) -> std::io::Result<Vec<JournalEntry>> {
        use std::fs;
        let mut entries = Vec::new();
        if !self.path.exists() {
            return Ok(entries);
        }
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(entry.path())?;
                let entry: JournalEntry = serde_json::from_str(&content)?;
                entries.push(entry);
            }
        }
        Ok(entries)
    }
}
