//! Recent files management
//!
//! Tracks recently opened files and persists them to disk.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Maximum number of recent files to track
const MAX_RECENT_FILES: usize = 10;

/// Recent files manager
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecentFiles {
    /// List of recent file paths (newest first)
    pub files: Vec<PathBuf>,
}

impl RecentFiles {
    /// Create a new empty recent files list
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Load recent files from disk
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                match std::fs::read_to_string(&path) {
                    Ok(contents) => match serde_json::from_str(&contents) {
                        Ok(recent) => return recent,
                        Err(e) => {
                            tracing::warn!("Failed to parse recent files: {}", e);
                        }
                    },
                    Err(e) => {
                        tracing::warn!("Failed to read recent files: {}", e);
                    }
                }
            }
        }
        Self::new()
    }

    /// Save recent files to disk
    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            // Create parent directory if needed
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    if let Err(e) = std::fs::create_dir_all(parent) {
                        tracing::warn!("Failed to create config directory: {}", e);
                        return;
                    }
                }
            }

            match serde_json::to_string_pretty(self) {
                Ok(contents) => {
                    if let Err(e) = std::fs::write(&path, contents) {
                        tracing::warn!("Failed to save recent files: {}", e);
                    } else {
                        tracing::debug!("Recent files saved to {}", path.display());
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to serialize recent files: {}", e);
                }
            }
        }
    }

    /// Add a file to the recent files list
    pub fn add_file(&mut self, path: PathBuf) {
        // Remove if already in list (to move to front)
        self.files.retain(|p| p != &path);
        // Add to front
        self.files.insert(0, path);
        // Trim to max size
        self.files.truncate(MAX_RECENT_FILES);
        // Save to disk
        self.save();
    }

    /// Remove a file from the recent files list
    pub fn remove_file(&mut self, path: &PathBuf) {
        self.files.retain(|p| p != path);
        self.save();
    }

    /// Clear all recent files
    pub fn clear(&mut self) {
        self.files.clear();
        self.save();
    }

    /// Get the list of recent files
    pub fn get_files(&self) -> &[PathBuf] {
        &self.files
    }

    /// Get config file path
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut p| {
            p.push("testruct");
            p.push("recent_files.json");
            p
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_file() {
        let mut recent = RecentFiles::new();
        recent.files.push(PathBuf::from("/path/to/file1.json"));
        recent.files.push(PathBuf::from("/path/to/file2.json"));

        // Adding an existing file should move it to front
        recent.add_file(PathBuf::from("/path/to/file1.json"));
        assert_eq!(recent.files[0], PathBuf::from("/path/to/file1.json"));
    }

    #[test]
    fn test_max_files() {
        let mut recent = RecentFiles::new();
        for i in 0..15 {
            recent.files.push(PathBuf::from(format!("/path/to/file{}.json", i)));
        }
        recent.files.truncate(MAX_RECENT_FILES);
        assert_eq!(recent.files.len(), MAX_RECENT_FILES);
    }
}
