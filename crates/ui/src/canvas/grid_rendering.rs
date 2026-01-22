//! Grid, ruler, and guide rendering
//!
//! This module provides drawing functions for grid lines, rulers, and guide lines.

use gtk4::cairo::Context;
use testruct_core::layout::Size;
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

/// Guide line orientation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideOrientation {
    Horizontal,
    Vertical,
}

/// A single guide line
#[derive(Clone, Debug)]
pub struct Guide {
    pub orientation: GuideOrientation,
    pub position: f32,
    pub color: Color,
}

impl Guide {
    pub fn new(orientation: GuideOrientation, position: f32) -> Self {
        Self {
            orientation,
            position,
            color: Color {
                r: 0.2,
                g: 0.6,
                b: 1.0,
                a: 0.6,
            },
        }
    }
}

/// Draw rulers on the canvas edges
///
/// Draws horizontal and vertical ruler bars with tick marks and measurements.
pub fn draw_rulers(
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

/// Grid display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridStyle {
    /// Solid lines
    Lines,
    /// Dotted pattern (points at intersections)
    #[default]
    Dots,
}

/// Grid configuration
#[derive(Clone, Debug)]
pub struct GridConfig {
    /// Grid spacing in pixels
    pub spacing: f32,
    /// Grid display style
    pub style: GridStyle,
    /// Grid color (RGBA)
    pub color: Color,
    /// Dot size for dotted grid
    pub dot_size: f32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            spacing: 20.0,
            style: GridStyle::Dots,
            color: Color {
                r: 0.7,
                g: 0.7,
                b: 0.7,
                a: 0.6,
            },
            dot_size: 1.5,
        }
    }
}

impl GridConfig {
    /// Create grid config with specific spacing
    pub fn with_spacing(spacing: f32) -> Self {
        Self {
            spacing,
            ..Default::default()
        }
    }
}

/// Draw the grid pattern
///
/// Renders a regular grid of lines or dots across the page.
pub fn draw_grid(ctx: &Context, page_size: &Size) -> Result<(), cairo::Error> {
    draw_grid_with_config(ctx, page_size, &GridConfig::default())
}

/// Draw the grid pattern with custom configuration
pub fn draw_grid_with_config(
    ctx: &Context,
    page_size: &Size,
    config: &GridConfig,
) -> Result<(), cairo::Error> {
    ctx.set_source_rgba(
        config.color.r as f64,
        config.color.g as f64,
        config.color.b as f64,
        config.color.a as f64,
    );

    let grid_spacing = config.spacing as f64;

    match config.style {
        GridStyle::Lines => {
            ctx.set_line_width(0.5);

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
        }
        GridStyle::Dots => {
            // Draw dots at grid intersections for better performance
            let dot_size = config.dot_size as f64;

            let mut y = grid_spacing;
            while y < page_size.height as f64 {
                let mut x = grid_spacing;
                while x < page_size.width as f64 {
                    ctx.arc(x, y, dot_size, 0.0, std::f64::consts::PI * 2.0);
                    ctx.fill()?;
                    x += grid_spacing;
                }
                y += grid_spacing;
            }
        }
    }

    Ok(())
}

/// Draw guide lines
///
/// Renders user-defined guide lines (vertical or horizontal).
pub fn draw_guides(ctx: &Context, guides: &[Guide], page_size: &Size) -> Result<(), cairo::Error> {
    for guide in guides {
        ctx.set_source_rgba(
            guide.color.r as f64,
            guide.color.g as f64,
            guide.color.b as f64,
            guide.color.a as f64,
        );
        ctx.set_line_width(1.0);

        match guide.orientation {
            GuideOrientation::Vertical => {
                // Vertical guide line
                let x = guide.position as f64;
                ctx.move_to(x, 0.0);
                ctx.line_to(x, page_size.height as f64);
            }
            GuideOrientation::Horizontal => {
                // Horizontal guide line
                let y = guide.position as f64;
                ctx.move_to(0.0, y);
                ctx.line_to(page_size.width as f64, y);
            }
        }
        ctx.stroke()?;
    }
    Ok(())
}
