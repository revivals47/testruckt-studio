//! Image loading and rendering utilities for export

use cairo::Context;
use std::path::Path;
use testruct_core::layout::Rect;
use testruct_core::workspace::assets::{AssetRef, AssetCatalog};

/// Render an image from AssetCatalog to a Cairo context
///
/// Resolves the asset reference to a file path and renders the image
pub fn render_image_from_asset(
    ctx: &Context,
    asset_ref: AssetRef,
    catalog: &AssetCatalog,
    bounds: &Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get image metadata from catalog
    if let Some(metadata) = catalog.get(asset_ref) {
        render_image_to_context(ctx, &metadata.path, bounds)
    } else {
        // Asset not found in catalog - draw placeholder
        draw_image_placeholder(ctx, bounds).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

/// Load an image file and render it to a Cairo context
///
/// Supports: JPEG, PNG, GIF, WebP
pub fn render_image_to_context(
    ctx: &Context,
    image_path: &Path,
    bounds: &Rect,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load the image file
    let img = image::open(image_path)?;

    // Resize image to match bounds while maintaining aspect ratio
    let width = bounds.size.width as u32;
    let height = bounds.size.height as u32;

    let resized = if width > 0 && height > 0 {
        img.resize_to_fill(width, height, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };

    // Convert to RGBA8 for Cairo compatibility
    let rgba = resized.to_rgba8();

    // Create Cairo image surface from pixel data
    // For ARGB32 format, stride is 4 bytes per pixel, rounded to 4-byte alignment
    let stride = (width as i32 * 4 + 3) & !3; // Align to 4 bytes
    let mut data = vec![0u8; (stride * height as i32) as usize];

    // Convert RGBA to BGRA (Cairo uses BGRA format)
    for y in 0..height {
        for x in 0..width {
            let pixel = rgba.get_pixel(x, y);
            let dst_idx = ((y as i32) * stride + (x as i32) * 4) as usize;

            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = pixel[3];

            // Cairo uses BGRA in little-endian
            data[dst_idx] = b;
            data[dst_idx + 1] = g;
            data[dst_idx + 2] = r;
            data[dst_idx + 3] = a;
        }
    }

    // Create Cairo surface from the converted data
    let surface = cairo::ImageSurface::create_for_data(
        data,
        cairo::Format::ARgb32,
        width as i32,
        height as i32,
        stride,
    )?;

    // Render the image surface to the context
    ctx.save()?;
    ctx.translate(bounds.origin.x as f64, bounds.origin.y as f64);
    ctx.scale(
        bounds.size.width as f64 / width as f64,
        bounds.size.height as f64 / height as f64,
    );

    ctx.set_source_surface(&surface, 0.0, 0.0)?;
    ctx.paint()?;
    ctx.restore()?;

    tracing::info!(
        "✅ Image rendered: {} ({}x{})",
        image_path.display(),
        width,
        height
    );

    Ok(())
}

/// Check if an image file exists and is readable
pub fn is_image_available(image_path: &Path) -> bool {
    if !image_path.exists() {
        tracing::warn!("⚠️  Image file not found: {}", image_path.display());
        return false;
    }

    if !image_path.is_file() {
        tracing::warn!("⚠️  Image path is not a file: {}", image_path.display());
        return false;
    }

    true
}

/// Draw a placeholder when image is not available
pub fn draw_image_placeholder(
    ctx: &Context,
    bounds: &Rect,
) -> Result<(), cairo::Error> {
    ctx.save()?;

    // Draw gray background
    ctx.set_source_rgb(0.9, 0.9, 0.9);
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64,
    );
    ctx.fill()?;

    // Draw border
    ctx.set_source_rgb(0.7, 0.7, 0.7);
    ctx.set_line_width(1.0);
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64,
    );
    ctx.stroke()?;

    // Draw X symbol to indicate missing image
    ctx.set_source_rgb(0.5, 0.5, 0.5);
    ctx.set_line_width(2.0);

    let x1 = bounds.origin.x as f64;
    let y1 = bounds.origin.y as f64;
    let x2 = bounds.origin.x as f64 + bounds.size.width as f64;
    let y2 = bounds.origin.y as f64 + bounds.size.height as f64;

    // Draw X
    ctx.move_to(x1 + 5.0, y1 + 5.0);
    ctx.line_to(x2 - 5.0, y2 - 5.0);
    ctx.stroke()?;

    ctx.move_to(x2 - 5.0, y1 + 5.0);
    ctx.line_to(x1 + 5.0, y2 - 5.0);
    ctx.stroke()?;

    ctx.restore()?;
    Ok(())
}
