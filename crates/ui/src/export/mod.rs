//! Document export functionality
//!
//! Provides PDF, PNG, and other format export capabilities using Cairo rendering.

pub mod pdf;
pub mod image;

use std::path::Path;
use anyhow::Result;
use testruct_core::Document;

/// Export a document to PDF format
pub fn export_pdf(document: &Document, output_path: &Path) -> Result<()> {
    pdf::render_to_pdf(document, output_path)
}

/// Export a document to PNG format
pub fn export_png(document: &Document, output_path: &Path, dpi: f64) -> Result<()> {
    image::render_to_png(document, output_path, dpi)
}

/// Export a document to JPEG format
pub fn export_jpeg(document: &Document, output_path: &Path, dpi: f64, quality: i32) -> Result<()> {
    image::render_to_jpeg(document, output_path, dpi, quality)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_loads() {
        // Basic module loading test
        assert!(true);
    }
}
