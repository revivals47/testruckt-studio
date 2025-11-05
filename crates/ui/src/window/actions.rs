//! Window-level action handlers for menu and toolbar events
//!
//! Implements action callbacks for file operations, editing, view toggles, and tools.

use gtk4::{gio, prelude::*, Box as GtkBox};

/// Register all window-level actions
pub fn register_window_actions(
    window: &gtk4::ApplicationWindow,
    state: crate::app::AppState,
    canvas_view: &crate::canvas::CanvasView,
    tool_palette: &GtkBox,
    properties_panel: &GtkBox,
) {
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
    let undo_drawing_area = canvas_view.drawing_area();
    add_window_action(window, "undo", move |_| {
        tracing::info!("Action: undo");
        if undo_state.undo() {
            tracing::info!("✅ Undo successful");
            undo_drawing_area.queue_draw();
        } else {
            tracing::info!("⚠️  Nothing to undo");
        }
    });

    let redo_state = state.clone();
    let redo_drawing_area = canvas_view.drawing_area();
    add_window_action(window, "redo", move |_| {
        tracing::info!("Action: redo");
        if redo_state.redo() {
            tracing::info!("✅ Redo successful");
            redo_drawing_area.queue_draw();
        } else {
            tracing::info!("⚠️  Nothing to redo");
        }
    });

    let select_all_state = state.clone();
    let select_all_drawing_area = canvas_view.drawing_area();
    let select_all_render_state = canvas_view.render_state().clone();
    add_window_action(window, "select-all", move |_| {
        tracing::info!("Action: select all objects");

        let all_ids = select_all_state.get_all_object_ids();

        if all_ids.is_empty() {
            tracing::info!("⚠️  No objects to select");
        } else {
            // Clear current selection and select all
            let mut selected = select_all_render_state.selected_ids.borrow_mut();
            selected.clear();
            for id in &all_ids {
                selected.push(*id);
            }
            drop(selected);
            select_all_drawing_area.queue_draw();
            tracing::info!("✅ Selected {} objects", all_ids.len());
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
    let grid_drawing_area = canvas_view.drawing_area();
    let grid_render_state = canvas_view.render_state().clone();
    add_window_action(window, "toggle-grid", move |_| {
        tracing::info!("Action: toggle grid visibility");
        let mut config = grid_render_state.config.borrow_mut();
        config.show_grid = !config.show_grid;
        let new_state = config.show_grid;
        drop(config);
        tracing::info!("✅ Grid visibility toggled: {}", new_state);
        grid_drawing_area.queue_draw();
    });

    let guides_drawing_area = canvas_view.drawing_area();
    let guides_render_state = canvas_view.render_state().clone();
    add_window_action(window, "toggle-guides", move |_| {
        tracing::info!("Action: toggle guides visibility");
        let mut config = guides_render_state.config.borrow_mut();
        config.show_guides = !config.show_guides;
        let new_state = config.show_guides;
        drop(config);
        tracing::info!("✅ Guides visibility toggled: {}", new_state);
        guides_drawing_area.queue_draw();
    });

    let rulers_drawing_area = canvas_view.drawing_area();
    let rulers_render_state = canvas_view.render_state().clone();
    add_window_action(window, "toggle-rulers", move |_| {
        tracing::info!("Action: toggle rulers");
        let mut config = rulers_render_state.config.borrow_mut();
        config.show_rulers = !config.show_rulers;
        let new_state = config.show_rulers;
        drop(config);
        tracing::info!("✅ Rulers visibility toggled: {}", new_state);
        rulers_drawing_area.queue_draw();
    });

    let tool_palette_toggle = tool_palette.clone();
    add_window_action(window, "toggle-layers", move |_| {
        tracing::info!("Action: toggle layers panel");
        let is_visible = tool_palette_toggle.is_visible();
        tool_palette_toggle.set_visible(!is_visible);
        tracing::info!("✅ Tool palette visibility toggled: {}", !is_visible);
    });

    let properties_panel_toggle = properties_panel.clone();
    add_window_action(window, "toggle-properties", move |_| {
        tracing::info!("Action: toggle properties panel");
        let is_visible = properties_panel_toggle.is_visible();
        properties_panel_toggle.set_visible(!is_visible);
        tracing::info!("✅ Properties panel visibility toggled: {}", !is_visible);
    });

    add_window_action(window, "open-json-editor", |_| {
        tracing::info!("Action: open JSON editor");
        // TODO: Open JSON editor panel
    });

    // Tools menu actions
    let insert_image_state = state.clone();
    let insert_image_window = window.clone();
    let insert_image_drawing_area = canvas_view.drawing_area();
    add_window_action(window, "insert-image", move |_| {
        tracing::info!("Action: insert image");

        let window_ref = insert_image_window.clone();
        let state_ref = insert_image_state.clone();
        let drawing_area = insert_image_drawing_area.clone();

        // Show image file chooser dialog
        let window_as_base = window_ref.upcast::<gtk4::Window>();
        crate::dialogs::show_image_chooser_async(
            &window_as_base,
            Box::new(move |path| {
                tracing::info!("Selected image file: {}", path.display());

                // Register image path with asset catalog
                let asset_catalog = state_ref.asset_catalog();
                let asset_ref = {
                    let mut catalog = asset_catalog.lock().expect("asset catalog");
                    catalog.register(&path)
                };
                tracing::info!("✅ Registered image asset: {:?}", asset_ref);

                // Create ImageElement with registered asset
                let image_element = testruct_core::document::ImageElement {
                    id: uuid::Uuid::new_v4(),
                    source: asset_ref,
                    bounds: testruct_core::layout::Rect {
                        origin: testruct_core::layout::Point { x: 100.0, y: 100.0 },
                        size: testruct_core::layout::Size { width: 200.0, height: 200.0 },
                    },
                };

                // Add image to document
                match state_ref.add_element_to_active_page(
                    testruct_core::document::DocumentElement::Image(image_element),
                ) {
                    Ok(_) => {
                        tracing::info!("✅ Image inserted: {}", path.display());
                        // Trigger canvas redraw
                        drawing_area.queue_draw();
                    }
                    Err(e) => {
                        tracing::error!("❌ Failed to insert image: {}", e);
                    }
                }
            }),
        );
    });

    add_window_action(window, "templates", |_| {
        tracing::info!("Action: show templates");
        // TODO: Show template manager
    });

    let item_library_panel = properties_panel.clone();
    add_window_action(window, "toggle-item-library", move |_| {
        tracing::info!("Action: toggle item library");
        let is_visible = item_library_panel.is_visible();
        item_library_panel.set_visible(!is_visible);
        tracing::info!("✅ Item library visibility toggled: {}", !is_visible);
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
        ("win.insert-image", "<Primary>i"),
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
