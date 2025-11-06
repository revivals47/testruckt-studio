//! Document file I/O operations
//!
//! Handles saving and loading Testruct documents in JSON format.

use anyhow::{Context, Result};
use std::path::Path;
use testruct_core::Document;

/// Save a document to a JSON file
pub fn save_document(document: &Document, path: &Path) -> Result<()> {
    let json =
        serde_json::to_string_pretty(document).context("Failed to serialize document to JSON")?;

    std::fs::write(path, json).context("Failed to write document file")?;

    tracing::info!("Document saved to: {}", path.display());
    Ok(())
}

/// Load a document from a JSON file
pub fn load_document(path: &Path) -> Result<Document> {
    let json = std::fs::read_to_string(path).context("Failed to read document file")?;

    let document: Document =
        serde_json::from_str(&json).context("Failed to deserialize document from JSON")?;

    tracing::info!("Document loaded from: {}", path.display());
    Ok(document)
}

/// Get the default documents directory
pub fn default_documents_dir() -> Option<std::path::PathBuf> {
    dirs::document_dir()
}

/// Get a default filename for a new document
pub fn default_filename() -> String {
    chrono::Local::now()
        .format("document_%Y%m%d_%H%M%S.json")
        .to_string()
}
