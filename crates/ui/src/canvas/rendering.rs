//! Canvas rendering pipeline - Cairo drawing system
//!
//! This module provides the complete drawing implementation for the canvas,
//! including rulers, grid, guides, and document objects.

use gtk4::cairo::{self, Context};
use gtk4::pango;
use testruct_core::layout::{Point, Rect, Size};
use testruct_core::typography::Color;

/// Configuration for ruler rendering
#[derive(Clone, Debug)]
pub struct RulerConfig {
    pub size: f64,
    pub bg_color: Color,
    pub tick_color: Color,
    pub text_color: Color,
}

impl Default for RulerConfig {
    fn default() -> Self {
        Self {
            size: 20.0,
            bg_color: Color {
                r: 0.95,
                g: 0.95,
                b: 0.95,
                a: 1.0,
            },
            tick_color: Color {
                r: 0.4,
                g: 0.4,
                b: 0.4,
                a: 1.0,
            },
            text_color: Color {
                r: 0.3,
                g: 0.3,
                b: 0.3,
                a: 1.0,
            },
        }
    }
}

/// Canvas rendering state
#[derive(Clone, Debug)]
pub struct RenderConfig {
    pub zoom: f64,
    pub pan_x: f64,
    pub pan_y: f64,
    pub show_grid: bool,
    pub show_rulers: bool,
    pub show_guides: bool,
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
        draw_rulers(ctx, width, height, config)?;
    }

    Ok(())
}

/// Draw horizontal and vertical rulers
fn draw_rulers(
    ctx: &Context,
    canvas_width: f64,
    canvas_height: f64,
    config: &RulerConfig,
) -> Result<(), cairo::Error> {
    let size = config.size;

    // Horizontal ruler background
    ctx.set_source_rgb(
        config.bg_color.r as f64,
        config.bg_color.g as f64,
        config.bg_color.b as f64,
    );
    ctx.rectangle(0.0, 0.0, canvas_width, size);
    ctx.fill()?;

    // Vertical ruler background
    ctx.rectangle(0.0, 0.0, size, canvas_height);
    ctx.fill()?;

    // Ruler ticks and text
    ctx.set_source_rgb(
        config.tick_color.r as f64,
        config.tick_color.g as f64,
        config.tick_color.b as f64,
    );
    ctx.set_line_width(1.0);
    ctx.set_font_size(9.0);

    // Horizontal ruler markings
    let mut x = 0.0;
    while x <= canvas_width - size {
        let screen_x = x + size;
        let tick_height = if (x as i32) % 100 == 0 {
            10.0
        } else if (x as i32) % 50 == 0 {
            7.0
        } else if (x as i32) % 10 == 0 {
            5.0
        } else {
            0.0
        };

        if tick_height > 0.0 {
            ctx.move_to(screen_x, size - tick_height);
            ctx.line_to(screen_x, size);
            ctx.stroke()?;

            // Draw measurement text
            if (x as i32) % 100 == 0 && x > 0.0 {
                ctx.set_source_rgb(
                    config.text_color.r as f64,
                    config.text_color.g as f64,
                    config.text_color.b as f64,
                );
                let text = format!("{}", x as i32);
                if let Ok(extents) = ctx.text_extents(&text) {
                    ctx.move_to(screen_x - extents.width() / 2.0, 12.0);
                    ctx.show_text(&text)?;
                }
                ctx.set_source_rgb(
                    config.tick_color.r as f64,
                    config.tick_color.g as f64,
                    config.tick_color.b as f64,
                );
            }
        }
        x += 10.0;
    }

    // Vertical ruler markings
    let mut y = 0.0;
    while y <= canvas_height - size {
        let screen_y = y + size;
        let tick_width = if (y as i32) % 100 == 0 {
            10.0
        } else if (y as i32) % 50 == 0 {
            7.0
        } else if (y as i32) % 10 == 0 {
            5.0
        } else {
            0.0
        };

        if tick_width > 0.0 {
            ctx.move_to(size - tick_width, screen_y);
            ctx.line_to(size, screen_y);
            ctx.stroke()?;

            // Draw measurement text
            if (y as i32) % 100 == 0 && y > 0.0 {
                ctx.set_source_rgb(
                    config.text_color.r as f64,
                    config.text_color.g as f64,
                    config.text_color.b as f64,
                );
                let text = format!("{}", y as i32);
                ctx.save()?;
                ctx.move_to(6.0, screen_y + 3.0);
                ctx.show_text(&text)?;
                ctx.restore()?;
                ctx.set_source_rgb(
                    config.tick_color.r as f64,
                    config.tick_color.g as f64,
                    config.tick_color.b as f64,
                );
            }
        }
        y += 10.0;
    }

    // Translate context past rulers for content drawing
    ctx.translate(size, size);

    Ok(())
}

/// Draw the grid pattern
pub fn draw_grid(
    ctx: &Context,
    page_size: &Size,
) -> Result<(), cairo::Error> {
    ctx.set_source_rgba(0.9, 0.9, 0.9, 0.5);
    ctx.set_line_width(0.5);

    let grid_spacing = 10.0;

    // Vertical grid lines
    let mut x = grid_spacing;
    while x < page_size.width as f64 {
        ctx.move_to(x, 0.0);
        ctx.line_to(x, page_size.height as f64);
        x += grid_spacing;
    }

    // Horizontal grid lines
    let mut y = grid_spacing;
    while y < page_size.height as f64 {
        ctx.move_to(0.0, y);
        ctx.line_to(page_size.width as f64, y);
        y += grid_spacing;
    }

    ctx.stroke()?;
    Ok(())
}

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

/// Draw a text element
pub fn draw_text_element(
    ctx: &Context,
    bounds: &Rect,
    text: &str,
    style: &testruct_core::typography::TextStyle,
) -> Result<(), cairo::Error> {
    ctx.save()?;

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

    // Set font
    let mut font_desc = pango::FontDescription::new();
    font_desc.set_family(&style.font_family);
    font_desc.set_size((style.font_size * pango::SCALE as f32) as i32);
    layout.set_font_description(Some(&font_desc));

    // Set text color
    ctx.set_source_rgb(0.2, 0.2, 0.2);
    ctx.move_to(bounds.origin.x as f64 + 5.0, bounds.origin.y as f64 + 5.0);

    // Render layout
    pangocairo::functions::show_layout(ctx, &layout);

    ctx.restore()?;
    Ok(())
}

/// Draw a rectangle shape
pub fn draw_rectangle(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    fill: &Option<Color>,
) -> Result<(), cairo::Error> {
    if let Some(fill_color) = fill {
        ctx.set_source_rgb(fill_color.r as f64, fill_color.g as f64, fill_color.b as f64);
        ctx.rectangle(
            bounds.origin.x as f64,
            bounds.origin.y as f64,
            bounds.size.width as f64,
            bounds.size.height as f64
        );
        ctx.fill()?;
    }

    if let Some(stroke_color) = stroke {
        ctx.set_source_rgb(stroke_color.r as f64, stroke_color.g as f64, stroke_color.b as f64);
        ctx.set_line_width(2.0);
        ctx.rectangle(
            bounds.origin.x as f64,
            bounds.origin.y as f64,
            bounds.size.width as f64,
            bounds.size.height as f64
        );
        ctx.stroke()?;
    }

    Ok(())
}

/// Draw a circle/ellipse shape
pub fn draw_ellipse(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    fill: &Option<Color>,
) -> Result<(), cairo::Error> {
    let cx = bounds.origin.x as f64 + bounds.size.width as f64 / 2.0;
    let cy = bounds.origin.y as f64 + bounds.size.height as f64 / 2.0;
    let rx = bounds.size.width as f64 / 2.0;
    let ry = bounds.size.height as f64 / 2.0;

    // Draw ellipse path using arc approximation
    ctx.save()?;
    ctx.translate(cx, cy);
    ctx.scale(rx, ry);
    ctx.arc(0.0, 0.0, 1.0, 0.0, 2.0 * std::f64::consts::PI);
    ctx.restore()?;

    if let Some(fill_color) = fill {
        ctx.set_source_rgb(fill_color.r as f64, fill_color.g as f64, fill_color.b as f64);
        ctx.fill_preserve()?;
    }

    if let Some(stroke_color) = stroke {
        ctx.set_source_rgb(stroke_color.r as f64, stroke_color.g as f64, stroke_color.b as f64);
        ctx.set_line_width(2.0);
        ctx.stroke()?;
    }

    Ok(())
}

/// Draw a line shape
pub fn draw_line(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
) -> Result<(), cairo::Error> {
    if let Some(stroke_color) = stroke {
        ctx.set_source_rgb(stroke_color.r as f64, stroke_color.g as f64, stroke_color.b as f64);
        ctx.set_line_width(2.0);
        ctx.move_to(bounds.origin.x as f64, bounds.origin.y as f64);
        ctx.line_to(
            bounds.origin.x as f64 + bounds.size.width as f64,
            bounds.origin.y as f64 + bounds.size.height as f64
        );
        ctx.stroke()?;
    }

    Ok(())
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
