use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::app::recent_files::RecentFiles;
use crate::undo_redo::UndoRedoStack;
use gtk4::glib::WeakRef;
use gtk4::prelude::GtkWindowExt;
use gtk4::ApplicationWindow;
use testruct_core::workspace::assets::AssetCatalog;
use testruct_core::{Document, DocumentId, Project};
use testruct_db::ItemBank;

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
                active_page_index: 0,
                undo_redo_stack: Arc::new(Mutex::new(crate::undo_redo::UndoRedoStack::default())),
                item_bank: Arc::new(Mutex::new(item_bank)),
                asset_catalog: Arc::new(Mutex::new(AssetCatalog::new())),
                window: None,
                recent_files: RecentFiles::load(),
                current_file_path: None,
                is_modified: false,
                auto_save_enabled: true,
                last_modified_time: None,
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

    /// Execute a function on the project (mutable access)
    pub fn with_project<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Project) -> R,
    {
        let mut inner = self.inner.lock().expect("state");
        f(&mut inner.project)
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

    pub fn add_element_to_active_page(
        &self,
        element: testruct_core::document::DocumentElement,
    ) -> Result<(), String> {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            let page_index = inner.active_page_index;
            if let Some(doc) = inner.project.document_mut(doc_id) {
                if page_index < doc.pages.len() {
                    doc.pages[page_index].add_element(element);
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

    pub fn asset_catalog(&self) -> Arc<Mutex<AssetCatalog>> {
        let inner = self.inner.lock().expect("state");
        inner.asset_catalog.clone()
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

    /// Get the active page index
    pub fn active_page_index(&self) -> usize {
        let inner = self.inner.lock().expect("state");
        inner.active_page_index
    }

    /// Set the active page index (bounds checked)
    pub fn set_active_page_index(&self, index: usize) -> Result<(), String> {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document(doc_id) {
                if index < doc.pages.len() {
                    inner.active_page_index = index;
                    tracing::info!("📄 Active page set to {} (0-indexed)", index);
                    return Ok(());
                }
                return Err(format!(
                    "Page index {} out of bounds (0..{})",
                    index,
                    doc.pages.len()
                ));
            }
        }
        Err("No active document".to_string())
    }

    /// Get a reference to the active page (read-only)
    pub fn active_page(&self) -> Option<testruct_core::document::Page> {
        let inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document(doc_id) {
                if inner.active_page_index < doc.pages.len() {
                    return Some(doc.pages[inner.active_page_index].clone());
                }
            }
        }
        None
    }

    /// Execute a function on the active page (mutable access)
    pub fn with_active_page<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut testruct_core::document::Page) -> R,
    {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            let page_index = inner.active_page_index;
            if let Some(doc) = inner.project.document_mut(doc_id) {
                if page_index < doc.pages.len() {
                    return Some(f(&mut doc.pages[page_index]));
                }
            }
        }
        None
    }

    /// Get all object IDs in the active page of the active document
    pub fn get_all_object_ids(&self) -> Vec<uuid::Uuid> {
        if let Some(page) = self.active_page() {
            page.elements
                .iter()
                .map(|element| match element {
                    testruct_core::document::DocumentElement::Shape(shape) => shape.id,
                    testruct_core::document::DocumentElement::Text(text) => text.id,
                    testruct_core::document::DocumentElement::Image(image) => image.id,
                    testruct_core::document::DocumentElement::Frame(frame) => frame.id,
                    testruct_core::document::DocumentElement::Group(group) => group.id,
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Set the main window reference
    pub fn set_window(&self, window: &ApplicationWindow) {
        let mut inner = self.inner.lock().expect("state");
        // Use glib's downgrade to create a WeakRef
        use gtk4::glib::object::ObjectExt;
        let weak_window = window.downgrade();
        inner.window = Some(weak_window);
    }

    /// Get the main window reference if available
    pub fn window(&self) -> Option<ApplicationWindow> {
        let inner = self.inner.lock().expect("state");
        inner.window.as_ref().and_then(|weak| weak.upgrade())
    }

    /// Execute an operation directly on the active document without command pattern
    /// This is used for operations that need to work with the project's document reference
    pub fn with_mutable_active_document<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&mut testruct_core::Document) -> R,
    {
        let mut inner = self.inner.lock().expect("state");
        if let Some(doc_id) = inner.active_document {
            if let Some(doc) = inner.project.document_mut(doc_id) {
                return Some(f(doc));
            }
        }
        None
    }

    /// Add a document to the project and set it as active
    pub fn add_and_activate_document(&self, document: Document) {
        let mut inner = self.inner.lock().expect("state");
        let doc_id = document.id;
        inner.project.add_document(document);
        inner.active_document = Some(doc_id);
    }

    /// Replace the active document with a new one (for loading documents)
    pub fn set_active_document(&self, document: Document) {
        let mut inner = self.inner.lock().expect("state");
        let doc_id = document.id;

        // Remove old document if exists
        if let Some(old_id) = inner.active_document {
            inner.project.remove_document(old_id);
        }

        // Add new document and set as active
        inner.project.add_document(document);
        inner.active_document = Some(doc_id);
    }

    /// Add a file to the recent files list
    pub fn add_recent_file(&self, path: PathBuf) {
        let mut inner = self.inner.lock().expect("state");
        inner.recent_files.add_file(path);
    }

    /// Get a copy of the recent files list
    pub fn recent_files(&self) -> Vec<PathBuf> {
        let inner = self.inner.lock().expect("state");
        inner.recent_files.files.clone()
    }

    /// Clear the recent files list
    pub fn clear_recent_files(&self) {
        let mut inner = self.inner.lock().expect("state");
        inner.recent_files.clear();
    }

    // ========== File path and modification tracking ==========

    /// Get the current file path for the active document
    pub fn current_file_path(&self) -> Option<PathBuf> {
        let inner = self.inner.lock().expect("state");
        inner.current_file_path.clone()
    }

    /// Set the current file path for the active document
    pub fn set_current_file_path(&self, path: Option<PathBuf>) {
        let mut inner = self.inner.lock().expect("state");
        inner.current_file_path = path;
    }

    /// Check if the document has been modified since last save
    pub fn is_modified(&self) -> bool {
        let inner = self.inner.lock().expect("state");
        inner.is_modified
    }

    /// Set the modified flag for the document
    pub fn set_modified(&self, modified: bool) {
        let mut inner = self.inner.lock().expect("state");
        inner.is_modified = modified;
    }

    /// Mark the document as saved (sets modified to false and optionally updates the file path)
    pub fn mark_as_saved(&self, path: PathBuf) {
        {
            let mut inner = self.inner.lock().expect("state");
            inner.is_modified = false;
            inner.current_file_path = Some(path);
        }
        self.update_window_title();
        tracing::info!("📁 Document marked as saved");
    }

    /// Clear document state (for new document)
    pub fn clear_document_state(&self) {
        {
            let mut inner = self.inner.lock().expect("state");
            inner.current_file_path = None;
            inner.is_modified = false;
        }
        self.update_window_title();
    }

    /// Mark document as modified and update window title
    pub fn mark_as_modified(&self) {
        {
            let mut inner = self.inner.lock().expect("state");
            inner.is_modified = true;
            inner.last_modified_time = Some(Instant::now());
        }
        self.update_window_title();
    }

    // ========== Auto-save management ==========

    /// Check if auto-save is enabled
    pub fn is_auto_save_enabled(&self) -> bool {
        let inner = self.inner.lock().expect("state");
        inner.auto_save_enabled
    }

    /// Enable or disable auto-save
    pub fn set_auto_save_enabled(&self, enabled: bool) {
        let mut inner = self.inner.lock().expect("state");
        inner.auto_save_enabled = enabled;
        tracing::info!("💾 Auto-save {}", if enabled { "enabled" } else { "disabled" });
    }

    /// Get the time of last modification
    pub fn last_modified_time(&self) -> Option<Instant> {
        let inner = self.inner.lock().expect("state");
        inner.last_modified_time
    }

    /// Clear last modified time (after auto-save)
    pub fn clear_last_modified_time(&self) {
        let mut inner = self.inner.lock().expect("state");
        inner.last_modified_time = None;
    }

    /// Check if auto-save should be triggered
    /// Returns true if: auto-save enabled, file has path, is modified, and enough time has passed
    pub fn should_auto_save(&self, delay_secs: u64) -> bool {
        let inner = self.inner.lock().expect("state");

        // Must have auto-save enabled
        if !inner.auto_save_enabled {
            return false;
        }

        // Must have a file path (not a new unsaved document)
        if inner.current_file_path.is_none() {
            return false;
        }

        // Must be modified
        if !inner.is_modified {
            return false;
        }

        // Check if enough time has passed since last modification
        if let Some(last_modified) = inner.last_modified_time {
            last_modified.elapsed().as_secs() >= delay_secs
        } else {
            false
        }
    }

    /// Update the window title to reflect current state
    /// Format: "filename* - Testruct Studio" or "Untitled* - Testruct Studio"
    pub fn update_window_title(&self) {
        let inner = self.inner.lock().expect("state");

        let filename = if let Some(ref path) = inner.current_file_path {
            path.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Untitled".to_string())
        } else {
            "Untitled".to_string()
        };

        let modified_marker = if inner.is_modified { "*" } else { "" };
        let title = format!("{}{} - Testruct Studio", filename, modified_marker);

        if let Some(ref weak_window) = inner.window {
            if let Some(window) = weak_window.upgrade() {
                window.set_title(Some(&title));
                tracing::debug!("🏷️ Window title updated: {}", title);
            }
        }
    }
}

struct AppShared {
    project: Project,
    active_document: Option<DocumentId>,
    active_page_index: usize,
    undo_redo_stack: Arc<Mutex<UndoRedoStack>>,
    item_bank: Arc<Mutex<ItemBank>>,
    asset_catalog: Arc<Mutex<AssetCatalog>>,
    window: Option<WeakRef<ApplicationWindow>>,
    recent_files: RecentFiles,
    /// Current file path for the active document (None if unsaved)
    current_file_path: Option<PathBuf>,
    /// Whether the document has been modified since last save
    is_modified: bool,
    /// Whether auto-save is enabled
    auto_save_enabled: bool,
    /// Time of last modification (for auto-save timer)
    last_modified_time: Option<Instant>,
}
