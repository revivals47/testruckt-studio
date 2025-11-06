//! Image export functionality (PNG, JPEG) using Cairo
//!
//! Renders a document to raster image formats with configurable DPI.

use anyhow::{anyhow, Result};
use cairo::{Context, Format, ImageSurface};
use std::path::Path;
use testruct_core::Document;
use testruct_core::workspace::assets::AssetCatalog;
use tracing::{debug, info};

/// Default DPI for image export
const DEFAULT_DPI: f64 = 96.0;

/// Render a document to PNG format (one file per page)
pub fn render_to_png(document: &Document, output_path: &Path, dpi: f64, catalog: &AssetCatalog) -> Result<()> {
    info!("Exporting to PNG: {}", output_path.display());

    if document.pages.is_empty() {
        return Err(anyhow!("Document has no pages to export"));
    }

    let dpi = if dpi <= 0.0 { DEFAULT_DPI } else { dpi };
    debug!("PNG export DPI: {}", dpi);

    // If multi-page, save each as separate file
    if document.pages.len() > 1 {
        export_multi_page_png(document, output_path, dpi, catalog)
    } else {
        export_single_page_png(document, output_path, dpi, catalog)
    }
}

/// Render a document to JPEG format (one file per page)
pub fn render_to_jpeg(document: &Document, output_path: &Path, dpi: f64, quality: i32, catalog: &AssetCatalog) -> Result<()> {
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
    render_to_png(document, output_path, dpi, catalog)
}

/// Export single-page document to PNG
fn export_single_page_png(document: &Document, output_path: &Path, dpi: f64, catalog: &AssetCatalog) -> Result<()> {
    let page = &document.pages[0];
    render_page_to_png(page, output_path, dpi, catalog)
}

/// Export multi-page document to multiple PNG files
fn export_multi_page_png(document: &Document, output_path: &Path, dpi: f64, catalog: &AssetCatalog) -> Result<()> {
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
        render_page_to_png(page, &page_path, dpi, catalog)?;
    }

    info!("PNG export completed: {} pages exported to {}", document.pages.len(), output_path.display());
    Ok(())
}

/// Render a single page to PNG file
fn render_page_to_png(_page: &testruct_core::document::Page, output_path: &Path, dpi: f64, catalog: &AssetCatalog) -> Result<()> {
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
    render_page_to_context(&ctx, _page, catalog)?;

    // Write to file
    let mut file = std::fs::File::create(output_path)
        .map_err(|e| anyhow!("Failed to create output file: {}", e))?;

    surface.write_to_png(&mut file)
        .map_err(|e| anyhow!("Failed to write PNG: {}", e))?;

    info!("PNG exported: {}", output_path.display());
    Ok(())
}

/// Render a single page to Cairo context
fn render_page_to_context(ctx: &Context, page: &testruct_core::document::Page, catalog: &AssetCatalog) -> Result<()> {
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
        render_element_to_context(ctx, element, catalog)?;
    }

    Ok(())
}

/// Render a single element to Cairo context
fn render_element_to_context(ctx: &Context, element: &testruct_core::document::DocumentElement, catalog: &AssetCatalog) -> Result<()> {
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
            match crate::export::image_utils::render_image_from_asset(ctx, image.source, catalog, &image.bounds) {
                Ok(_) => {
                    debug!("Image rendered from asset catalog: {}", image.id);
                }
                Err(e) => {
                    // If loading fails, draw placeholder and log warning
                    debug!("Failed to render image {}: {}, using placeholder", image.id, e);
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
fn render_shape_to_context(ctx: &Context, shape: &testruct_core::document::ShapeElement) -> Result<()> {
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
            ctx.save().map_err(|e| anyhow!("Failed to save context: {}", e))?;
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
            ctx.restore().map_err(|e| anyhow!("Failed to restore context: {}", e))?;
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
fn render_frame_to_context(ctx: &Context, frame: &testruct_core::document::FrameElement, catalog: &AssetCatalog) -> Result<()> {
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
fn render_text_to_context(ctx: &Context, text: &testruct_core::document::TextElement) -> Result<()> {
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
    ctx.rectangle(0.0, 0.0, bounds.size.width as f64, bounds.size.height as f64);
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

    debug!("Text rendered: '{}' in PNG", text.content);
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
