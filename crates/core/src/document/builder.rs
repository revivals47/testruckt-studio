use super::{Document, DocumentId, Page};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DocumentBuilder {
    pub title: Option<String>,
    pub author: Option<String>,
    pub pages: Vec<Page>,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    pub fn add_page(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }

    pub fn build(self) -> Result<Document, DocumentBuilderError> {
        let metadata = super::metadata::DocumentMetadata::new(
            self.title.unwrap_or_else(|| "Untitled".into()),
            self.author.unwrap_or_else(|| "".into()),
        );

        if self.pages.is_empty() {
            return Err(DocumentBuilderError::EmptyDocument);
        }

        Ok(Document {
            id: DocumentId::new(),
            metadata,
            pages: self.pages,
            styles: Default::default(),
        })
    }
}

#[derive(Debug, Error)]
pub enum DocumentBuilderError {
    #[error("document must contain at least one page")]
    EmptyDocument,
}
