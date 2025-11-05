//! File I/O operations for document persistence
//!
//! Provides functionality for saving and loading documents in JSON format,
//! along with GTK4 file dialog integration.

pub mod file_io;
pub mod file_dialog;

pub use file_io::{save_document, load_document, default_documents_dir, default_filename};
pub use file_dialog::{show_open_dialog, show_save_dialog, show_export_dialog};
