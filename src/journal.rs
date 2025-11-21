use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JournalEntry {
    pub timestamp: DateTime<Utc>,
    pub content: String,
}

pub struct Journal {
    pub path: PathBuf,
}

impl Default for Journal {
    fn default() -> Self {
        Self::new()
    }
}

impl Journal {
    pub fn new() -> Self {
        Journal {
            path: PathBuf::from("journals"),
        }
    }

    pub fn save(&self, entry: &JournalEntry) -> std::io::Result<()> {
        use std::fs::{File, create_dir_all};
        use std::io::Write;

        create_dir_all(&self.path)?;
        let filename = self.path.join(format!(
            "{}.json",
            Utc::now().format("%Y-%m-%d-%H-%M-%S-%3f")
        ));
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
