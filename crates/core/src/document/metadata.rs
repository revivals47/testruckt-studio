use serde::{Deserialize, Serialize};
use super::PageSize;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub title: String,
    pub author: String,
    pub tags: Vec<String>,
    #[serde(default)]
    pub page_size: PageSize,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Default for DocumentMetadata {
    fn default() -> Self {
        Self::new("", "")
    }
}

impl DocumentMetadata {
    pub fn new(title: impl Into<String>, author: impl Into<String>) -> Self {
        let now = chrono::Utc::now();
        Self {
            title: title.into(),
            author: author.into(),
            tags: Vec::new(),
            page_size: PageSize::A4,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn touch(&mut self) {
        self.updated_at = chrono::Utc::now();
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PageMetadata {
    pub name: String,
    pub notes: Option<String>,
}
