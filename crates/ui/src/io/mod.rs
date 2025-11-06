//! File I/O operations for document persistence
//!
//! Provides functionality for saving and loading documents in JSON format,
//! along with GTK4 file dialog integration.

pub mod file_dialog;
pub mod file_io;

pub use file_dialog::{show_export_dialog, show_open_dialog, show_save_dialog};
pub use file_io::{default_documents_dir, default_filename, load_document, save_document};
