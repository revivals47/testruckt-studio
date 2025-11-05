//! File and page management action handlers

use super::common::add_window_action;
use gtk4::prelude::*;

/// Register file menu actions
pub fn register(window: &gtk4::ApplicationWindow, state: crate::app::AppState) {
    // File menu actions
    let new_state = state.clone();
    add_window_action(window, "new", move |_| {
        tracing::info!("Action: new document");
        perform_new_document(&new_state);
    });

    let open_state = state.clone();
    let window_weak_open = window.downgrade();
    add_window_action(window, "open", move |_| {
        tracing::info!("Action: open document");
        if let Some(window) = window_weak_open.upgrade() {
            perform_open_document(&window, &open_state);
        }
    });

    let save_state = state.clone();
    add_window_action(window, "save", move |_| {
        tracing::info!("Action: save document");
        perform_save_document(&save_state);
    });

    let save_as_state = state.clone();
    let window_weak_save = window.downgrade();
    add_window_action(window, "save-as", move |_| {
        tracing::info!("Action: save document as");
        if let Some(window) = window_weak_save.upgrade() {
            perform_save_as_document(&window, &save_as_state);
        }
    });

    // Page management actions
    let page_state = state.clone();
    add_window_action(window, "add-page", move |_| {
        tracing::info!("Action: add page");
        match page_state.add_page() {
            Ok(_) => {
                tracing::info!("✅ Page added. Total pages: {}", page_state.page_count());
            }
            Err(e) => {
                tracing::warn!("⚠️  Failed to add page: {}", e);
            }
        }
    });

    let page_state = state.clone();
    add_window_action(window, "delete-page", move |_| {
        tracing::info!("Action: delete page");
        match page_state.delete_page(0) {
            Ok(_) => {
                tracing::info!("✅ Page deleted. Total pages: {}", page_state.page_count());
            }
            Err(e) => {
                tracing::warn!("⚠️  Failed to delete page: {}", e);
            }
        }
    });

    let page_state = state.clone();
    add_window_action(window, "duplicate-page", move |_| {
        tracing::info!("Action: duplicate page");
        match page_state.duplicate_page(0) {
            Ok(_) => {
                tracing::info!("✅ Page duplicated. Total pages: {}", page_state.page_count());
            }
            Err(e) => {
                tracing::warn!("⚠️  Failed to duplicate page: {}", e);
            }
        }
    });

    let page_state = state.clone();
    add_window_action(window, "move-page-up", move |_| {
        tracing::info!("Action: move page up");
        match page_state.move_page_up(0) {
            Ok(_) => {
                tracing::info!("✅ Page moved up");
            }
            Err(e) => {
                tracing::warn!("⚠️  Failed to move page up: {}", e);
            }
        }
    });

    let page_state = state.clone();
    add_window_action(window, "move-page-down", move |_| {
        tracing::info!("Action: move page down");
        match page_state.move_page_down(0) {
            Ok(_) => {
                tracing::info!("✅ Page moved down");
            }
            Err(e) => {
                tracing::warn!("⚠️  Failed to move page down: {}", e);
            }
        }
    });
}

/// Perform new document creation
fn perform_new_document(state: &crate::app::AppState) {
    tracing::info!("Creating new document");

    let doc = testruct_core::document::DocumentBuilder::new()
        .with_title("Untitled")
        .add_page(testruct_core::document::Page::empty())
        .build()
        .expect("Failed to create document");

    // Get project and add document
    let mut project = state.project();
    let doc_id = doc.id;
    project.add_document(doc.clone());

    tracing::info!("✅ New document created with ID: {:?}", doc_id);
}

/// Perform open document with file dialog
fn perform_open_document(window: &gtk4::ApplicationWindow, state: &crate::app::AppState) {
    tracing::info!("Opening document");

    let window_clone = window.clone();
    let state_clone = state.clone();

    glib::spawn_future_local(async move {
        if let Some(path) = crate::io::file_dialog::show_open_dialog(&window_clone).await {
            match crate::io::file_io::load_document(&path) {
                Ok(document) => {
                    let mut project = state_clone.project();
                    project.add_document(document);
                    tracing::info!("✅ Document loaded: {}", path.display());
                }
                Err(e) => {
                    tracing::error!("❌ Failed to load document: {}", e);
                }
            }
        } else {
            tracing::info!("File open cancelled by user");
        }
    });
}

/// Perform save document
fn perform_save_document(state: &crate::app::AppState) {
    tracing::info!("Saving document");

    if let Some(document) = state.active_document() {
        let filename = format!("{}.json", document.metadata.title);

        if let Some(mut path) = crate::io::file_io::default_documents_dir() {
            path.push(&filename);

            match crate::io::file_io::save_document(&document, &path) {
                Ok(_) => {
                    tracing::info!("✅ Document saved: {}", path.display());
                }
                Err(e) => {
                    tracing::error!("❌ Failed to save document: {}", e);
                }
            }
        } else {
            tracing::error!("❌ Could not determine documents directory");
        }
    } else {
        tracing::warn!("No active document to save");
    }
}

/// Perform save document with dialog
fn perform_save_as_document(window: &gtk4::ApplicationWindow, state: &crate::app::AppState) {
    tracing::info!("Saving document as");

    if let Some(_document) = state.active_document() {
        let window_clone = window.clone();
        let state_clone = state.clone();

        glib::spawn_future_local(async move {
            if let Some(path) = crate::io::file_dialog::show_save_dialog(&window_clone).await {
                match crate::io::file_io::save_document(&state_clone.active_document().unwrap(), &path) {
                    Ok(_) => {
                        tracing::info!("✅ Document saved as: {}", path.display());
                    }
                    Err(e) => {
                        tracing::error!("❌ Failed to save document: {}", e);
                    }
                }
            } else {
                tracing::info!("Save as cancelled by user");
            }
        });
    } else {
        tracing::warn!("No active document to save");
    }
}
