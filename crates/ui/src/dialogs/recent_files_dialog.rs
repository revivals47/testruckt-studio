//! Recent files dialog
//!
//! Shows a list of recently opened files and allows opening them.

use crate::app::AppState;
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow, Window,
};

/// Show the recent files dialog
pub fn show_recent_files_dialog(
    parent: &Window,
    app_state: AppState,
    drawing_area: gtk4::DrawingArea,
    render_state: CanvasRenderState,
) {
    let dialog = gtk4::ApplicationWindow::builder()
        .transient_for(parent)
        .modal(true)
        .title("最近のファイル")
        .default_width(400)
        .default_height(300)
        .build();

    let main_box = GtkBox::new(Orientation::Vertical, 8);
    main_box.set_margin_start(12);
    main_box.set_margin_end(12);
    main_box.set_margin_top(12);
    main_box.set_margin_bottom(12);

    // Title
    let title = Label::new(Some("最近開いたファイル"));
    title.add_css_class("title-3");
    main_box.append(&title);

    // List of recent files
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::Single);

    let recent_files = app_state.recent_files();

    if recent_files.is_empty() {
        let empty_label = Label::new(Some("最近のファイルはありません"));
        empty_label.add_css_class("dim-label");
        empty_label.set_margin_top(20);
        empty_label.set_margin_bottom(20);
        list_box.append(&empty_label);
    } else {
        for path in &recent_files {
            let row = ListBoxRow::new();
            let row_box = GtkBox::new(Orientation::Vertical, 2);
            row_box.set_margin_start(8);
            row_box.set_margin_end(8);
            row_box.set_margin_top(4);
            row_box.set_margin_bottom(4);

            // File name
            let file_name = path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            let name_label = Label::new(Some(&file_name));
            name_label.set_halign(gtk4::Align::Start);
            name_label.add_css_class("heading");
            row_box.append(&name_label);

            // Full path
            let path_str = path.display().to_string();
            let path_label = Label::new(Some(&path_str));
            path_label.set_halign(gtk4::Align::Start);
            path_label.add_css_class("dim-label");
            path_label.set_ellipsize(gtk4::pango::EllipsizeMode::Middle);
            row_box.append(&path_label);

            row.set_child(Some(&row_box));
            // Store the path in the row's widget name for retrieval
            row.set_widget_name(&path_str);
            list_box.append(&row);
        }
    }

    scrolled.set_child(Some(&list_box));
    main_box.append(&scrolled);

    // Button box
    let button_box = GtkBox::new(Orientation::Horizontal, 8);
    button_box.set_halign(gtk4::Align::End);

    let open_btn = Button::with_label("開く");
    let clear_btn = Button::with_label("履歴をクリア");
    let close_btn = Button::with_label("閉じる");

    // Open button action
    let dialog_clone = dialog.clone();
    let list_box_clone = list_box.clone();
    let app_state_clone = app_state.clone();
    let drawing_area_clone = drawing_area.clone();
    let render_state_clone = render_state.clone();
    open_btn.connect_clicked(move |_| {
        if let Some(row) = list_box_clone.selected_row() {
            let path_str = row.widget_name().to_string();
            let path = std::path::PathBuf::from(&path_str);

            if path.exists() {
                match crate::io::file_io::load_document(&path) {
                    Ok(document) => {
                        app_state_clone.set_active_document(document);
                        app_state_clone.add_recent_file(path.clone());
                        render_state_clone.selected_ids.borrow_mut().clear();
                        drawing_area_clone.queue_draw();
                        tracing::info!("✅ Document loaded from recent: {}", path.display());
                        dialog_clone.close();
                    }
                    Err(e) => {
                        tracing::error!("❌ Failed to load document: {}", e);
                    }
                }
            } else {
                tracing::warn!("⚠️  File no longer exists: {}", path.display());
            }
        }
    });

    // Clear button action
    let app_state_clear = app_state.clone();
    let list_box_clear = list_box.clone();
    clear_btn.connect_clicked(move |_| {
        app_state_clear.clear_recent_files();
        // Remove all children from the list box
        while let Some(child) = list_box_clear.first_child() {
            list_box_clear.remove(&child);
        }
        let empty_label = Label::new(Some("最近のファイルはありません"));
        empty_label.add_css_class("dim-label");
        empty_label.set_margin_top(20);
        empty_label.set_margin_bottom(20);
        list_box_clear.append(&empty_label);
        tracing::info!("✅ Recent files cleared");
    });

    // Close button action
    let dialog_close = dialog.clone();
    close_btn.connect_clicked(move |_| {
        dialog_close.close();
    });

    // Double-click to open
    let dialog_dbl = dialog.clone();
    let app_state_dbl = app_state.clone();
    let drawing_area_dbl = drawing_area.clone();
    let render_state_dbl = render_state.clone();
    list_box.connect_row_activated(move |_, row| {
        let path_str = row.widget_name().to_string();
        let path = std::path::PathBuf::from(&path_str);

        if path.exists() {
            match crate::io::file_io::load_document(&path) {
                Ok(document) => {
                    app_state_dbl.set_active_document(document);
                    app_state_dbl.add_recent_file(path.clone());
                    render_state_dbl.selected_ids.borrow_mut().clear();
                    drawing_area_dbl.queue_draw();
                    tracing::info!("✅ Document loaded from recent: {}", path.display());
                    dialog_dbl.close();
                }
                Err(e) => {
                    tracing::error!("❌ Failed to load document: {}", e);
                }
            }
        } else {
            tracing::warn!("⚠️  File no longer exists: {}", path.display());
        }
    });

    button_box.append(&clear_btn);
    button_box.append(&open_btn);
    button_box.append(&close_btn);
    main_box.append(&button_box);

    dialog.set_child(Some(&main_box));
    dialog.present();
}
