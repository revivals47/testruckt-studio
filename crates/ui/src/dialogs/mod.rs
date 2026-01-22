//! Dialog modules for the application
//!
//! Note: Some dialogs use deprecated GTK4 4.10 APIs (Dialog, FileChooserNative, etc.)
//! These will be migrated to the new APIs (Window, FileDialog, AlertDialog) in a future update.
#![allow(deprecated)]

pub mod about_dialog;
pub mod image_dialog;
pub mod item_dialog;
pub mod json_editor;
mod project_settings;
pub mod recent_files_dialog;
pub mod shortcuts_dialog;
pub mod template_browser;
pub mod unsaved_dialog;
pub mod user_manual_dialog;

pub use about_dialog::{get_app_name, get_version, show_about_dialog, APP_NAME, APP_VERSION};
pub use image_dialog::{show_image_chooser, show_image_chooser_async};
pub use item_dialog::{create_new_item, delete_item};
pub use json_editor::show_json_editor;
pub use project_settings::show_project_settings;
pub use recent_files_dialog::show_recent_files_dialog;
pub use shortcuts_dialog::show_shortcuts_dialog;
pub use template_browser::show_template_browser_async;
pub use unsaved_dialog::{check_unsaved_changes, show_unsaved_dialog, UnsavedDialogResponse};
pub use user_manual_dialog::show_user_manual_dialog;
