//! Window-level action handlers for menu and toolbar events
//!
//! Implements action callbacks for file operations, editing, view toggles, and tools.

use gtk4::{gio, prelude::*};

/// Register all window-level actions
pub fn register_window_actions(window: &gtk4::ApplicationWindow, state: crate::app::AppState) {
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

    // Export actions
    let export_state = state.clone();
    let window_weak_pdf = window.downgrade();
    add_window_action(window, "export-pdf", move |_| {
        tracing::info!("Action: export as PDF");
        if let Some(window) = window_weak_pdf.upgrade() {
            perform_pdf_export(&window, &export_state);
        }
    });

    let export_state = state.clone();
    let window_weak_png = window.downgrade();
    add_window_action(window, "export-png", move |_| {
        tracing::info!("Action: export as PNG");
        if let Some(window) = window_weak_png.upgrade() {
            perform_image_export(&window, &export_state, "png");
        }
    });

    let export_state = state.clone();
    let window_weak_jpeg = window.downgrade();
    add_window_action(window, "export-jpeg", move |_| {
        tracing::info!("Action: export as JPEG");
        if let Some(window) = window_weak_jpeg.upgrade() {
            perform_image_export(&window, &export_state, "jpeg");
        }
    });

    let export_state = state.clone();
    let window_weak_svg = window.downgrade();
    add_window_action(window, "export-svg", move |_| {
        tracing::info!("Action: export as SVG");
        if let Some(window) = window_weak_svg.upgrade() {
            perform_image_export(&window, &export_state, "svg");
        }
    });

    // Edit menu actions
    let undo_state = state.clone();
    add_window_action(window, "undo", move |_| {
        tracing::info!("Action: undo");
        if undo_state.undo() {
            tracing::info!("✅ Undo successful");
            // TODO: Trigger canvas redraw
        } else {
            tracing::info!("⚠️  Nothing to undo");
        }
    });

    let redo_state = state.clone();
    add_window_action(window, "redo", move |_| {
        tracing::info!("Action: redo");
        if redo_state.redo() {
            tracing::info!("✅ Redo successful");
            // TODO: Trigger canvas redraw
        } else {
            tracing::info!("⚠️  Nothing to redo");
        }
    });

    add_window_action(window, "select-all", |_| {
        tracing::info!("Action: select all objects");
        // TODO: Select all objects on canvas
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
        // Delete current page (index 0 for now)
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
        // Duplicate current page (index 0 for now)
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
        // Move current page up (index 0 for now)
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
        // Move current page down (index 0 for now)
        match page_state.move_page_down(0) {
            Ok(_) => {
                tracing::info!("✅ Page moved down");
            }
            Err(e) => {
                tracing::warn!("⚠️  Failed to move page down: {}", e);
            }
        }
    });

    // View menu actions
    add_window_action(window, "toggle-grid", |_| {
        tracing::info!("Action: toggle grid visibility");
        // TODO: Toggle grid visibility on canvas
    });

    add_window_action(window, "toggle-guides", |_| {
        tracing::info!("Action: toggle guides visibility");
        // TODO: Toggle guides visibility
    });

    add_window_action(window, "toggle-rulers", |_| {
        tracing::info!("Action: toggle rulers");
        // TODO: Toggle rulers visibility
    });

    add_window_action(window, "toggle-layers", |_| {
        tracing::info!("Action: toggle layers panel");
        // TODO: Toggle layers panel visibility
    });

    add_window_action(window, "toggle-properties", |_| {
        tracing::info!("Action: toggle properties panel");
        // TODO: Toggle properties panel visibility
    });

    add_window_action(window, "open-json-editor", |_| {
        tracing::info!("Action: open JSON editor");
        // TODO: Open JSON editor panel
    });

    // Tools menu actions
    add_window_action(window, "templates", |_| {
        tracing::info!("Action: show templates");
        // TODO: Show template manager
    });

    add_window_action(window, "toggle-item-library", |_| {
        tracing::info!("Action: toggle item library");
        // TODO: Toggle item library panel
    });

    add_window_action(window, "toggle-block-tools", |_| {
        tracing::info!("Action: toggle block tools");
        // TODO: Toggle block tools panel
    });

    add_window_action(window, "settings", |_| {
        tracing::info!("Action: show settings");
        // TODO: Show settings dialog
    });

    // Help menu actions
    add_window_action(window, "user-manual", |_| {
        tracing::info!("Action: open user manual");
        // TODO: Open user manual
    });

    add_window_action(window, "about", |_| {
        tracing::info!("Action: show about dialog");
        // TODO: Show about dialog
    });

    // Set keyboard accelerators
    set_accelerators(window);
}

/// Add a window action with a callback
fn add_window_action<F>(window: &gtk4::ApplicationWindow, name: &str, callback: F)
where
    F: Fn(&gtk4::ApplicationWindow) + 'static,
{
    let action = gio::SimpleAction::new(name, None);
    let window_ref = window.clone();
    action.connect_activate(move |_, _| callback(&window_ref));
    window.add_action(&action);
}

/// Add a window action with captured state
fn add_window_action_with_capture<F, T>(window: &gtk4::ApplicationWindow, name: &str, capture: T, callback: F)
where
    F: Fn(&gtk4::ApplicationWindow, T) + 'static,
    T: Clone + 'static,
{
    let action = gio::SimpleAction::new(name, None);
    let window_ref = window.clone();
    action.connect_activate(move |_, _| callback(&window_ref, capture.clone()));
    window.add_action(&action);
}

/// Set keyboard accelerators for window-level actions
fn set_accelerators(window: &gtk4::ApplicationWindow) {
    let app = window.application().unwrap();

    let shortcuts = [
        ("win.new", "<Primary>n"),
        ("win.open", "<Primary>o"),
        ("win.save", "<Primary>s"),
        ("win.save-as", "<Primary><Shift>s"),
        ("win.undo", "<Primary>z"),
        ("win.redo", "<Primary><Shift>z"),
        ("win.select-all", "<Primary>a"),
        ("win.add-page", "<Primary><Shift>n"),
        ("win.delete-page", "<Primary><Shift>d"),
        ("win.duplicate-page", "<Primary><Shift>c"),
        ("win.move-page-up", "<Primary><Shift>Page_Up"),
        ("win.move-page-down", "<Primary><Shift>Page_Down"),
        ("win.toggle-grid", "F8"),
        ("win.toggle-guides", "F7"),
        ("win.toggle-rulers", "F6"),
    ];

    for (action, accel) in &shortcuts {
        app.set_accels_for_action(action, &[accel]);
    }
}

/// Perform PDF export
fn perform_pdf_export(window: &gtk4::ApplicationWindow, state: &crate::app::AppState) {
    // Get active document
    if state.active_document().is_some() {
        tracing::info!("Exporting active document to PDF");

        let window_clone = window.clone();
        let state_clone = state.clone();

        glib::spawn_future_local(async move {
            // Show export dialog
            if let Some(path) = crate::io::file_dialog::show_export_dialog(&window_clone, "pdf").await {
                // Perform export
                match crate::export::export_pdf(&state_clone.active_document().unwrap(), &path) {
                    Ok(_) => {
                        tracing::info!("✅ PDF export completed: {}", path.display());
                    }
                    Err(e) => {
                        tracing::error!("❌ PDF export failed: {}", e);
                    }
                }
            } else {
                tracing::info!("PDF export cancelled by user");
            }
        });
    } else {
        tracing::warn!("No active document to export");
    }
}

/// Perform image export (PNG/JPEG/SVG)
fn perform_image_export(window: &gtk4::ApplicationWindow, state: &crate::app::AppState, format: &str) {
    // Get active document
    if state.active_document().is_some() {
        tracing::info!("Exporting active document to {}", format.to_uppercase());

        let window_clone = window.clone();
        let state_clone = state.clone();
        let format_str = format.to_string();

        glib::spawn_future_local(async move {
            // Show export dialog
            if let Some(path) = crate::io::file_dialog::show_export_dialog(&window_clone, &format_str).await {
                // Perform export
                match format_str.as_str() {
                    "png" => {
                        match crate::export::export_png(&state_clone.active_document().unwrap(), &path, 96.0) {
                            Ok(_) => {
                                tracing::info!("✅ PNG export completed: {}", path.display());
                            }
                            Err(e) => {
                                tracing::error!("❌ PNG export failed: {}", e);
                            }
                        }
                    }
                    "jpeg" => {
                        match crate::export::export_jpeg(&state_clone.active_document().unwrap(), &path, 96.0, 85) {
                            Ok(_) => {
                                tracing::info!("✅ JPEG export completed: {}", path.display());
                            }
                            Err(e) => {
                                tracing::error!("❌ JPEG export failed: {}", e);
                            }
                        }
                    }
                    "svg" => {
                        match crate::export::export_svg(&state_clone.active_document().unwrap(), &path) {
                            Ok(_) => {
                                tracing::info!("✅ SVG export completed: {}", path.display());
                            }
                            Err(e) => {
                                tracing::error!("❌ SVG export failed: {}", e);
                            }
                        }
                    }
                    _ => {
                        tracing::error!("Unknown export format: {}", format_str);
                    }
                }
            } else {
                tracing::info!("{} export cancelled by user", format_str.to_uppercase());
            }
        });
    } else {
        tracing::warn!("No active document to export");
    }
}

/// Create a new document
fn perform_new_document(state: &crate::app::AppState) {
    tracing::info!("Creating new document");

    // Create a new document using builder pattern
    match testruct_core::document::DocumentBuilder::new()
        .with_title("Untitled Document")
        .add_page(testruct_core::document::Page::empty())
        .build()
    {
        Ok(document) => {
            // Add document to project
            state.with_active_document(|_doc| {
                // Document will be replaced by new one through project mutation
            });

            // In a full implementation, would update the UI
            tracing::info!("✅ New document created");
        }
        Err(e) => {
            tracing::error!("❌ Failed to create new document: {}", e);
        }
    }
}

/// Open a document from file
fn perform_open_document(window: &gtk4::ApplicationWindow, state: &crate::app::AppState) {
    tracing::info!("Opening document");

    let window_clone = window.clone();
    let state_clone = state.clone();

    glib::spawn_future_local(async move {
        // Show open dialog
        if let Some(path) = crate::io::file_dialog::show_open_dialog(&window_clone).await {
            // Load document
            match crate::io::file_io::load_document(&path) {
                Ok(_document) => {
                    tracing::info!("✅ Document loaded: {}", path.display());
                    // In a full implementation, would update the active document in state
                }
                Err(e) => {
                    tracing::error!("❌ Failed to load document: {}", e);
                }
            }
        } else {
            tracing::info!("Open document cancelled by user");
        }
    });
}

/// Save the active document
fn perform_save_document(state: &crate::app::AppState) {
    tracing::info!("Saving document");

    if let Some(document) = state.active_document() {
        // Use document's title with default extension
        let filename = format!("{}.json", document.metadata.title);

        // Save to documents directory
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

/// Save document with dialog to choose filename
fn perform_save_as_document(window: &gtk4::ApplicationWindow, state: &crate::app::AppState) {
    tracing::info!("Saving document as");

    if let Some(document) = state.active_document() {
        let window_clone = window.clone();
        let state_clone = state.clone();

        glib::spawn_future_local(async move {
            // Show save dialog
            if let Some(path) = crate::io::file_dialog::show_save_dialog(&window_clone).await {
                // Save document
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
