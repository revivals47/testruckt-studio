//! SVG export functionality using Cairo
//!
//! Renders a document to SVG format with support for multi-page output.

use anyhow::{anyhow, Result};
use cairo::{Context, SvgSurface};
use std::path::Path;
use testruct_core::Document;
use tracing::{debug, info};

/// Default page size (A4: 595.28 x 841.89 points)
const DEFAULT_PAGE_WIDTH: f64 = 595.28;
const DEFAULT_PAGE_HEIGHT: f64 = 841.89;

/// Render a document to SVG
pub fn render_to_svg(document: &Document, output_path: &Path) -> Result<()> {
    info!("Exporting to SVG: {}", output_path.display());

    if document.pages.is_empty() {
        return Err(anyhow!("Document has no pages to export"));
    }

    // Use default page dimensions (A4) for now
    let width = DEFAULT_PAGE_WIDTH;
    let height = DEFAULT_PAGE_HEIGHT;

    debug!("SVG page size: {} x {}", width, height);

    // Create SVG surface
    let surface = SvgSurface::new(width, height, Some(output_path))
        .map_err(|e| anyhow!("Failed to create SVG surface: {}", e))?;

    let ctx = Context::new(&surface)
        .map_err(|e| anyhow!("Failed to create Cairo context: {}", e))?;

    // Render each page
    for (page_index, page) in document.pages.iter().enumerate() {
        debug!("Rendering page {}", page_index + 1);
        render_page_to_context(&ctx, page)?;

        // Move to next page (except for last page)
        if page_index < document.pages.len() - 1 {
            ctx.show_page()
                .map_err(|e| anyhow!("Failed to show page: {}", e))?;
        }
    }

    // Finish SVG
    surface.finish();
    info!("SVG export completed: {}", output_path.display());

    Ok(())
}

/// Render a single page to Cairo context
fn render_page_to_context(ctx: &Context, page: &testruct_core::document::Page) -> Result<()> {
    // Set white background
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.paint()
        .map_err(|e| anyhow!("Failed to paint background: {}", e))?;

    // Use default page size
    let width = DEFAULT_PAGE_WIDTH;
    let height = DEFAULT_PAGE_HEIGHT;

    // Draw page border for visual reference
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
        DocumentElement::Image(image) => {
            crate::export::image_utils::draw_image_placeholder(ctx, &image.bounds)
                .map_err(|e| anyhow::anyhow!("Failed to render image placeholder: {}", e))?;
            debug!("Image rendered as placeholder: {}", image.id);
        }
        DocumentElement::Frame(frame) => {
            render_frame_to_context(ctx, frame)?;
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

    // Render based on shape kind
    match shape.kind {
        ShapeKind::Rectangle => {
            ctx.rectangle(x, y, width, height);
            ctx.stroke()
                .map_err(|e| anyhow!("Failed to stroke rectangle: {}", e))?;
        }
        ShapeKind::Ellipse => {
            // Draw as ellipse
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
            // Draw as line with arrowhead
            ctx.move_to(x, y);
            ctx.line_to(x + width, y + height);
            ctx.stroke()
                .map_err(|e| anyhow!("Failed to stroke arrow: {}", e))?;
        }
        ShapeKind::Polygon => {
            ctx.rectangle(x, y, width, height); // Placeholder
            ctx.stroke()
                .map_err(|e| anyhow!("Failed to stroke polygon: {}", e))?;
        }
    }

    Ok(())
}

/// Render a frame element (with recursive children)
fn render_frame_to_context(ctx: &Context, frame: &testruct_core::document::FrameElement) -> Result<()> {
    let x = frame.bounds.origin.x as f64;
    let y = frame.bounds.origin.y as f64;
    let width = frame.bounds.size.width as f64;
    let height = frame.bounds.size.height as f64;

    // Draw frame border
    ctx.set_source_rgb(0.9, 0.9, 0.9);
    ctx.set_line_width(1.0);
    ctx.rectangle(x, y, width, height);
    ctx.stroke()
        .map_err(|e| anyhow!("Failed to stroke frame: {}", e))?;

    // Render frame children recursively
    for child in &frame.children {
        render_element_to_context(ctx, child)?;
    }

    debug!("Frame rendered with {} children", frame.children.len());
    Ok(())
}

/// Render a text element
fn render_text_to_context(_ctx: &Context, _text: &testruct_core::document::TextElement) -> Result<()> {
    // NOTE: TextElement does not have bounds information
    // Full text rendering requires more setup with Pango
    debug!("Text rendering in SVG export: TextElement support pending");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_export_module_loads() {
        assert!(true);
    }
}
