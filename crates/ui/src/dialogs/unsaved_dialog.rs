//! Unsaved changes confirmation dialog
//!
//! Provides a dialog to confirm saving changes before destructive operations.

use gtk4::prelude::*;
use gtk4::{AlertDialog, Window};

/// Response from the unsaved changes dialog
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnsavedDialogResponse {
    /// User wants to save changes
    Save,
    /// User wants to discard changes
    DontSave,
    /// User cancelled the operation
    Cancel,
}

/// Show an unsaved changes confirmation dialog
///
/// Returns the user's choice asynchronously.
///
/// # Arguments
///
/// * `parent` - The parent window for the dialog
///
/// # Returns
///
/// The user's response indicating whether to save, discard, or cancel
pub async fn show_unsaved_dialog(parent: &Window) -> UnsavedDialogResponse {
    let dialog = AlertDialog::builder()
        .modal(true)
        .message("Save changes?")
        .detail("Your document has unsaved changes. Do you want to save them before continuing?")
        .build();

    // Set up buttons: Cancel (0), Don't Save (1), Save (2)
    dialog.set_buttons(&["Cancel", "Don't Save", "Save"]);
    dialog.set_cancel_button(0);
    dialog.set_default_button(2);

    match dialog.choose_future(Some(parent)).await {
        Ok(response) => match response {
            0 => UnsavedDialogResponse::Cancel,
            1 => UnsavedDialogResponse::DontSave,
            2 => UnsavedDialogResponse::Save,
            _ => UnsavedDialogResponse::Cancel,
        },
        Err(_) => UnsavedDialogResponse::Cancel,
    }
}

/// Check if document is modified and show dialog if needed
///
/// Returns true if the operation should proceed, false if cancelled.
///
/// # Arguments
///
/// * `state` - The application state
/// * `parent` - The parent window for the dialog
///
/// # Returns
///
/// `true` if the operation should continue, `false` if the user cancelled
pub async fn check_unsaved_changes(
    state: &crate::app::AppState,
    parent: &Window,
) -> bool {
    if !state.is_modified() {
        return true;
    }

    match show_unsaved_dialog(parent).await {
        UnsavedDialogResponse::Save => {
            // Save the document first
            if let Some(path) = state.current_file_path() {
                // Overwrite existing file
                if let Some(document) = state.active_document() {
                    match crate::io::file_io::save_document(&document, &path) {
                        Ok(_) => {
                            state.mark_as_saved(path);
                            tracing::info!("✅ Document saved before operation");
                            true
                        }
                        Err(e) => {
                            tracing::error!("❌ Failed to save document: {}", e);
                            false
                        }
                    }
                } else {
                    false
                }
            } else {
                // No existing path - need to show save dialog
                // For simplicity, we'll show the save dialog here
                if let Some(save_path) = crate::io::file_dialog::show_save_dialog(
                    &parent.clone().downcast::<gtk4::ApplicationWindow>().unwrap()
                ).await {
                    if let Some(document) = state.active_document() {
                        match crate::io::file_io::save_document(&document, &save_path) {
                            Ok(_) => {
                                state.add_recent_file(save_path.clone());
                                state.mark_as_saved(save_path);
                                tracing::info!("✅ Document saved before operation");
                                true
                            }
                            Err(e) => {
                                tracing::error!("❌ Failed to save document: {}", e);
                                false
                            }
                        }
                    } else {
                        false
                    }
                } else {
                    // User cancelled save dialog
                    false
                }
            }
        }
        UnsavedDialogResponse::DontSave => {
            // Proceed without saving
            true
        }
        UnsavedDialogResponse::Cancel => {
            // Cancel the operation
            false
        }
    }
}
