//! JSON editor dialog for viewing and editing documents as JSON
//!
//! Allows users to view the current document structure in JSON format
//! and make direct edits to the JSON representation.

use crate::app::AppState;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, Dialog, Label, Orientation, ScrolledWindow, TextView, Window};

/// Show the JSON editor dialog for the active document
pub fn show_json_editor(parent: &Window, app_state: AppState) {
    // Create dialog
    let dialog = Dialog::builder()
        .transient_for(parent)
        .modal(true)
        .title("JSON エディタ")
        .default_width(800)
        .default_height(600)
        .destroy_with_parent(true)
        .build();

    // Main container
    let content_area = dialog.content_area();
    let main_box = GtkBox::new(Orientation::Vertical, 12);
    main_box.set_margin_start(12);
    main_box.set_margin_end(12);
    main_box.set_margin_top(12);
    main_box.set_margin_bottom(12);

    // Title
    let title = Label::new(Some("ドキュメント JSON"));
    title.add_css_class("title-2");
    title.set_halign(gtk4::Align::Start);
    main_box.append(&title);

    // Subtitle
    let subtitle = Label::new(Some("JSONを直接編集してドキュメント構造を変更できます"));
    subtitle.add_css_class("dim-label");
    subtitle.set_halign(gtk4::Align::Start);
    main_box.append(&subtitle);

    // Text view with scrolling
    let scrolled = ScrolledWindow::new();
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let text_view = TextView::new();
    text_view.set_monospace(true);
    text_view.set_wrap_mode(gtk4::WrapMode::Word);
    scrolled.set_child(Some(&text_view));
    main_box.append(&scrolled);

    // Load current document as JSON
    if let Some(doc) = app_state.active_document() {
        if let Ok(json_string) = serde_json::to_string_pretty(&doc) {
            let buffer = text_view.buffer();
            buffer.set_text(&json_string);
            tracing::debug!("✅ Loaded document as JSON ({} bytes)", json_string.len());
        } else {
            let buffer = text_view.buffer();
            buffer.set_text("Error: Could not serialize document to JSON");
            tracing::warn!("❌ Failed to serialize document to JSON");
        }
    } else {
        let buffer = text_view.buffer();
        buffer.set_text("No active document");
    }

    // Button box
    let button_box = GtkBox::new(Orientation::Horizontal, 6);
    button_box.set_halign(gtk4::Align::End);
    button_box.set_spacing(6);

    // Save button
    let save_btn = Button::with_label("保存");
    let app_state_save = app_state.clone();
    let dialog_save = dialog.clone();
    let text_view_save = text_view.clone();

    save_btn.connect_clicked(move |_| {
        let buffer = text_view_save.buffer();
        let start_iter = buffer.start_iter();
        let end_iter = buffer.end_iter();
        let json_text = buffer.text(&start_iter, &end_iter, false).to_string();

        // Try to parse JSON and update document
        match serde_json::from_str::<testruct_core::document::Document>(&json_text) {
            Ok(new_doc) => {
                // Update the active document
                app_state_save.with_active_document(|current_doc| {
                    *current_doc = new_doc;
                });
                tracing::info!("✅ Document updated from JSON");
                dialog_save.close();
            }
            Err(e) => {
                tracing::error!("❌ JSON parse error: {}", e);
                // Show error in a dialog or status message
                let error_dialog = gtk4::MessageDialog::new(
                    Some(&dialog_save),
                    gtk4::DialogFlags::MODAL,
                    gtk4::MessageType::Error,
                    gtk4::ButtonsType::Ok,
                    "JSON エラー",
                );
                error_dialog.set_secondary_text(Some(&format!("パース エラー: {}", e)));
                error_dialog.run_async(|dialog, _| {
                    dialog.close();
                });
            }
        }
    });
    button_box.append(&save_btn);

    // Cancel button
    let cancel_btn = Button::with_label("キャンセル");
    let dialog_cancel = dialog.clone();
    cancel_btn.connect_clicked(move |_| {
        dialog_cancel.close();
    });
    button_box.append(&cancel_btn);

    // Copy button (for convenience)
    let copy_btn = Button::with_label("📋 コピー");
    let text_view_copy = text_view.clone();
    copy_btn.connect_clicked(move |_| {
        let buffer = text_view_copy.buffer();
        let start_iter = buffer.start_iter();
        let end_iter = buffer.end_iter();
        let json_text = buffer.text(&start_iter, &end_iter, false).to_string();

        // Use GTK4's built-in clipboard
        let clipboard = text_view_copy.clipboard();
        clipboard.set_text(&json_text);
        tracing::debug!("✅ JSON copied to clipboard");
    });
    button_box.append(&copy_btn);

    main_box.append(&button_box);
    content_area.append(&main_box);

    dialog.present();
}
