use std::sync::{Arc, Mutex};

use testruct_core::{Document, DocumentId, Project};

#[derive(Clone, Default)]
pub struct AppState {
    inner: Arc<Mutex<AppShared>>,
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
}

#[derive(Default)]
struct AppShared {
    project: Project,
    active_document: Option<DocumentId>,
}
