mod builder;
mod metadata;
mod page;
mod page_size;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub use builder::{DocumentBuilder, DocumentBuilderError};
pub use metadata::{DocumentMetadata, PageMetadata};
pub use page::{
    DocumentElement, FrameElement, GroupElement, ImageElement, Page, PageId, ShapeElement,
    ShapeKind, TextElement,
};
pub use page_size::PageSize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentId(uuid::Uuid);

impl DocumentId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub metadata: DocumentMetadata,
    pub pages: Vec<Page>,
    #[serde(default)]
    pub styles: HashMap<String, String>,
    #[serde(default)]
    pub assets: crate::workspace::AssetCatalog,
}

impl Document {
    pub fn empty(title: impl Into<String>) -> Self {
        DocumentBuilder::new()
            .with_title(title)
            .add_page(Page::empty())
            .build()
            .expect("empty document always valid")
    }

    pub fn page(&self, id: PageId) -> Option<&Page> {
        self.pages.iter().find(|p| p.id == id)
    }

    pub fn pages(&self) -> impl Iterator<Item = &Page> {
        self.pages.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_empty_document() {
        let doc = Document::empty("Test");
        assert_eq!(doc.metadata.title, "Test");
        assert_eq!(doc.pages.len(), 1);
    }
}
