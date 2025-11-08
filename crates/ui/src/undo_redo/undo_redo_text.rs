//! Text and element manipulation undo/redo commands
//!
//! Provides commands for pasting and duplicating text elements
//! and other document elements.

use super::Command;
use super::undo_redo_shape::elem_id;
use std::sync::{Arc, Mutex};
use testruct_core::document::{Document, DocumentElement};
use uuid::Uuid;

/// Paste command for pasting elements from clipboard
#[derive(Debug)]
pub struct PasteCommand {
    document: Arc<Mutex<Document>>,
    page_index: usize,
    pasted_element_ids: Vec<Uuid>,
}

impl PasteCommand {
    /// Create a new paste command
    pub fn new(
        document: Arc<Mutex<Document>>,
        elements: Vec<DocumentElement>,
        page_index: usize,
    ) -> Self {
        let pasted_element_ids = elements.iter().map(elem_id).collect();
        {
            let mut doc = document.lock().expect("document");
            if page_index < doc.pages.len() {
                for elem in elements {
                    doc.pages[page_index].add_element(elem);
                }
            }
        }

        Self {
            document,
            page_index,
            pasted_element_ids,
        }
    }
}

impl Command for PasteCommand {
    fn execute(&mut self) -> Result<String, String> {
        Ok(format!("Pasted {} elements", self.pasted_element_ids.len()))
    }

    fn undo(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document");

        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];
        for id in &self.pasted_element_ids {
            page.elements.retain(|e| elem_id(e) != *id);
        }

        Ok(format!("Removed {} pasted elements", self.pasted_element_ids.len()))
    }

    fn description(&self) -> &str {
        "Paste"
    }
}

/// Duplicate command for duplicating selected elements
#[derive(Debug)]
pub struct DuplicateCommand {
    document: Arc<Mutex<Document>>,
    page_index: usize,
    element_ids_to_duplicate: Vec<Uuid>,
    duplicated_ids: Vec<Uuid>,
}

impl DuplicateCommand {
    /// Create a new duplicate command
    pub fn new(
        document: Arc<Mutex<Document>>,
        element_ids: Vec<Uuid>,
        page_index: usize,
    ) -> Self {
        Self {
            document,
            page_index,
            element_ids_to_duplicate: element_ids,
            duplicated_ids: Vec::new(),
        }
    }
}

impl Command for DuplicateCommand {
    fn execute(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document");

        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];

        for orig_id in &self.element_ids_to_duplicate {
            if let Some(element) = page.elements.iter().find(|e| elem_id(e) == *orig_id) {
                let mut new_elem = element.clone();
                let new_id = Uuid::new_v4();

                // Update ID and offset position
                match &mut new_elem {
                    DocumentElement::Text(t) => {
                        t.id = new_id;
                        t.bounds.origin.x += 20.0;
                        t.bounds.origin.y += 20.0;
                    }
                    DocumentElement::Image(img) => {
                        img.id = new_id;
                        img.bounds.origin.x += 20.0;
                        img.bounds.origin.y += 20.0;
                    }
                    DocumentElement::Shape(shape) => {
                        shape.id = new_id;
                        shape.bounds.origin.x += 20.0;
                        shape.bounds.origin.y += 20.0;
                    }
                    DocumentElement::Frame(frame) => {
                        frame.id = new_id;
                        frame.bounds.origin.x += 20.0;
                        frame.bounds.origin.y += 20.0;
                    }
                    DocumentElement::Group(group) => {
                        group.id = new_id;
                        group.bounds.origin.x += 20.0;
                        group.bounds.origin.y += 20.0;
                    }
                }

                page.add_element(new_elem);
                self.duplicated_ids.push(new_id);
            }
        }

        Ok(format!("Duplicated {} objects", self.element_ids_to_duplicate.len()))
    }

    fn undo(&mut self) -> Result<String, String> {
        let mut doc = self.document.lock().expect("document");

        if self.page_index >= doc.pages.len() {
            return Err("Page index out of bounds".to_string());
        }

        let page = &mut doc.pages[self.page_index];
        for dup_id in &self.duplicated_ids {
            page.elements.retain(|e| elem_id(e) != *dup_id);
        }

        Ok("Removed duplicated objects".to_string())
    }

    fn description(&self) -> &str {
        "Duplicate"
    }
}
