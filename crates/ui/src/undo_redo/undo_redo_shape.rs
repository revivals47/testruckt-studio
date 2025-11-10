//! Shape-related undo/redo commands
//!
//! Provides commands for creating, deleting, pasting, and duplicating shapes
//! and other document elements.

use super::Command;
use std::sync::{Arc, Mutex};
use testruct_core::document::{Document, DocumentElement};
use uuid::Uuid;

/// Helper function to get element ID from DocumentElement enum
pub(crate) fn elem_id(element: &DocumentElement) -> Uuid {
    match element {
        DocumentElement::Frame(f) => f.id,
        DocumentElement::Text(t) => t.id,
        DocumentElement::Image(i) => i.id,
        DocumentElement::Shape(s) => s.id,
        DocumentElement::Group(g) => g.id,
    }
}

/// Delete command for removing document elements
pub struct DeleteCommand {
    document: Arc<Mutex<Document>>,
    element_id: Uuid,
    deleted_element: Option<DocumentElement>,
    deleted_page_index: usize,
}

impl DeleteCommand {
    /// Create a new delete command
    pub fn new(document: Arc<Mutex<Document>>, element_id: Uuid, page_index: usize) -> Self {
        Self {
            document,
            element_id,
            deleted_element: None,
            deleted_page_index: page_index,
        }
    }
}

impl Command for DeleteCommand {
    fn execute(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document lock");

        if self.deleted_page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.deleted_page_index];

        // Find and remove the element
        if let Some(position) = page
            .elements
            .iter()
            .position(|elem| elem_id(elem) == self.element_id)
        {
            self.deleted_element = Some(page.elements.remove(position));
            Ok(format!("Deleted element {}", self.element_id))
        } else {
            Err(format!("Element {} not found", self.element_id))
        }
    }

    fn undo(&mut self) -> Result<String, String> {
        if let Some(element) = self.deleted_element.take() {
            let mut doc = self.document.lock().expect("document lock");

            if self.deleted_page_index >= doc.pages.len() {
                return Err("Page index out of bounds".to_string());
            }

            doc.pages[self.deleted_page_index].add_element(element);
            Ok(format!("Restored element {}", self.element_id))
        } else {
            Err("No deleted element to restore".to_string())
        }
    }

    fn description(&self) -> &str {
        "Delete object"
    }
}

impl std::fmt::Debug for DeleteCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteCommand")
            .field("element_id", &self.element_id)
            .field("page_index", &self.deleted_page_index)
            .finish()
    }
}

/// Create command for adding new document elements
#[derive(Debug)]
pub struct CreateCommand {
    document: Arc<Mutex<Document>>,
    element: Option<DocumentElement>,
    element_id: Option<Uuid>,
    page_index: usize,
}

impl CreateCommand {
    /// Create a new create command
    pub fn new(
        document: Arc<Mutex<Document>>,
        element: DocumentElement,
        page_index: usize,
    ) -> Self {
        let element_id = Some(elem_id(&element));
        Self {
            document,
            element: Some(element),
            element_id,
            page_index,
        }
    }
}

impl Command for CreateCommand {
    fn execute(&mut self) -> Result<String, String> {
        if let Some(element) = self.element.take() {
            let element_id = elem_id(&element);
            let mut doc = self.document.lock().expect("document lock");

            if self.page_index >= doc.pages.len() {
                return Err("Page index out of bounds".to_string());
            }

            doc.pages[self.page_index].add_element(element);
            Ok(format!("Created element {}", element_id))
        } else {
            Err("No element to create".to_string())
        }
    }

    fn undo(&mut self) -> Result<String, String> {
        // Find and remove the element we created by ID
        if let Some(element_id) = self.element_id {
            let mut doc = self.document.lock().expect("document lock");

            if self.page_index >= doc.pages.len() {
                return Err("Page index out of bounds".to_string());
            }

            let page = &mut doc.pages[self.page_index];
            if let Some(position) = page
                .elements
                .iter()
                .position(|elem| elem_id(elem) == element_id)
            {
                self.element = Some(page.elements.remove(position));
                Ok(format!("Removed element {}", element_id))
            } else {
                Err(format!("Element {} not found for undo", element_id))
            }
        } else {
            Err("No element ID stored for undo".to_string())
        }
    }

    fn description(&self) -> &str {
        "Create object"
    }
}
