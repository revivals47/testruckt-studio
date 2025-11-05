use std::sync::{Arc, Mutex};

use testruct_core::{Document, DocumentId, Project};
use testruct_db::ItemBank;
use crate::undo_redo::UndoRedoStack;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Mutex<AppShared>>,
}

impl Default for AppState {
    fn default() -> Self {
        // Initialize in-memory database for item bank
        let item_bank = ItemBank::memory().expect("Failed to initialize item bank");

        let app_state = Self {
            inner: Arc::new(Mutex::new(AppShared {
                project: Project::default(),
                active_document: None,
                undo_redo_stack: Arc::new(Mutex::new(crate::undo_redo::UndoRedoStack::default())),
                item_bank: Arc::new(Mutex::new(item_bank)),
            })),
        };

        // Initialize with a default document
        {
            let mut inner = app_state.inner.lock().expect("state");
            let doc = testruct_core::document::DocumentBuilder::new()
                .with_title("Untitled")
                .add_page(testruct_core::document::Page::empty())
                .build()
                .expect("document");

            let doc_id = doc.id;
            inner.project.add_document(doc);
            inner.active_document = Some(doc_id);
        }

        app_state
    }
}

impl AppState {
    pub fn project(&self) -> Project {
        self.inner.lock().expect("state").project.clone()
    }

    pub fn active_document(&self) -> Option<Document> {
        let inner = self.inner.lock().expect("state");
        inner
            .active_document
            .and_then(|id| inner.project.document(id).cloned())
    }

    pub fn with_active_document<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Document) -> R,
    {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document_mut(doc_id) {
                return Some(f(doc));
            }
        }
        None
    }

    pub fn undo_redo_stack(&self) -> std::sync::Arc<std::sync::Mutex<UndoRedoStack>> {
        let inner = self.inner.lock().expect("state");
        inner.undo_redo_stack.clone()
    }

    pub fn with_undo_stack<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut UndoRedoStack) -> R,
    {
        let inner = self.inner.lock().expect("state");
        let mut stack = inner.undo_redo_stack.lock().expect("undo stack");
        f(&mut stack)
    }

    pub fn can_undo(&self) -> bool {
        self.with_undo_stack(|stack| stack.can_undo())
    }

    pub fn can_redo(&self) -> bool {
        self.with_undo_stack(|stack| stack.can_redo())
    }

    pub fn undo(&self) -> bool {
        self.with_undo_stack(|stack| stack.undo())
    }

    pub fn redo(&self) -> bool {
        self.with_undo_stack(|stack| stack.redo())
    }

    pub fn push_command(&self, command: Box<dyn crate::undo_redo::Command>) {
        let stack = {
            let inner = self.inner.lock().expect("state");
            inner.undo_redo_stack.clone()
        };
        let mut undo_stack = stack.lock().expect("undo stack");
        undo_stack.push(command);
    }

    pub fn add_element_to_active_page(&self, element: testruct_core::document::DocumentElement) -> Result<(), String> {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document_mut(doc_id) {
                if !doc.pages.is_empty() {
                    doc.pages[0].add_element(element);
                    return Ok(());
                }
            }
        }
        Err("No active document or pages".to_string())
    }

    pub fn item_bank(&self) -> Arc<Mutex<ItemBank>> {
        let inner = self.inner.lock().expect("state");
        inner.item_bank.clone()
    }

    /// Add a new page to the active document
    pub fn add_page(&self) -> Result<(), String> {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document_mut(doc_id) {
                doc.pages.push(testruct_core::document::Page::empty());
                return Ok(());
            }
        }
        Err("No active document".to_string())
    }

    /// Delete a page from the active document by index
    pub fn delete_page(&self, index: usize) -> Result<(), String> {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document_mut(doc_id) {
                if index < doc.pages.len() && doc.pages.len() > 1 {
                    doc.pages.remove(index);
                    return Ok(());
                }
                return Err("Cannot delete the only page".to_string());
            }
        }
        Err("No active document".to_string())
    }

    /// Duplicate a page in the active document
    pub fn duplicate_page(&self, index: usize) -> Result<(), String> {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document_mut(doc_id) {
                if index < doc.pages.len() {
                    let page_clone = doc.pages[index].clone();
                    doc.pages.insert(index + 1, page_clone);
                    return Ok(());
                }
                return Err("Page index out of bounds".to_string());
            }
        }
        Err("No active document".to_string())
    }

    /// Move a page up in the active document
    pub fn move_page_up(&self, index: usize) -> Result<(), String> {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document_mut(doc_id) {
                if index > 0 && index < doc.pages.len() {
                    doc.pages.swap(index, index - 1);
                    return Ok(());
                }
                return Err("Cannot move page up from first position".to_string());
            }
        }
        Err("No active document".to_string())
    }

    /// Move a page down in the active document
    pub fn move_page_down(&self, index: usize) -> Result<(), String> {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document_mut(doc_id) {
                if index < doc.pages.len() - 1 {
                    doc.pages.swap(index, index + 1);
                    return Ok(());
                }
                return Err("Cannot move page down from last position".to_string());
            }
        }
        Err("No active document".to_string())
    }

    /// Get the total number of pages in the active document
    pub fn page_count(&self) -> usize {
        if let Some(doc) = self.active_document() {
            doc.pages.len()
        } else {
            0
        }
    }

}

struct AppShared {
    project: Project,
    active_document: Option<DocumentId>,
    undo_redo_stack: Arc<Mutex<UndoRedoStack>>,
    item_bank: Arc<Mutex<ItemBank>>,
}
