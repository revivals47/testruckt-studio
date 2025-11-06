//! Document export functionality
//!
//! Provides PDF, PNG, SVG, and other format export capabilities using Cairo rendering.

pub mod image;
pub mod image_utils;
pub mod pdf;
pub mod svg;

use anyhow::Result;
use std::path::Path;
use testruct_core::workspace::assets::AssetCatalog;
use testruct_core::Document;

/// Export a document to PDF format
pub fn export_pdf(document: &Document, output_path: &Path, catalog: &AssetCatalog) -> Result<()> {
    pdf::render_to_pdf(document, output_path, catalog)
}

/// Export a document to PNG format
pub fn export_png(
    document: &Document,
    output_path: &Path,
    dpi: f64,
    catalog: &AssetCatalog,
) -> Result<()> {
    image::render_to_png(document, output_path, dpi, catalog)
}

/// Export a document to JPEG format
pub fn export_jpeg(
    document: &Document,
    output_path: &Path,
    dpi: f64,
    quality: i32,
    catalog: &AssetCatalog,
) -> Result<()> {
    image::render_to_jpeg(document, output_path, dpi, quality, catalog)
}

/// Export a document to SVG format
pub fn export_svg(document: &Document, output_path: &Path, catalog: &AssetCatalog) -> Result<()> {
    svg::render_to_svg(document, output_path, catalog)
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
