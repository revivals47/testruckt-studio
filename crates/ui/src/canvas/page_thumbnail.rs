//! Page thumbnail/preview generation
//!
//! Generate small preview images of pages for the pages panel

use cairo::{Context, Format, ImageSurface};
use testruct_core::document::Page;
use testruct_core::layout::Rect;

/// Page thumbnail dimensions
pub const THUMBNAIL_WIDTH: i32 = 180;
pub const THUMBNAIL_HEIGHT: i32 = 120;

/// Generate a thumbnail preview of a page
pub fn generate_page_thumbnail(page: &Page) -> Result<Vec<u8>, String> {
    // Create a Cairo surface for the thumbnail
    let surface = ImageSurface::create(Format::Rgb24, THUMBNAIL_WIDTH, THUMBNAIL_HEIGHT)
        .map_err(|e| format!("Failed to create surface: {:?}", e))?;

    let context =
        Context::new(&surface).map_err(|e| format!("Failed to create context: {:?}", e))?;

    // Draw white background
    context.set_source_rgb(1.0, 1.0, 1.0);
    context
        .paint()
        .map_err(|e| format!("Failed to paint background: {:?}", e))?;

    // Draw a border
    context.set_source_rgb(0.8, 0.8, 0.8);
    context.set_line_width(1.0);
    context.rectangle(
        0.5,
        0.5,
        THUMBNAIL_WIDTH as f64 - 1.0,
        THUMBNAIL_HEIGHT as f64 - 1.0,
    );
    context
        .stroke()
        .map_err(|e| format!("Failed to stroke border: {:?}", e))?;

    // Render elements at small scale
    render_page_elements(&context, page)?;

    // Write to PNG buffer
    let mut png_data = Vec::new();
    surface
        .write_to_png(&mut png_data)
        .map_err(|e| format!("Failed to write PNG: {:?}", e))?;

    Ok(png_data)
}

/// Render page elements on thumbnail
fn render_page_elements(context: &Context, page: &Page) -> Result<(), String> {
    // Scale factor to fit content to thumbnail
    let scale_x = (THUMBNAIL_WIDTH as f64 - 4.0) / 800.0; // Assume 800px page width
    let scale_y = (THUMBNAIL_HEIGHT as f64 - 4.0) / 600.0; // Assume 600px page height

    context.set_source_rgb(0.2, 0.2, 0.2);
    context.set_line_width(0.5);

    for element in &page.elements {
        match element {
            testruct_core::document::DocumentElement::Shape(shape) => {
                let bounds = &shape.bounds;
                let x = 2.0 + bounds.origin.x as f64 * scale_x;
                let y = 2.0 + bounds.origin.y as f64 * scale_y;
                let w = bounds.size.width as f64 * scale_x;
                let h = bounds.size.height as f64 * scale_y;

                // Draw shape representation
                match shape.kind {
                    testruct_core::document::ShapeKind::Rectangle => {
                        context.rectangle(x, y, w, h);
                    }
                    testruct_core::document::ShapeKind::Ellipse => {
                        context
                            .save()
                            .map_err(|e| format!("Failed to save context: {:?}", e))?;
                        context.translate(x + w / 2.0, y + h / 2.0);
                        context.scale(w / 2.0, h / 2.0);
                        context.arc(0.0, 0.0, 1.0, 0.0, 2.0 * std::f64::consts::PI);
                        context
                            .restore()
                            .map_err(|e| format!("Failed to restore context: {:?}", e))?;
                    }
                    testruct_core::document::ShapeKind::Line => {
                        context.move_to(x, y);
                        context.line_to(x + w, y + h);
                    }
                    _ => {
                        // Draw rectangle for other shapes
                        context.rectangle(x, y, w, h);
                    }
                }
            }
            testruct_core::document::DocumentElement::Text(text) => {
                let bounds = &text.bounds;
                let x = 2.0 + bounds.origin.x as f64 * scale_x;
                let y = 2.0 + bounds.origin.y as f64 * scale_y;
                let w = bounds.size.width as f64 * scale_x;
                let h = bounds.size.height as f64 * scale_y;

                // Draw text box representation
                context.rectangle(x, y, w, h);
            }
            testruct_core::document::DocumentElement::Image(image) => {
                let bounds = &image.bounds;
                let x = 2.0 + bounds.origin.x as f64 * scale_x;
                let y = 2.0 + bounds.origin.y as f64 * scale_y;
                let w = bounds.size.width as f64 * scale_x;
                let h = bounds.size.height as f64 * scale_y;

                // Draw image placeholder with diagonal
                context.rectangle(x, y, w, h);
                context
                    .stroke()
                    .map_err(|e| format!("Failed to stroke: {:?}", e))?;
                context.move_to(x, y);
                context.line_to(x + w, y + h);
            }
            testruct_core::document::DocumentElement::Frame(frame) => {
                let bounds = &frame.bounds;
                let x = 2.0 + bounds.origin.x as f64 * scale_x;
                let y = 2.0 + bounds.origin.y as f64 * scale_y;
                let w = bounds.size.width as f64 * scale_x;
                let h = bounds.size.height as f64 * scale_y;

                // Draw frame with dashed appearance
                context.set_dash(&[2.0, 2.0], 0.0);
                context.rectangle(x, y, w, h);
            }
            testruct_core::document::DocumentElement::Group(group) => {
                let bounds = &group.bounds;
                let x = 2.0 + bounds.origin.x as f64 * scale_x;
                let y = 2.0 + bounds.origin.y as f64 * scale_y;
                let w = bounds.size.width as f64 * scale_x;
                let h = bounds.size.height as f64 * scale_y;

                // Draw group rectangle
                context.set_dash(&[1.0, 1.0], 0.0);
                context.set_source_rgb(0.5, 0.5, 0.5);
                context.rectangle(x, y, w, h);
            }
        }
    }

    context.set_dash(&[], 0.0);
    context
        .stroke()
        .map_err(|e| format!("Failed to stroke: {:?}", e))?;

    Ok(())
}

/// Get thumbnail cache key for a page
pub fn get_thumbnail_cache_key(page_id: testruct_core::document::PageId) -> String {
    format!("page_thumbnail_{:?}", page_id)
}

/// Check if page content has changed (simple version)
pub fn has_page_changed(page: &Page, last_hash: Option<u64>) -> bool {
    // Simple hash of element count and types
    let current_hash = page.elements.len() as u64;
    last_hash != Some(current_hash)
}
