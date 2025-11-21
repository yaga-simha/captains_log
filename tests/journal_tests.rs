use captains_log::journal::{Journal, JournalEntry};
use chrono::Utc;
use tempfile::tempdir;

#[test]
fn test_journal_new() {
    let journal = Journal::new();
    assert_eq!(journal.path.to_str(), Some("journals"));
}

#[test]
fn test_journal_save_and_load() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let journal_path = dir.path().join("test_journals");
    let journal = Journal {
        path: journal_path.clone(),
    };

    // Test saving an entry
    let entry1 = JournalEntry {
        timestamp: Utc::now(),
        content: "Test content 1".to_string(),
    };
    journal.save(&entry1)?;
    std::thread::sleep(std::time::Duration::from_millis(100)); // Add a small delay


    // Verify file exists and content is correct
    let entries = journal.load_all()?;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].content, entry1.content);
    assert_eq!(entries[0].timestamp.to_rfc3339(), entry1.timestamp.to_rfc3339());

    // Test saving another entry
    let entry2 = JournalEntry {
        timestamp: Utc::now(),
        content: "Test content 2".to_string(),
    };
    journal.save(&entry2)?;

    // Verify two entries are loaded
    let entries = journal.load_all()?;
    assert_eq!(entries.len(), 2);
    // Note: Order of entries loaded from directory might not be guaranteed,
    // so we just check if both contents are present.
    let contents: Vec<String> = entries.iter().map(|e| e.content.clone()).collect();
    assert!(contents.contains(&entry1.content));
    assert!(contents.contains(&entry2.content));

    Ok(())
}

#[test]
fn test_journal_load_empty_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let journal_path = dir.path().join("empty_journals");
    let journal = Journal {
        path: journal_path,
    };

    let entries = journal.load_all()?;
    assert!(entries.is_empty());
    Ok(())
}

#[test]
fn test_journal_save_directory_creation() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let journal_path = dir.path().join("non_existent_journals");
    let journal = Journal {
        path: journal_path.clone(),
    };

    let entry = JournalEntry {
        timestamp: Utc::now(),
        content: "Content for new dir".to_string(),
    };
    journal.save(&entry)?;

    assert!(journal_path.exists());
    assert!(journal_path.is_dir());
    Ok(())
}
