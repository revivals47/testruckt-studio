//! File and page management action handlers

use super::common::add_window_action;
use crate::canvas::CanvasView;
use gtk4::prelude::*;

/// Register file menu actions
pub fn register(
    window: &gtk4::ApplicationWindow,
    state: crate::app::AppState,
    canvas_view: &CanvasView,
) {
    // File menu actions
    let new_state = state.clone();
    let new_drawing_area = canvas_view.drawing_area();
    let new_render_state = canvas_view.render_state().clone();
    let window_weak_new = window.downgrade();
    add_window_action(window, "new", move |_| {
        tracing::info!("Action: new document");
        if let Some(window) = window_weak_new.upgrade() {
            let state_clone = new_state.clone();
            let drawing_area_clone = new_drawing_area.clone();
            let render_state_clone = new_render_state.clone();
            glib::spawn_future_local(async move {
                // Check for unsaved changes before creating new document
                if crate::dialogs::check_unsaved_changes(&state_clone, &window.clone().upcast()).await {
                    perform_new_document(&state_clone);
                    render_state_clone.selected_ids.borrow_mut().clear();
                    drawing_area_clone.queue_draw();
                }
            });
        }
    });

    let open_state = state.clone();
    let window_weak_open = window.downgrade();
    let open_drawing_area = canvas_view.drawing_area();
    let open_render_state = canvas_view.render_state().clone();
    add_window_action(window, "open", move |_| {
        tracing::info!("Action: open document");
        if let Some(window) = window_weak_open.upgrade() {
            let state_clone = open_state.clone();
            let drawing_area_clone = open_drawing_area.clone();
            let render_state_clone = open_render_state.clone();
            glib::spawn_future_local(async move {
                // Check for unsaved changes before opening another document
                if crate::dialogs::check_unsaved_changes(&state_clone, &window.clone().upcast()).await {
                    perform_open_document_async(&window, &state_clone, drawing_area_clone, render_state_clone).await;
                }
            });
        }
    });

    let save_state = state.clone();
    let window_weak_save_overwrite = window.downgrade();
    add_window_action(window, "save", move |_| {
        tracing::info!("Action: save document");
        if let Some(window) = window_weak_save_overwrite.upgrade() {
            perform_save_document(&window, &save_state);
        }
    });

    let save_as_state = state.clone();
    let window_weak_save = window.downgrade();
    add_window_action(window, "save-as", move |_| {
        tracing::info!("Action: save document as");
        if let Some(window) = window_weak_save.upgrade() {
            perform_save_as_document(&window, &save_as_state);
        }
    });

    // Recent files action
    let recent_state = state.clone();
    let window_weak_recent = window.downgrade();
    let recent_drawing_area = canvas_view.drawing_area();
    let recent_render_state = canvas_view.render_state().clone();
    add_window_action(window, "recent-files", move |_| {
        tracing::info!("Action: show recent files");
        if let Some(window) = window_weak_recent.upgrade() {
            crate::dialogs::show_recent_files_dialog(
                &window.clone().upcast(),
                recent_state.clone(),
                recent_drawing_area.clone(),
                recent_render_state.clone(),
            );
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
                tracing::info!(
                    "✅ Page duplicated. Total pages: {}",
                    page_state.page_count()
                );
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

    let doc_id = doc.id;
    state.set_active_document(doc);

    // Clear file path and modified state for new document
    state.clear_document_state();

    tracing::info!("✅ New document created with ID: {:?}", doc_id);
}

/// Perform open document with file dialog (async version)
async fn perform_open_document_async(
    window: &gtk4::ApplicationWindow,
    state: &crate::app::AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: crate::canvas::CanvasRenderState,
) {
    tracing::info!("Opening document");

    if let Some(path) = crate::io::file_dialog::show_open_dialog(window).await {
        match crate::io::file_io::load_document(&path) {
            Ok(document) => {
                state.set_active_document(document);
                // Add to recent files and mark as saved with the file path
                state.add_recent_file(path.clone());
                state.mark_as_saved(path.clone());
                render_state.selected_ids.borrow_mut().clear();
                drawing_area.queue_draw();
                tracing::info!("✅ Document loaded and activated: {}", path.display());
            }
            Err(e) => {
                tracing::error!("❌ Failed to load document: {}", e);
            }
        }
    } else {
        tracing::info!("File open cancelled by user");
    }
}

/// Perform save document (overwrite if path exists, otherwise show save dialog)
fn perform_save_document(window: &gtk4::ApplicationWindow, state: &crate::app::AppState) {
    tracing::info!("Saving document");

    // Check if we have an existing file path
    if let Some(path) = state.current_file_path() {
        // Overwrite existing file
        if let Some(document) = state.active_document() {
            match crate::io::file_io::save_document(&document, &path) {
                Ok(_) => {
                    state.mark_as_saved(path.clone());
                    tracing::info!("✅ Document saved (overwrite): {}", path.display());
                }
                Err(e) => {
                    tracing::error!("❌ Failed to save document: {}", e);
                }
            }
        } else {
            tracing::warn!("No active document to save");
        }
    } else {
        // No existing path, show "Save As" dialog
        tracing::info!("No existing file path, showing Save As dialog");
        perform_save_as_document(window, state);
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
                match crate::io::file_io::save_document(
                    &state_clone.active_document().unwrap(),
                    &path,
                ) {
                    Ok(_) => {
                        // Add to recent files and mark as saved with the new path
                        state_clone.add_recent_file(path.clone());
                        state_clone.mark_as_saved(path.clone());
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
