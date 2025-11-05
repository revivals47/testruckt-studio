use std::sync::{Arc, Mutex};

use testruct_core::{Document, DocumentId, Project};
use crate::undo_redo::UndoRedoStack;

#[derive(Clone)]
pub struct AppState {
    inner: Arc<Mutex<AppShared>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            inner: Arc::new(Mutex::new(AppShared::default())),
        }
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
}

#[derive(Default)]
struct AppShared {
    project: Project,
    active_document: Option<DocumentId>,
    undo_redo_stack: Arc<Mutex<UndoRedoStack>>,
}
