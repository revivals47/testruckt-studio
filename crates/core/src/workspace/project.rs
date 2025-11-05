use crate::document::{Document, DocumentId};
use crate::template::{TemplateLibrary, TemplateRef};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub settings: ProjectSettings,
    pub documents: HashMap<DocumentId, Document>,
    pub templates: TemplateLibrary,
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            settings: ProjectSettings::default(),
            documents: HashMap::new(),
            templates: TemplateLibrary::default(),
        }
    }

    pub fn add_document(&mut self, document: Document) {
        self.documents.insert(document.id, document);
    }

    pub fn document(&self, id: DocumentId) -> Option<&Document> {
        self.documents.get(&id)
    }

    pub fn document_mut(&mut self, id: DocumentId) -> Option<&mut Document> {
        self.documents.get_mut(&id)
    }

    pub fn apply_template(&self, template: TemplateRef) -> Option<Document> {
        self.templates.get(template.id).map(|tmpl| {
            let mut builder = crate::document::DocumentBuilder::new().with_title(&tmpl.name);
            for page in &tmpl.pages {
                builder = builder.add_page(crate::document::Page {
                    id: crate::document::PageId::new(),
                    metadata: page.metadata.clone(),
                    elements: Vec::new(),
                });
            }
            builder.build().expect("template produces document")
        })
    }
}

impl Default for Project {
    fn default() -> Self {
        Self::new("Untitled Project")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub autosave_minutes: u32,
    pub workspace_root: std::path::PathBuf,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            autosave_minutes: 5,
            workspace_root: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
        }
    }
}
