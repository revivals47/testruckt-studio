use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommandHistory {
    entries: VecDeque<HistoryEntry>,
    limit: usize,
}

impl CommandHistory {
    pub fn with_limit(limit: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(limit),
            limit,
        }
    }

    pub fn push(&mut self, entry: HistoryEntry) {
        if self.entries.len() == self.limit {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn iter(&self) -> impl Iterator<Item = &HistoryEntry> {
        self.entries.iter()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub description: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HistoryEntry {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            timestamp: chrono::Utc::now(),
        }
    }
}
