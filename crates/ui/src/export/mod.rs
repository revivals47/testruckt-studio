//! Document export functionality
//!
//! Provides PDF, PNG, SVG, and other format export capabilities using Cairo rendering.

pub mod pdf;
pub mod image;
pub mod svg;

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

/// Export a document to SVG format
pub fn export_svg(document: &Document, output_path: &Path) -> Result<()> {
    svg::render_to_svg(document, output_path)
}

/// Export selected frame/image to PDF format
pub fn export_frame_to_pdf(document: &Document, frame_id: uuid::Uuid, output_path: &Path) -> Result<()> {
    // For now, export entire document. In future, could support individual frame export
    tracing::info!("Exporting frame {} to PDF", frame_id);
    export_pdf(document, output_path)
}

/// Export selected image to PNG format
pub fn export_image_to_png(document: &Document, image_id: uuid::Uuid, output_path: &Path, dpi: f64) -> Result<()> {
    // For now, export entire document. In future, could support individual image export
    tracing::info!("Exporting image {} to PNG", image_id);
    export_png(document, output_path, dpi)
}

/// Export selected image to JPEG format
pub fn export_image_to_jpeg(document: &Document, image_id: uuid::Uuid, output_path: &Path, dpi: f64, quality: i32) -> Result<()> {
    // For now, export entire document. In future, could support individual image export
    tracing::info!("Exporting image {} to JPEG", image_id);
    export_jpeg(document, output_path, dpi, quality)
}

/// Export selected frame/image to SVG format
pub fn export_frame_to_svg(document: &Document, frame_id: uuid::Uuid, output_path: &Path) -> Result<()> {
    // For now, export entire document. In future, could support individual frame export
    tracing::info!("Exporting frame {} to SVG", frame_id);
    export_svg(document, output_path)
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
