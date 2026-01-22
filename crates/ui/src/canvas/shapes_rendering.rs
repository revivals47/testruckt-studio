//! Shape drawing functions
//!
//! This module provides drawing implementations for shapes like rectangles,
//! ellipses, lines, arrows, and polygons.

use gtk4::cairo::Context;
use testruct_core::layout::Rect;
use testruct_core::typography::Color;

/// Draw a rectangle shape
pub fn draw_rectangle(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    stroke_width: f32,
    fill: &Option<Color>,
) -> Result<(), cairo::Error> {
    if let Some(fill_color) = fill {
        ctx.set_source_rgb(
            fill_color.r as f64,
            fill_color.g as f64,
            fill_color.b as f64,
        );
        ctx.rectangle(
            bounds.origin.x as f64,
            bounds.origin.y as f64,
            bounds.size.width as f64,
            bounds.size.height as f64,
        );
        ctx.fill()?;
    }

    if let Some(stroke_color) = stroke {
        ctx.set_source_rgb(
            stroke_color.r as f64,
            stroke_color.g as f64,
            stroke_color.b as f64,
        );
        ctx.set_line_width(stroke_width as f64);
        ctx.rectangle(
            bounds.origin.x as f64,
            bounds.origin.y as f64,
            bounds.size.width as f64,
            bounds.size.height as f64,
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
    stroke_width: f32,
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
        ctx.set_source_rgb(
            fill_color.r as f64,
            fill_color.g as f64,
            fill_color.b as f64,
        );
        ctx.fill_preserve()?;
    }

    if let Some(stroke_color) = stroke {
        ctx.set_source_rgb(
            stroke_color.r as f64,
            stroke_color.g as f64,
            stroke_color.b as f64,
        );
        ctx.set_line_width(stroke_width as f64);
        ctx.stroke()?;
    }

    Ok(())
}

/// Draw a line shape
pub fn draw_line(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    stroke_width: f32,
) -> Result<(), cairo::Error> {
    if let Some(stroke_color) = stroke {
        ctx.set_source_rgb(
            stroke_color.r as f64,
            stroke_color.g as f64,
            stroke_color.b as f64,
        );
        ctx.set_line_width(stroke_width as f64);
        ctx.move_to(bounds.origin.x as f64, bounds.origin.y as f64);
        ctx.line_to(
            bounds.origin.x as f64 + bounds.size.width as f64,
            bounds.origin.y as f64 + bounds.size.height as f64,
        );
        ctx.stroke()?;
    }

    Ok(())
}

/// Draw an arrow shape with properly calculated arrowhead
///
/// The arrow consists of:
/// - A line from start point (origin) to end point (origin + size)
/// - A filled triangular arrowhead at the end point
///
/// The arrowhead size scales with stroke_width for visual consistency
pub fn draw_arrow(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    stroke_width: f32,
) -> Result<(), cairo::Error> {
    let stroke_color = stroke.unwrap_or(Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    });

    // Start and end points of the arrow
    let x1 = bounds.origin.x as f64;
    let y1 = bounds.origin.y as f64;
    let x2 = bounds.origin.x as f64 + bounds.size.width as f64;
    let y2 = bounds.origin.y as f64 + bounds.size.height as f64;

    // Calculate arrow direction angle
    let angle = (y2 - y1).atan2(x2 - x1);

    // Arrowhead size proportional to stroke width (minimum 8, scales with stroke)
    let arrow_length = (stroke_width as f64 * 4.0).max(8.0).min(24.0);
    let arrow_width = arrow_length * 0.6; // Width is 60% of length

    // Calculate arrowhead vertices
    // The arrowhead is a triangle with tip at (x2, y2)
    let _half_angle = std::f64::consts::PI / 6.0; // 30 degrees from center line

    // Back point of the arrowhead (where line meets the head)
    let back_x = x2 - arrow_length * angle.cos();
    let back_y = y2 - arrow_length * angle.sin();

    // Two side points of the arrowhead triangle
    let left_x = back_x + arrow_width * (angle + std::f64::consts::FRAC_PI_2).cos();
    let left_y = back_y + arrow_width * (angle + std::f64::consts::FRAC_PI_2).sin();
    let right_x = back_x + arrow_width * (angle - std::f64::consts::FRAC_PI_2).cos();
    let right_y = back_y + arrow_width * (angle - std::f64::consts::FRAC_PI_2).sin();

    // Set color
    ctx.set_source_rgb(
        stroke_color.r as f64,
        stroke_color.g as f64,
        stroke_color.b as f64,
    );

    // Draw the line (from start to the back of the arrowhead)
    ctx.set_line_width(stroke_width as f64);
    ctx.move_to(x1, y1);
    ctx.line_to(back_x, back_y);
    ctx.stroke()?;

    // Draw the arrowhead as a filled triangle
    ctx.move_to(x2, y2);           // Tip
    ctx.line_to(left_x, left_y);   // Left wing
    ctx.line_to(right_x, right_y); // Right wing
    ctx.close_path();
    ctx.fill()?;

    Ok(())
}

/// Draw a regular polygon shape
///
/// Draws a regular polygon (equal sides and angles) inscribed in the bounding rectangle.
/// Default is a pentagon (5 sides). The polygon is oriented with the first vertex at the top.
///
/// # Arguments
/// * `ctx` - Cairo drawing context
/// * `bounds` - Bounding rectangle for the polygon
/// * `stroke` - Optional stroke color
/// * `stroke_width` - Width of the stroke line
pub fn draw_polygon(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    stroke_width: f32,
) -> Result<(), cairo::Error> {
    draw_regular_polygon(ctx, bounds, stroke, stroke_width, &None, 5)
}

/// Draw a regular polygon with configurable number of sides and optional fill
///
/// # Arguments
/// * `ctx` - Cairo drawing context
/// * `bounds` - Bounding rectangle for the polygon
/// * `stroke` - Optional stroke color
/// * `stroke_width` - Width of the stroke line
/// * `fill` - Optional fill color
/// * `sides` - Number of sides (minimum 3)
pub fn draw_regular_polygon(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    stroke_width: f32,
    fill: &Option<Color>,
    sides: usize,
) -> Result<(), cairo::Error> {
    // Ensure at least 3 sides
    let sides = sides.max(3);

    // Calculate center and radii for ellipse-fitting polygon
    let center_x = bounds.origin.x as f64 + bounds.size.width as f64 / 2.0;
    let center_y = bounds.origin.y as f64 + bounds.size.height as f64 / 2.0;
    let radius_x = bounds.size.width as f64 / 2.0;
    let radius_y = bounds.size.height as f64 / 2.0;

    // Build the polygon path
    // Start from the top (angle = -PI/2) for consistent orientation
    let start_angle = -std::f64::consts::FRAC_PI_2;

    for i in 0..sides {
        let angle = start_angle + (2.0 * std::f64::consts::PI * i as f64 / sides as f64);
        let x = center_x + radius_x * angle.cos();
        let y = center_y + radius_y * angle.sin();

        if i == 0 {
            ctx.move_to(x, y);
        } else {
            ctx.line_to(x, y);
        }
    }
    ctx.close_path();

    // Fill first (so stroke draws on top)
    if let Some(fill_color) = fill {
        ctx.set_source_rgba(
            fill_color.r as f64,
            fill_color.g as f64,
            fill_color.b as f64,
            fill_color.a as f64,
        );
        ctx.fill_preserve()?;
    }

    // Then stroke
    if let Some(stroke_color) = stroke {
        ctx.set_source_rgba(
            stroke_color.r as f64,
            stroke_color.g as f64,
            stroke_color.b as f64,
            stroke_color.a as f64,
        );
        ctx.set_line_width(stroke_width as f64);
        ctx.stroke()?;
    } else {
        // Clear the path if no stroke
        ctx.new_path();
    }

    Ok(())
}

/// Draw a triangle (3-sided polygon)
pub fn draw_triangle(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    stroke_width: f32,
    fill: &Option<Color>,
) -> Result<(), cairo::Error> {
    draw_regular_polygon(ctx, bounds, stroke, stroke_width, fill, 3)
}

/// Draw a hexagon (6-sided polygon)
pub fn draw_hexagon(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    stroke_width: f32,
    fill: &Option<Color>,
) -> Result<(), cairo::Error> {
    draw_regular_polygon(ctx, bounds, stroke, stroke_width, fill, 6)
}

/// Draw a star shape
///
/// # Arguments
/// * `ctx` - Cairo drawing context
/// * `bounds` - Bounding rectangle for the star
/// * `stroke` - Optional stroke color
/// * `stroke_width` - Width of the stroke line
/// * `fill` - Optional fill color
/// * `points` - Number of star points (minimum 3)
/// * `inner_ratio` - Ratio of inner radius to outer radius (0.0-1.0, default ~0.38)
pub fn draw_star(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
    stroke_width: f32,
    fill: &Option<Color>,
    points: usize,
    inner_ratio: f64,
) -> Result<(), cairo::Error> {
    let points = points.max(3);
    let inner_ratio = inner_ratio.clamp(0.1, 0.9);

    let center_x = bounds.origin.x as f64 + bounds.size.width as f64 / 2.0;
    let center_y = bounds.origin.y as f64 + bounds.size.height as f64 / 2.0;
    let outer_radius_x = bounds.size.width as f64 / 2.0;
    let outer_radius_y = bounds.size.height as f64 / 2.0;
    let inner_radius_x = outer_radius_x * inner_ratio;
    let inner_radius_y = outer_radius_y * inner_ratio;

    let start_angle = -std::f64::consts::FRAC_PI_2;
    let angle_step = std::f64::consts::PI / points as f64;

    // Build star path (alternating outer and inner vertices)
    for i in 0..(points * 2) {
        let angle = start_angle + angle_step * i as f64;
        let (rx, ry) = if i % 2 == 0 {
            (outer_radius_x, outer_radius_y)
        } else {
            (inner_radius_x, inner_radius_y)
        };
        let x = center_x + rx * angle.cos();
        let y = center_y + ry * angle.sin();

        if i == 0 {
            ctx.move_to(x, y);
        } else {
            ctx.line_to(x, y);
        }
    }
    ctx.close_path();

    // Fill first
    if let Some(fill_color) = fill {
        ctx.set_source_rgba(
            fill_color.r as f64,
            fill_color.g as f64,
            fill_color.b as f64,
            fill_color.a as f64,
        );
        ctx.fill_preserve()?;
    }

    // Then stroke
    if let Some(stroke_color) = stroke {
        ctx.set_source_rgba(
            stroke_color.r as f64,
            stroke_color.g as f64,
            stroke_color.b as f64,
            stroke_color.a as f64,
        );
        ctx.set_line_width(stroke_width as f64);
        ctx.stroke()?;
    } else {
        ctx.new_path();
    }

    Ok(())
}
