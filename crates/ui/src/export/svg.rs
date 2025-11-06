//! SVG export functionality using Cairo
//!
//! Renders a document to SVG format with support for multi-page output.

use anyhow::{anyhow, Result};
use cairo::{Context, SvgSurface};
use std::path::Path;
use testruct_core::workspace::assets::AssetCatalog;
use testruct_core::Document;
use tracing::{debug, info};

/// Default page size (A4: 595.28 x 841.89 points)
const DEFAULT_PAGE_WIDTH: f64 = 595.28;
const DEFAULT_PAGE_HEIGHT: f64 = 841.89;

/// Render a document to SVG
pub fn render_to_svg(
    document: &Document,
    output_path: &Path,
    catalog: &AssetCatalog,
) -> Result<()> {
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

    let ctx =
        Context::new(&surface).map_err(|e| anyhow!("Failed to create Cairo context: {}", e))?;

    // Render each page
    for (page_index, page) in document.pages.iter().enumerate() {
        debug!("Rendering page {}", page_index + 1);
        render_page_to_context(&ctx, page, catalog)?;

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
fn render_page_to_context(
    ctx: &Context,
    page: &testruct_core::document::Page,
    catalog: &AssetCatalog,
) -> Result<()> {
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
        render_element_to_context(ctx, element, catalog)?;
    }

    Ok(())
}

/// Render a single element to Cairo context
fn render_element_to_context(
    ctx: &Context,
    element: &testruct_core::document::DocumentElement,
    catalog: &AssetCatalog,
) -> Result<()> {
    use testruct_core::document::DocumentElement;

    match element {
        DocumentElement::Shape(shape) => {
            render_shape_to_context(ctx, shape)?;
        }
        DocumentElement::Text(text) => {
            render_text_to_context(ctx, text)?;
        }
        DocumentElement::Image(image) => {
            // Try to load image from catalog, fallback to placeholder if not found
            match crate::export::image_utils::render_image_from_asset(
                ctx,
                image.source,
                catalog,
                &image.bounds,
            ) {
                Ok(_) => {
                    debug!("Image rendered from asset catalog: {}", image.id);
                }
                Err(e) => {
                    // If loading fails, draw placeholder and log warning
                    debug!(
                        "Failed to render image {}: {}, using placeholder",
                        image.id, e
                    );
                    let _ = crate::export::image_utils::draw_image_placeholder(ctx, &image.bounds);
                }
            }
        }
        DocumentElement::Frame(frame) => {
            render_frame_to_context(ctx, frame, catalog)?;
        }
    }

    Ok(())
}

/// Render a shape element
fn render_shape_to_context(
    ctx: &Context,
    shape: &testruct_core::document::ShapeElement,
) -> Result<()> {
    use testruct_core::document::ShapeKind;

    let x = shape.bounds.origin.x as f64;
    let y = shape.bounds.origin.y as f64;
    let width = shape.bounds.size.width as f64;
    let height = shape.bounds.size.height as f64;

    // Render based on shape kind
    match shape.kind {
        ShapeKind::Rectangle => {
            // Draw fill color if present
            if let Some(fill) = &shape.fill {
                ctx.set_source_rgb(fill.r as f64, fill.g as f64, fill.b as f64);
                ctx.rectangle(x, y, width, height);
                ctx.fill()
                    .map_err(|e| anyhow!("Failed to fill rectangle: {}", e))?;
            }
            // Draw stroke if present
            if let Some(stroke) = &shape.stroke {
                ctx.set_source_rgb(stroke.r as f64, stroke.g as f64, stroke.b as f64);
                ctx.set_line_width(1.0);
                ctx.rectangle(x, y, width, height);
                ctx.stroke()
                    .map_err(|e| anyhow!("Failed to stroke rectangle: {}", e))?;
            }
        }
        ShapeKind::Ellipse => {
            ctx.save()
                .map_err(|e| anyhow!("Failed to save context: {}", e))?;
            ctx.translate(x + width / 2.0, y + height / 2.0);
            ctx.scale(width / 2.0, height / 2.0);
            ctx.arc(0.0, 0.0, 1.0, 0.0, std::f64::consts::PI * 2.0);

            // Draw fill if present
            if let Some(fill) = &shape.fill {
                ctx.set_source_rgb(fill.r as f64, fill.g as f64, fill.b as f64);
                ctx.fill_preserve()
                    .map_err(|e| anyhow!("Failed to fill ellipse: {}", e))?;
            }
            // Draw stroke if present
            if let Some(stroke) = &shape.stroke {
                ctx.set_source_rgb(stroke.r as f64, stroke.g as f64, stroke.b as f64);
                ctx.set_line_width(1.0);
                ctx.stroke()
                    .map_err(|e| anyhow!("Failed to stroke ellipse: {}", e))?;
            }
            ctx.restore()
                .map_err(|e| anyhow!("Failed to restore context: {}", e))?;
        }
        ShapeKind::Line => {
            // Lines use stroke color
            if let Some(stroke) = &shape.stroke {
                ctx.set_source_rgb(stroke.r as f64, stroke.g as f64, stroke.b as f64);
                ctx.set_line_width(1.0);
                ctx.move_to(x, y);
                ctx.line_to(x + width, y + height);
                ctx.stroke()
                    .map_err(|e| anyhow!("Failed to stroke line: {}", e))?;
            }
        }
        ShapeKind::Arrow => {
            // Arrows use stroke color for the line
            if let Some(stroke) = &shape.stroke {
                ctx.set_source_rgb(stroke.r as f64, stroke.g as f64, stroke.b as f64);
                ctx.set_line_width(1.0);
                ctx.move_to(x, y);
                ctx.line_to(x + width, y + height);
                ctx.stroke()
                    .map_err(|e| anyhow!("Failed to stroke arrow: {}", e))?;
            }
        }
        ShapeKind::Polygon => {
            // Placeholder for polygon (same as rectangle)
            if let Some(fill) = &shape.fill {
                ctx.set_source_rgb(fill.r as f64, fill.g as f64, fill.b as f64);
                ctx.rectangle(x, y, width, height);
                ctx.fill()
                    .map_err(|e| anyhow!("Failed to fill polygon: {}", e))?;
            }
            if let Some(stroke) = &shape.stroke {
                ctx.set_source_rgb(stroke.r as f64, stroke.g as f64, stroke.b as f64);
                ctx.set_line_width(1.0);
                ctx.rectangle(x, y, width, height);
                ctx.stroke()
                    .map_err(|e| anyhow!("Failed to stroke polygon: {}", e))?;
            }
        }
    }

    Ok(())
}

/// Render a frame element (with recursive children)
fn render_frame_to_context(
    ctx: &Context,
    frame: &testruct_core::document::FrameElement,
    catalog: &AssetCatalog,
) -> Result<()> {
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
        render_element_to_context(ctx, child, catalog)?;
    }

    debug!("Frame rendered with {} children", frame.children.len());
    Ok(())
}

/// Render a text element with Pango layout
fn render_text_to_context(
    ctx: &Context,
    text: &testruct_core::document::TextElement,
) -> Result<()> {
    ctx.save()
        .map_err(|e| anyhow!("Failed to save context: {}", e))?;

    let bounds = &text.bounds;
    let style = &text.style;

    // Draw background color if specified
    if let Some(bg_color) = style.background_color {
        ctx.set_source_rgb(bg_color.r as f64, bg_color.g as f64, bg_color.b as f64);
        ctx.rectangle(
            bounds.origin.x as f64,
            bounds.origin.y as f64,
            bounds.size.width as f64,
            bounds.size.height as f64,
        );
        ctx.fill()
            .map_err(|e| anyhow!("Failed to fill text background: {}", e))?;
    }

    // Translate to text origin
    ctx.translate(bounds.origin.x as f64, bounds.origin.y as f64);

    // Create clipping rectangle
    ctx.rectangle(
        0.0,
        0.0,
        bounds.size.width as f64,
        bounds.size.height as f64,
    );
    ctx.clip();

    // Create Pango layout
    let layout = pangocairo::functions::create_layout(ctx);
    layout.set_text(&text.content);

    // Set font with styling
    let mut font_desc = pango::FontDescription::new();
    font_desc.set_family(&style.font_family);
    font_desc.set_size((style.font_size * pango::SCALE as f32) as i32);

    // Apply font weight
    let pango_weight = match style.weight {
        testruct_core::typography::FontWeight::Thin => pango::Weight::Thin,
        testruct_core::typography::FontWeight::Light => pango::Weight::Light,
        testruct_core::typography::FontWeight::Regular => pango::Weight::Normal,
        testruct_core::typography::FontWeight::Medium => pango::Weight::Medium,
        testruct_core::typography::FontWeight::Bold => pango::Weight::Bold,
        testruct_core::typography::FontWeight::Black => pango::Weight::Ultrabold,
    };
    font_desc.set_weight(pango_weight);

    // Apply italic style
    if style.italic {
        font_desc.set_style(pango::Style::Italic);
    }

    layout.set_font_description(Some(&font_desc));

    // Apply text alignment
    let pango_alignment = match style.alignment {
        testruct_core::typography::TextAlignment::Start => pango::Alignment::Left,
        testruct_core::typography::TextAlignment::Center => pango::Alignment::Center,
        testruct_core::typography::TextAlignment::End => pango::Alignment::Right,
        testruct_core::typography::TextAlignment::Justified => pango::Alignment::Center,
    };
    layout.set_alignment(pango_alignment);
    layout.set_width((bounds.size.width as f64 * pango::SCALE as f64) as i32);

    // Apply underline and strikethrough decorations
    if style.underline || style.strikethrough {
        let attrs = pango::AttrList::new();
        if style.underline {
            let underline_attr = pango::AttrInt::new_underline(pango::Underline::Single);
            attrs.insert(underline_attr);
        }
        if style.strikethrough {
            let strikethrough_attr = pango::AttrInt::new_strikethrough(true);
            attrs.insert(strikethrough_attr);
        }
        layout.set_attributes(Some(&attrs));
    }

    // Set text color
    ctx.set_source_rgb(
        style.color.r as f64,
        style.color.g as f64,
        style.color.b as f64,
    );

    // Render the text using pangocairo
    pangocairo::functions::show_layout(ctx, &layout);

    ctx.restore()
        .map_err(|e| anyhow!("Failed to restore context: {}", e))?;

    debug!("Text rendered: '{}' in SVG", text.content);
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
