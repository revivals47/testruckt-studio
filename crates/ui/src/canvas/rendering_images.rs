//! Image rendering functions for canvas
//!
//! This module handles rendering of image elements, including placeholders
//! and actual image loading and display.

use gtk4::cairo::{self, Context};
use gtk4::pango;
use testruct_core::layout::Rect;

/// Draw a placeholder for image elements
pub fn draw_image_placeholder(ctx: &Context, bounds: &Rect) -> Result<(), cairo::Error> {
    ctx.save()?;

    // Draw background with subtle gradient effect
    ctx.set_source_rgb(0.96, 0.96, 0.98);
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64,
    );
    ctx.fill()?;

    // Draw border with rounded corners effect
    ctx.set_source_rgb(0.65, 0.71, 0.82);
    ctx.set_line_width(1.5);
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64,
    );
    ctx.stroke()?;

    // Draw image icon (simple mountain/photo symbol)
    let cx = bounds.origin.x as f64 + bounds.size.width as f64 / 2.0;
    let cy = bounds.origin.y as f64 + bounds.size.height as f64 / 2.0;
    let icon_size = 30.0;

    ctx.set_source_rgb(0.55, 0.63, 0.75);
    ctx.set_line_width(2.0);

    // Draw mountain peaks
    ctx.move_to(cx - icon_size, cy + icon_size * 0.3);
    ctx.line_to(cx - icon_size * 0.3, cy - icon_size * 0.3);
    ctx.line_to(cx + icon_size * 0.3, cy + icon_size * 0.2);
    ctx.line_to(cx + icon_size, cy - icon_size * 0.3);
    ctx.stroke()?;

    // Draw circle (representing sun)
    ctx.arc(
        cx - icon_size * 0.5,
        cy - icon_size * 0.5,
        icon_size * 0.2,
        0.0,
        std::f64::consts::PI * 2.0,
    );
    ctx.stroke()?;

    // Draw "Image" text with better styling
    ctx.set_source_rgb(0.4, 0.4, 0.5);
    ctx.move_to(cx - icon_size * 0.4, cy + icon_size * 0.8);
    let layout = pangocairo::functions::create_layout(ctx);
    layout.set_text("Image Placeholder");
    let mut font_desc = pango::FontDescription::new();
    font_desc.set_family("Sans");
    font_desc.set_size((11 * pango::SCALE as i32) as i32);
    layout.set_font_description(Some(&font_desc));
    pangocairo::functions::show_layout(ctx, &layout);

    // Draw helpful hint text
    if bounds.size.height > 80.0 && bounds.size.width > 100.0 {
        ctx.set_source_rgb(0.6, 0.6, 0.6);
        ctx.move_to(
            bounds.origin.x as f64 + 5.0,
            bounds.origin.y as f64 + bounds.size.height as f64 - 15.0,
        );
        let hint_layout = pangocairo::functions::create_layout(ctx);
        hint_layout.set_text("(Actual image will display here)");
        let mut hint_font = pango::FontDescription::new();
        hint_font.set_family("Sans");
        hint_font.set_size((9 * pango::SCALE as i32) as i32);
        hint_layout.set_font_description(Some(&hint_font));
        pangocairo::functions::show_layout(ctx, &hint_layout);
    }

    ctx.restore()?;
    Ok(())
}

/// Draw image element with actual image file or fallback to placeholder
pub fn draw_image_element(
    ctx: &Context,
    bounds: &Rect,
    asset_ref: &testruct_core::workspace::assets::AssetRef,
    app_state: &crate::app::AppState,
) -> Result<(), Box<dyn std::error::Error>> {
    // Try to get the asset catalog and load the image
    let catalog = app_state.asset_catalog();
    let cat = catalog.lock().expect("asset catalog");

    if let Some(metadata) = cat.get(*asset_ref) {
        // Try to load and render the actual image
        if let Ok(_) = load_and_render_image(ctx, bounds, &metadata.path) {
            return Ok(());
        }
    }

    // Fallback to placeholder if image loading fails
    draw_image_placeholder(ctx, bounds).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

/// Load and render an image file to Cairo context
fn load_and_render_image(
    ctx: &Context,
    bounds: &Rect,
    path: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Load image file
    let image = image::open(path)?;

    // Convert to RGBA8 format
    let rgba_image = image.to_rgba8();
    let (img_width, img_height) = rgba_image.dimensions();

    // Create Cairo surface from image data (use RGB24 format for simplicity)
    let mut surface_data = Vec::with_capacity((img_width * img_height * 4) as usize);

    // Convert RGBA to RGB (dropping alpha for Cairo RGB24 format)
    let rgba_vec = rgba_image.into_raw();
    for chunk in rgba_vec.chunks_exact(4) {
        surface_data.push(chunk[2]); // B
        surface_data.push(chunk[1]); // G
        surface_data.push(chunk[0]); // R
        surface_data.push(255); // A (fully opaque)
    }

    // Create Cairo image surface
    let stride = (img_width * 4) as i32;
    let surface = cairo::ImageSurface::create_for_data(
        surface_data,
        cairo::Format::Rgb24,
        img_width as i32,
        img_height as i32,
        stride,
    )
    .map_err(|e| format!("Failed to create Cairo surface: {:?}", e))?;

    // Draw the image to fit bounds
    ctx.save()
        .map_err(|e| format!("Failed to save context: {:?}", e))?;

    // Fill background
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64,
    );
    ctx.fill()
        .map_err(|e| format!("Failed to fill background: {:?}", e))?;

    // Calculate scale to fit image in bounds while maintaining aspect ratio
    let scale_x = bounds.size.width as f64 / img_width as f64;
    let scale_y = bounds.size.height as f64 / img_height as f64;
    let scale = scale_x.min(scale_y);

    // Calculate position to center image
    let scaled_width = img_width as f64 * scale;
    let scaled_height = img_height as f64 * scale;
    let offset_x = bounds.origin.x as f64 + (bounds.size.width as f64 - scaled_width) / 2.0;
    let offset_y = bounds.origin.y as f64 + (bounds.size.height as f64 - scaled_height) / 2.0;

    // Draw image
    ctx.translate(offset_x, offset_y);
    ctx.scale(scale, scale);
    ctx.set_source_surface(&surface, 0.0, 0.0)
        .map_err(|e| format!("Failed to set image source: {:?}", e))?;
    ctx.paint()
        .map_err(|e| format!("Failed to paint image: {:?}", e))?;

    ctx.restore()
        .map_err(|e| format!("Failed to restore context: {:?}", e))?;
    Ok(())
}
