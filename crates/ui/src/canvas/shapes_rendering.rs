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
        ctx.set_line_width(2.0);
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
        ctx.set_line_width(2.0);
        ctx.stroke()?;
    }

    Ok(())
}

/// Draw a line shape
pub fn draw_line(ctx: &Context, bounds: &Rect, stroke: &Option<Color>) -> Result<(), cairo::Error> {
    if let Some(stroke_color) = stroke {
        ctx.set_source_rgb(
            stroke_color.r as f64,
            stroke_color.g as f64,
            stroke_color.b as f64,
        );
        ctx.set_line_width(2.0);
        ctx.move_to(bounds.origin.x as f64, bounds.origin.y as f64);
        ctx.line_to(
            bounds.origin.x as f64 + bounds.size.width as f64,
            bounds.origin.y as f64 + bounds.size.height as f64,
        );
        ctx.stroke()?;
    }

    Ok(())
}

/// Draw an arrow shape
pub fn draw_arrow(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
) -> Result<(), cairo::Error> {
    let stroke_color = stroke.unwrap_or(Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    });

    let x1 = bounds.origin.x as f64;
    let y1 = bounds.origin.y as f64;
    let x2 = bounds.origin.x as f64 + bounds.size.width as f64;
    let y2 = bounds.origin.y as f64 + bounds.size.height as f64;

    // Draw the line
    ctx.set_source_rgb(
        stroke_color.r as f64,
        stroke_color.g as f64,
        stroke_color.b as f64,
    );
    ctx.set_line_width(2.0);
    ctx.move_to(x1, y1);
    ctx.line_to(x2, y2);
    ctx.stroke()?;

    // Draw the arrowhead
    let arrow_size = 12.0;
    let angle = (y2 - y1).atan2(x2 - x1);

    // Calculate arrowhead points
    let point1_x = x2 - arrow_size * angle.cos();
    let point1_y = y2 - arrow_size * angle.sin();

    let arrow_angle = std::f64::consts::PI / 6.0; // 30 degrees
    let p1x = point1_x - arrow_size * (angle - arrow_angle).cos();
    let p1y = point1_y - arrow_size * (angle - arrow_angle).sin();
    let p2x = point1_x - arrow_size * (angle + arrow_angle).cos();
    let p2y = point1_y - arrow_size * (angle + arrow_angle).sin();

    // Draw arrowhead triangle
    ctx.move_to(x2, y2);
    ctx.line_to(p1x, p1y);
    ctx.line_to(p2x, p2y);
    ctx.close_path();
    ctx.set_source_rgb(
        stroke_color.r as f64,
        stroke_color.g as f64,
        stroke_color.b as f64,
    );
    ctx.fill()?;

    Ok(())
}

/// Draw a polygon shape (pentagon by default)
pub fn draw_polygon(
    ctx: &Context,
    bounds: &Rect,
    stroke: &Option<Color>,
) -> Result<(), cairo::Error> {
    let stroke_color = stroke.unwrap_or(Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    });

    let center_x = bounds.origin.x as f64 + bounds.size.width as f64 / 2.0;
    let center_y = bounds.origin.y as f64 + bounds.size.height as f64 / 2.0;
    let radius = (bounds.size.width as f64 / 2.0).min(bounds.size.height as f64 / 2.0);

    // Draw a pentagon (5-sided polygon)
    const SIDES: usize = 5;
    let mut first = true;

    for i in 0..SIDES {
        let angle =
            (2.0 * std::f64::consts::PI * i as f64 / SIDES as f64) - std::f64::consts::PI / 2.0;
        let x = center_x + radius * angle.cos();
        let y = center_y + radius * angle.sin();

        if first {
            ctx.move_to(x, y);
            first = false;
        } else {
            ctx.line_to(x, y);
        }
    }

    ctx.close_path();
    ctx.set_source_rgb(
        stroke_color.r as f64,
        stroke_color.g as f64,
        stroke_color.b as f64,
    );
    ctx.set_line_width(2.0);
    ctx.stroke()?;

    Ok(())
}
