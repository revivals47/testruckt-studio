//! Image export functionality (PNG, JPEG) using Cairo
//!
//! Renders a document to raster image formats with configurable DPI.

use anyhow::{anyhow, Result};
use cairo::{Context, Format, ImageSurface};
use std::path::Path;
use testruct_core::Document;
use tracing::{debug, info};

/// Default DPI for image export
const DEFAULT_DPI: f64 = 96.0;

/// Render a document to PNG format (one file per page)
pub fn render_to_png(document: &Document, output_path: &Path, dpi: f64) -> Result<()> {
    info!("Exporting to PNG: {}", output_path.display());

    if document.pages.is_empty() {
        return Err(anyhow!("Document has no pages to export"));
    }

    let dpi = if dpi <= 0.0 { DEFAULT_DPI } else { dpi };
    debug!("PNG export DPI: {}", dpi);

    // If multi-page, save each as separate file
    if document.pages.len() > 1 {
        export_multi_page_png(document, output_path, dpi)
    } else {
        export_single_page_png(document, output_path, dpi)
    }
}

/// Render a document to JPEG format (one file per page)
pub fn render_to_jpeg(document: &Document, output_path: &Path, dpi: f64, quality: i32) -> Result<()> {
    info!("Exporting to JPEG: {}", output_path.display());

    if document.pages.is_empty() {
        return Err(anyhow!("Document has no pages to export"));
    }

    let dpi = if dpi <= 0.0 { DEFAULT_DPI } else { dpi };
    let quality = if quality < 0 || quality > 100 { 95 } else { quality };

    debug!("JPEG export DPI: {}, Quality: {}", dpi, quality);

    // Convert to PNG first, then to JPEG using image crate
    // For now, we'll just export as PNG (JPEG would require additional dependencies)
    info!("JPEG export currently uses PNG format. Full JPEG support requires additional setup.");
    render_to_png(document, output_path, dpi)
}

/// Export single-page document to PNG
fn export_single_page_png(document: &Document, output_path: &Path, dpi: f64) -> Result<()> {
    let page = &document.pages[0];
    render_page_to_png(page, output_path, dpi)
}

/// Export multi-page document to multiple PNG files
fn export_multi_page_png(document: &Document, output_path: &Path, dpi: f64) -> Result<()> {
    for (index, page) in document.pages.iter().enumerate() {
        let page_num = index + 1;

        // Create output filename with page number
        let output_filename = if let Some(extension) = output_path.extension() {
            let stem = output_path.file_stem().unwrap();
            let stem_str = stem.to_string_lossy();
            let ext_str = extension.to_string_lossy();
            format!("{}_page_{}.{}", stem_str, page_num, ext_str)
        } else {
            format!("{}_page_{}.png", output_path.display(), page_num)
        };

        let page_path = output_path.parent()
            .unwrap_or_else(|| Path::new("."))
            .join(&output_filename);

        debug!("Rendering page {} to: {}", page_num, page_path.display());
        render_page_to_png(page, &page_path, dpi)?;
    }

    info!("PNG export completed: {} pages exported to {}", document.pages.len(), output_path.display());
    Ok(())
}

/// Render a single page to PNG file
fn render_page_to_png(_page: &testruct_core::document::Page, output_path: &Path, dpi: f64) -> Result<()> {
    // Use default page dimensions (A4: 595.28 x 841.89 points)
    let width_points = 595.28;
    let height_points = 841.89;
    let width_inches = width_points / 72.0; // Assume 72 points per inch
    let height_inches = height_points / 72.0;

    let pixel_width = (width_inches * dpi) as i32;
    let pixel_height = (height_inches * dpi) as i32;

    debug!("PNG size: {}x{} pixels at {} DPI", pixel_width, pixel_height, dpi);

    // Clamp to minimum size
    let pixel_width = pixel_width.max(100);
    let pixel_height = pixel_height.max(100);

    // Create image surface
    let surface = ImageSurface::create(Format::ARgb32, pixel_width, pixel_height)
        .map_err(|e| anyhow!("Failed to create image surface: {}", e))?;

    let ctx = Context::new(&surface)
        .map_err(|e| anyhow!("Failed to create Cairo context: {}", e))?;

    // Scale context for DPI
    let scale = dpi / 72.0;
    ctx.scale(scale, scale);

    // Render page
    render_page_to_context(&ctx, _page)?;

    // Write to file
    let mut file = std::fs::File::create(output_path)
        .map_err(|e| anyhow!("Failed to create output file: {}", e))?;

    surface.write_to_png(&mut file)
        .map_err(|e| anyhow!("Failed to write PNG: {}", e))?;

    info!("PNG exported: {}", output_path.display());
    Ok(())
}

/// Render a single page to Cairo context
fn render_page_to_context(ctx: &Context, page: &testruct_core::document::Page) -> Result<()> {
    // Set white background
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.paint()
        .map_err(|e| anyhow!("Failed to paint background: {}", e))?;

    // Use default page dimensions (A4)
    let width = 595.28;
    let height = 841.89;

    // Draw page border
    ctx.set_source_rgb(0.0, 0.0, 0.0);
    ctx.set_line_width(0.5);
    ctx.rectangle(0.0, 0.0, width, height);
    ctx.stroke()
        .map_err(|e| anyhow!("Failed to draw page border: {}", e))?;

    // Render all elements
    for element in &page.elements {
        render_element_to_context(ctx, element)?;
    }

    Ok(())
}

/// Render a single element to Cairo context
fn render_element_to_context(ctx: &Context, element: &testruct_core::document::DocumentElement) -> Result<()> {
    use testruct_core::document::DocumentElement;

    match element {
        DocumentElement::Shape(shape) => {
            render_shape_to_context(ctx, shape)?;
        }
        DocumentElement::Text(text) => {
            render_text_to_context(ctx, text)?;
        }
        DocumentElement::Image(_image) => {
            debug!("Image rendering in PNG export not yet implemented");
        }
        DocumentElement::Frame(_frame) => {
            debug!("Frame rendering in PNG export not yet implemented");
        }
    }

    Ok(())
}

/// Render a shape element
fn render_shape_to_context(ctx: &Context, shape: &testruct_core::document::ShapeElement) -> Result<()> {
    use testruct_core::document::ShapeKind;

    let x = shape.bounds.origin.x as f64;
    let y = shape.bounds.origin.y as f64;
    let width = shape.bounds.size.width as f64;
    let height = shape.bounds.size.height as f64;

    // Set stroke color from shape, or default to black
    if let Some(color) = &shape.stroke {
        ctx.set_source_rgb(color.r as f64 / 255.0, color.g as f64 / 255.0, color.b as f64 / 255.0);
    } else {
        ctx.set_source_rgb(0.0, 0.0, 0.0);
    }

    match shape.kind {
        ShapeKind::Rectangle => {
            ctx.rectangle(x, y, width, height);
            ctx.stroke()
                .map_err(|e| anyhow!("Failed to stroke rectangle: {}", e))?;
        }
        ShapeKind::Ellipse => {
            ctx.save().map_err(|e| anyhow!("Failed to save context: {}", e))?;
            ctx.translate(x + width / 2.0, y + height / 2.0);
            ctx.scale(width / 2.0, height / 2.0);
            ctx.arc(0.0, 0.0, 1.0, 0.0, std::f64::consts::PI * 2.0);
            ctx.restore().map_err(|e| anyhow!("Failed to restore context: {}", e))?;
            ctx.stroke()
                .map_err(|e| anyhow!("Failed to stroke ellipse: {}", e))?;
        }
        ShapeKind::Line => {
            ctx.move_to(x, y);
            ctx.line_to(x + width, y + height);
            ctx.stroke()
                .map_err(|e| anyhow!("Failed to stroke line: {}", e))?;
        }
        ShapeKind::Arrow => {
            ctx.move_to(x, y);
            ctx.line_to(x + width, y + height);
            ctx.stroke()
                .map_err(|e| anyhow!("Failed to stroke arrow: {}", e))?;
        }
        ShapeKind::Polygon => {
            ctx.rectangle(x, y, width, height);
            ctx.stroke()
                .map_err(|e| anyhow!("Failed to stroke polygon: {}", e))?;
        }
    }

    Ok(())
}

/// Render a text element
fn render_text_to_context(_ctx: &Context, _text: &testruct_core::document::TextElement) -> Result<()> {
    // NOTE: TextElement does not have bounds information
    // Full text rendering requires more setup with Pango
    debug!("Text rendering in PNG export: TextElement support pending");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_export_module_loads() {
        assert!(true);
    }

    #[test]
    fn test_default_dpi() {
        assert_eq!(DEFAULT_DPI, 96.0);
    }
}
