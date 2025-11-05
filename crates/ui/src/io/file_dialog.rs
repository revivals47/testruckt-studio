//! File dialog operations for document I/O
//!
//! Provides GTK4 file chooser dialogs for opening and saving documents.

use gtk4::{prelude::*, FileDialog};
use std::path::PathBuf;

/// Show an open file dialog and return the selected path
pub async fn show_open_dialog(parent: &gtk4::ApplicationWindow) -> Option<PathBuf> {
    let dialog = FileDialog::new();
    dialog.set_title("Open Document");

    // Set initial folder
    if let Some(docs_dir) = crate::io::file_io::default_documents_dir() {
        let file = gtk4::gio::File::for_path(&docs_dir);
        dialog.set_initial_folder(Some(&file));
    }

    match dialog.open_future(Some(parent)).await {
        Ok(file) => file.path(),
        Err(_) => None,
    }
}

/// Show a save file dialog and return the selected path
pub async fn show_save_dialog(parent: &gtk4::ApplicationWindow) -> Option<PathBuf> {
    let dialog = FileDialog::new();
    dialog.set_title("Save Document");

    // Set initial folder and name
    if let Some(docs_dir) = crate::io::file_io::default_documents_dir() {
        let file = gtk4::gio::File::for_path(&docs_dir);
        dialog.set_initial_folder(Some(&file));
    }

    dialog.set_initial_name(Some(&crate::io::file_io::default_filename()));

    match dialog.save_future(Some(parent)).await {
        Ok(file) => file.path(),
        Err(_) => None,
    }
}

/// Show an export dialog for selecting an export location and format
pub async fn show_export_dialog(
    parent: &gtk4::ApplicationWindow,
    format: &str,
) -> Option<PathBuf> {
    let dialog = FileDialog::new();
    dialog.set_title(&format!("Export Document as {}", format.to_uppercase()));

    // Set initial folder and name
    if let Some(docs_dir) = crate::io::file_io::default_documents_dir() {
        let file = gtk4::gio::File::for_path(&docs_dir);
        dialog.set_initial_folder(Some(&file));
    }

    let extension = match format {
        "pdf" => "pdf",
        "png" => "png",
        "jpeg" => "jpg",
        "svg" => "svg",
        _ => "bin",
    };

    let filename = format!("document.{}", extension);
    dialog.set_initial_name(Some(&filename));

    match dialog.save_future(Some(parent)).await {
        Ok(file) => file.path(),
        Err(_) => None,
    }
}
