//! Text rendering functions for canvas
//!
//! This module handles all text-related rendering operations including
//! text elements, cursor positioning, editing frames, and text measurement.

use gtk4::cairo::{self, Context};
use gtk4::pango;
use testruct_core::layout::Rect;

/// Text element rendering constants
pub const TEXT_PADDING: f64 = 5.0;

/// Draw a text element with line wrapping support
pub fn draw_text_element(
    ctx: &Context,
    bounds: &Rect,
    text: &str,
    style: &testruct_core::typography::TextStyle,
) -> Result<(), cairo::Error> {
    ctx.save()?;

    // Draw background color if specified
    if let Some(bg_color) = style.background_color {
        ctx.set_source_rgb(bg_color.r as f64, bg_color.g as f64, bg_color.b as f64);
        ctx.rectangle(
            bounds.origin.x as f64,
            bounds.origin.y as f64,
            bounds.size.width as f64,
            bounds.size.height as f64,
        );
        ctx.fill()?;
    }

    // Clipping rectangle
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64,
    );
    ctx.clip();

    // Create Pango layout using pangocairo
    let layout = pangocairo::functions::create_layout(ctx);
    layout.set_text(text);

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
        testruct_core::typography::TextAlignment::Justified => pango::Alignment::Center, // Fallback for justified
    };
    layout.set_alignment(pango_alignment);

    // Enable text wrapping by setting width constraint
    // This makes canvas rendering consistent with PDF/SVG export
    let available_width = (bounds.size.width as f64 - (TEXT_PADDING * 2.0)).max(0.0);
    layout.set_width((available_width * pango::SCALE as f64) as i32);

    // Apply text decorations
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

    // Set text color and position
    ctx.set_source_rgb(
        style.color.r as f64,
        style.color.g as f64,
        style.color.b as f64,
    );
    ctx.translate(
        bounds.origin.x as f64 + TEXT_PADDING,
        bounds.origin.y as f64 + TEXT_PADDING,
    );

    // Render layout
    pangocairo::functions::show_layout(ctx, &layout);

    ctx.restore()?;
    Ok(())
}

/// Measure the height of a text block for a given width and style.
///
/// This mirrors the rendering configuration (padding, font settings, alignment)
/// so that layout calculations are consistent between draw and measurement.
pub fn measure_text_height(
    text: &str,
    style: &testruct_core::typography::TextStyle,
    width: f32,
) -> f32 {
    // Use an off-screen surface to create a Pango layout consistent with canvas rendering.
    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 1, 1)
        .expect("Failed to create surface for text measurement");
    let ctx = Context::new(&surface).expect("Failed to create Cairo context for text measurement");

    let layout = pangocairo::functions::create_layout(&ctx);
    layout.set_text(text);

    // Configure font description to match canvas rendering
    let mut font_desc = pango::FontDescription::new();
    font_desc.set_family(&style.font_family);
    font_desc.set_size((style.font_size * pango::SCALE as f32) as i32);

    let weight = match style.weight {
        testruct_core::typography::FontWeight::Thin => pango::Weight::Thin,
        testruct_core::typography::FontWeight::Light => pango::Weight::Light,
        testruct_core::typography::FontWeight::Regular => pango::Weight::Normal,
        testruct_core::typography::FontWeight::Medium => pango::Weight::Medium,
        testruct_core::typography::FontWeight::Bold => pango::Weight::Bold,
        testruct_core::typography::FontWeight::Black => pango::Weight::Ultrabold,
    };
    font_desc.set_weight(weight);

    if style.italic {
        font_desc.set_style(pango::Style::Italic);
    }

    layout.set_font_description(Some(&font_desc));

    let alignment = match style.alignment {
        testruct_core::typography::TextAlignment::Start => pango::Alignment::Left,
        testruct_core::typography::TextAlignment::Center => pango::Alignment::Center,
        testruct_core::typography::TextAlignment::End => pango::Alignment::Right,
        testruct_core::typography::TextAlignment::Justified => pango::Alignment::Center,
    };
    layout.set_alignment(alignment);

    let attrs = pango::AttrList::new();
    if style.underline {
        let underline_attr = pango::AttrInt::new_underline(pango::Underline::Single);
        attrs.insert(underline_attr);
    }
    if style.strikethrough {
        let strike_attr = pango::AttrInt::new_strikethrough(true);
        attrs.insert(strike_attr);
    }
    layout.set_attributes(Some(&attrs));

    // Account for the same padding used during draw
    let available_width = (width as f64 - (TEXT_PADDING * 2.0)).max(0.0);
    layout.set_width((available_width * pango::SCALE as f64) as i32);
    layout.set_wrap(pango::WrapMode::WordChar);

    // Measure logical height in device pixels
    let (_, logical_rect) = layout.pixel_extents();
    let layout_height = logical_rect.height().max(0) as f64;

    let padded_height = layout_height + (TEXT_PADDING * 2.0);
    let min_height = (style.font_size as f64) + (TEXT_PADDING * 2.0);

    padded_height.max(min_height) as f32
}

/// Draw a text cursor for editing mode
pub fn draw_text_cursor(
    ctx: &Context,
    bounds: &Rect,
    text: &str,
    cursor_pos: usize,
    style: &testruct_core::typography::TextStyle,
) -> Result<(), cairo::Error> {
    ctx.save()?;

    let x_offset = bounds.origin.x as f64 + TEXT_PADDING;
    let y_offset = bounds.origin.y as f64 + TEXT_PADDING;

    // Create Pango layout for the FULL text to handle line wrapping correctly
    let layout = pangocairo::functions::create_layout(ctx);
    layout.set_text(text);

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

    // Set width constraint to match text rendering
    let available_width = (bounds.size.width as f64 - (TEXT_PADDING * 2.0)).max(0.0);
    layout.set_width((available_width * pango::SCALE as f64) as i32);

    // Get line height from the layout
    let (_, logical_rect) = layout.extents();
    let line_height = (logical_rect.height() as f64 / pango::SCALE as f64).max(style.font_size as f64 * 1.2);

    // Convert character position to byte position for Pango
    let byte_pos = text.chars().take(cursor_pos).map(|c| c.len_utf8()).sum::<usize>();

    // Get cursor position using Pango's indexing
    // Find which line the cursor is on by getting the cursor rectangle
    let mut line_x = 0.0;
    let mut line_y = 0.0;

    // Use a simpler approach: count newlines to find which line we're on
    let text_up_to_cursor = text.chars().take(cursor_pos).collect::<String>();
    let line_number = text_up_to_cursor.matches('\n').count();

    // Get width of the last line (after the last newline)
    let last_line = text_up_to_cursor.split('\n').last().unwrap_or("");
    let layout_line = pangocairo::functions::create_layout(ctx);
    layout_line.set_text(last_line);
    layout_line.set_font_description(Some(&font_desc));
    layout_line.set_width((available_width * pango::SCALE as f64) as i32);

    let (ink_rect, _logical_rect) = layout_line.pixel_extents();
    line_x = x_offset + ink_rect.width() as f64;
    line_y = y_offset + (line_number as f64 * line_height);

    // Draw text cursor as a thin vertical line
    ctx.set_source_rgb(0.0, 0.5, 1.0); // Blue cursor
    ctx.set_line_width(2.0);
    ctx.move_to(line_x, line_y);
    ctx.line_to(line_x, line_y + line_height);
    ctx.stroke()?;

    ctx.restore()?;
    Ok(())
}

/// Draw a frame to indicate text editing mode
pub fn draw_text_editing_frame(ctx: &Context, bounds: &Rect) -> Result<(), cairo::Error> {
    ctx.save()?;

    // Draw a dashed border to indicate editing mode
    ctx.set_source_rgb(0.2, 0.6, 1.0); // Light blue
    ctx.set_line_width(2.0);
    ctx.set_dash(&[5.0, 3.0], 0.0);
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64,
    );
    ctx.stroke()?;

    ctx.restore()?;
    Ok(())
}
