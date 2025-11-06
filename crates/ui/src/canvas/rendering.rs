//! Canvas rendering pipeline - Cairo drawing system
//!
//! This module provides the complete drawing implementation for the canvas,
//! including document objects (shapes, text, images) and UI overlays.
//!
//! Grid, ruler, and guide rendering has been moved to the `grid_rendering` module.

use gtk4::cairo::{self, Context};
use gtk4::pango;
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::Color;

// Re-export types from grid_rendering for backward compatibility
pub use super::grid_rendering::{Guide, GuideOrientation, RulerConfig};

/// Canvas rendering state
#[derive(Clone, Debug)]
pub struct RenderConfig {
    pub zoom: f64,
    pub pan_x: f64,
    pub pan_y: f64,
    pub show_grid: bool,
    pub show_rulers: bool,
    pub show_guides: bool,
    pub snap_to_grid: bool,
    pub grid_spacing: f32,
    pub guides: Vec<Guide>,
    pub snap_to_guides: bool,
    pub guide_snap_distance: f32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            show_grid: true,
            show_rulers: true,
            show_guides: true,
            snap_to_grid: true,
            grid_spacing: 10.0,
            guides: Vec::new(),
            snap_to_guides: true,
            guide_snap_distance: 5.0,
        }
    }
}

/// Resize handle positions
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResizeHandle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl ResizeHandle {
    /// Get the screen position of a resize handle
    pub fn position(self, bounds: &Rect) -> Point {
        match self {
            ResizeHandle::TopLeft => Point::new(bounds.origin.x, bounds.origin.y),
            ResizeHandle::Top => Point::new(bounds.origin.x + bounds.size.width / 2.0, bounds.origin.y),
            ResizeHandle::TopRight => Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y),
            ResizeHandle::Right => Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height / 2.0),
            ResizeHandle::BottomRight => Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y + bounds.size.height),
            ResizeHandle::Bottom => Point::new(bounds.origin.x + bounds.size.width / 2.0, bounds.origin.y + bounds.size.height),
            ResizeHandle::BottomLeft => Point::new(bounds.origin.x, bounds.origin.y + bounds.size.height),
            ResizeHandle::Left => Point::new(bounds.origin.x, bounds.origin.y + bounds.size.height / 2.0),
        }
    }

    pub const SIZE: f64 = 8.0;
    pub const RESIZE_HANDLE_COUNT: usize = 8;

    pub fn all() -> [Self; 8] {
        [
            Self::TopLeft,
            Self::Top,
            Self::TopRight,
            Self::Right,
            Self::BottomRight,
            Self::Bottom,
            Self::BottomLeft,
            Self::Left,
        ]
    }
}

/// Draw the canvas background and rulers
pub fn draw_background(
    ctx: &Context,
    width: f64,
    height: f64,
    config: &RulerConfig,
) -> Result<(), cairo::Error> {
    // White background
    ctx.set_source_rgb(1.0, 1.0, 1.0);
    ctx.paint()?;

    // Draw rulers if enabled
    if config.size > 0.0 {
        super::grid_rendering::draw_rulers(ctx, width, height, config)?;
    }

    Ok(())
}

// Re-export draw_grid for backward compatibility
pub use super::grid_rendering::draw_grid;

/// Draw page border
pub fn draw_page_border(
    ctx: &Context,
    page_size: &Size,
) -> Result<(), cairo::Error> {
    ctx.set_source_rgb(0.8, 0.8, 0.8);
    ctx.set_line_width(1.0);
    ctx.rectangle(0.0, 0.0, page_size.width as f64, page_size.height as f64);
    ctx.stroke()?;
    Ok(())
}

/// Draw a selection rectangle
pub fn draw_selection_box(
    ctx: &Context,
    bounds: &Rect,
    stroke_color: &Color,
) -> Result<(), cairo::Error> {
    ctx.set_source_rgb(stroke_color.r as f64, stroke_color.g as f64, stroke_color.b as f64);
    ctx.set_line_width(2.0);
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64
    );
    ctx.stroke()?;

    // Semi-transparent fill
    ctx.set_source_rgba(stroke_color.r as f64, stroke_color.g as f64, stroke_color.b as f64, 0.1);
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64
    );
    ctx.fill()?;

    Ok(())
}

/// Draw resize handles for a selected object
pub fn draw_resize_handles(
    ctx: &Context,
    bounds: &Rect,
    handle_color: &Color,
) -> Result<(), cairo::Error> {
    for handle in ResizeHandle::all().iter() {
        let pos = handle.position(bounds);
        let half_size = ResizeHandle::SIZE / 2.0;

        ctx.set_source_rgb(handle_color.r as f64, handle_color.g as f64, handle_color.b as f64);
        ctx.rectangle(
            pos.x as f64 - half_size,
            pos.y as f64 - half_size,
            ResizeHandle::SIZE,
            ResizeHandle::SIZE,
        );
        ctx.fill()?;

        // White border
        ctx.set_source_rgb(1.0, 1.0, 1.0);
        ctx.set_line_width(1.0);
        ctx.rectangle(
            pos.x as f64 - half_size,
            pos.y as f64 - half_size,
            ResizeHandle::SIZE,
            ResizeHandle::SIZE,
        );
        ctx.stroke()?;
    }

    Ok(())
}

/// Text element rendering constants
const TEXT_PADDING: f64 = 5.0;

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
            bounds.size.height as f64
        );
        ctx.fill()?;
    }

    // Clipping rectangle
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64
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
    ctx.set_source_rgb(style.color.r as f64, style.color.g as f64, style.color.b as f64);
    ctx.translate(bounds.origin.x as f64 + TEXT_PADDING, bounds.origin.y as f64 + TEXT_PADDING);

    // Render layout
    pangocairo::functions::show_layout(ctx, &layout);

    ctx.restore()?;
    Ok(())
}

/// Draw a text cursor for editing mode
pub fn draw_text_cursor(
    ctx: &Context,
    bounds: &Rect,
    _text: &str,
    cursor_pos: usize,
    style: &testruct_core::typography::TextStyle,
) -> Result<(), cairo::Error> {
    ctx.save()?;

    let x_offset = bounds.origin.x as f64 + TEXT_PADDING;
    let y_offset = bounds.origin.y as f64 + TEXT_PADDING;

    // Estimate cursor x position based on font size and character count
    // Average character width is approximately 60% of font size for monospace
    // and ~40-50% for proportional fonts. We use an approximation here.
    let char_width = (style.font_size as f64) * 0.5;
    let cursor_x = x_offset + (cursor_pos as f64 * char_width);

    // Draw text cursor as a thin vertical line
    ctx.set_source_rgb(0.0, 0.5, 1.0); // Blue cursor
    ctx.set_line_width(2.0);
    ctx.move_to(cursor_x, y_offset);
    ctx.line_to(cursor_x, y_offset + (bounds.size.height as f64 - TEXT_PADDING));
    ctx.stroke()?;

    ctx.restore()?;
    Ok(())
}

/// Draw a frame to indicate text editing mode
pub fn draw_text_editing_frame(
    ctx: &Context,
    bounds: &Rect,
) -> Result<(), cairo::Error> {
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

/// Draw a placeholder for image elements
pub fn draw_image_placeholder(
    ctx: &Context,
    bounds: &Rect,
) -> Result<(), cairo::Error> {
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
    ctx.arc(cx - icon_size * 0.5, cy - icon_size * 0.5, icon_size * 0.2, 0.0, std::f64::consts::PI * 2.0);
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

// Re-export shape drawing functions for backward compatibility
pub use super::shapes_rendering::{
    draw_rectangle, draw_ellipse, draw_line, draw_arrow, draw_polygon
};

// Re-export draw_guides for backward compatibility
pub use super::grid_rendering::draw_guides;

/// Find the closest guide to a position
pub fn snap_to_guide(
    value: f32,
    guides: &[Guide],
    orientation: GuideOrientation,
    snap_distance: f32,
) -> Option<f32> {
    for guide in guides {
        if guide.orientation == orientation {
            let distance = (value - guide.position).abs();
            if distance <= snap_distance {
                return Some(guide.position);
            }
        }
    }
    None
}

/// Grid snapping configuration
pub struct GridSnapConfig {
    pub enabled: bool,
    pub spacing: f32,
}

impl Default for GridSnapConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            spacing: 10.0,
        }
    }
}

/// Snap a coordinate value to the grid
pub fn snap_to_grid(value: f32, spacing: f32) -> f32 {
    if spacing > 0.0 {
        (value / spacing).round() * spacing
    } else {
        value
    }
}

/// Snap a point to the grid
pub fn snap_point_to_grid(point: &Point, spacing: f32) -> Point {
    Point::new(
        snap_to_grid(point.x, spacing),
        snap_to_grid(point.y, spacing),
    )
}

/// Snap a rectangle's origin to the grid
pub fn snap_rect_to_grid(rect: &Rect, spacing: f32) -> Rect {
    Rect::new(
        snap_point_to_grid(&rect.origin, spacing),
        rect.size.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_handle_positions() {
        let bounds = Rect::new(
            Point::new(10.0, 20.0),
            testruct_core::layout::Size::new(100.0, 80.0)
        );

        let tl = ResizeHandle::TopLeft.position(&bounds);
        assert_eq!(tl.x, 10.0);
        assert_eq!(tl.y, 20.0);

        let br = ResizeHandle::BottomRight.position(&bounds);
        assert_eq!(br.x, 110.0);
        assert_eq!(br.y, 100.0);

        let center = ResizeHandle::Top.position(&bounds);
        assert_eq!(center.x, 60.0);
        assert_eq!(center.y, 20.0);
    }

    #[test]
    fn test_ruler_config_default() {
        let config = RulerConfig::default();
        assert_eq!(config.size, 20.0);
        assert!(config.bg_color.r > 0.9);
    }

    #[test]
    fn test_render_config_default() {
        let config = RenderConfig::default();
        assert_eq!(config.zoom, 1.0);
        assert_eq!(config.pan_x, 0.0);
        assert_eq!(config.pan_y, 0.0);
        assert!(config.show_grid);
        assert!(config.show_rulers);
    }
}
