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

    pub fn remove_document(&mut self, id: DocumentId) -> Option<Document> {
        self.documents.remove(&id)
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

    // Canvas settings
    pub default_canvas_width: f32,
    pub default_canvas_height: f32,

    // Grid settings
    pub grid_size: f32,
    pub snap_to_grid: bool,

    // Guide settings
    pub snap_to_guides: bool,
    pub snap_distance: f32,

    // Autosave settings
    pub autosave_enabled: bool,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            autosave_minutes: 5,
            workspace_root: std::env::current_dir()
                .unwrap_or_else(|_| std::path::PathBuf::from(".")),
            default_canvas_width: 595.0,
            default_canvas_height: 842.0,
            grid_size: 10.0,
            snap_to_grid: true,
            snap_to_guides: true,
            snap_distance: 5.0,
            autosave_enabled: true,
        }
    }
}
