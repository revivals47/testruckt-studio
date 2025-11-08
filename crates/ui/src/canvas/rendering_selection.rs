//! Selection and resize handle rendering for canvas
//!
//! This module handles rendering of selection boxes and resize handles
//! for selected objects on the canvas.

use gtk4::cairo::{self, Context};
use testruct_core::layout::{Point, Rect};
use testruct_core::typography::Color;

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
            ResizeHandle::Top => {
                Point::new(bounds.origin.x + bounds.size.width / 2.0, bounds.origin.y)
            }
            ResizeHandle::TopRight => {
                Point::new(bounds.origin.x + bounds.size.width, bounds.origin.y)
            }
            ResizeHandle::Right => Point::new(
                bounds.origin.x + bounds.size.width,
                bounds.origin.y + bounds.size.height / 2.0,
            ),
            ResizeHandle::BottomRight => Point::new(
                bounds.origin.x + bounds.size.width,
                bounds.origin.y + bounds.size.height,
            ),
            ResizeHandle::Bottom => Point::new(
                bounds.origin.x + bounds.size.width / 2.0,
                bounds.origin.y + bounds.size.height,
            ),
            ResizeHandle::BottomLeft => {
                Point::new(bounds.origin.x, bounds.origin.y + bounds.size.height)
            }
            ResizeHandle::Left => {
                Point::new(bounds.origin.x, bounds.origin.y + bounds.size.height / 2.0)
            }
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

/// Draw a selection rectangle
pub fn draw_selection_box(
    ctx: &Context,
    bounds: &Rect,
    stroke_color: &Color,
) -> Result<(), cairo::Error> {
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

    // Semi-transparent fill
    ctx.set_source_rgba(
        stroke_color.r as f64,
        stroke_color.g as f64,
        stroke_color.b as f64,
        0.1,
    );
    ctx.rectangle(
        bounds.origin.x as f64,
        bounds.origin.y as f64,
        bounds.size.width as f64,
        bounds.size.height as f64,
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

        ctx.set_source_rgb(
            handle_color.r as f64,
            handle_color.g as f64,
            handle_color.b as f64,
        );
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resize_handle_positions() {
        let bounds = Rect::new(
            Point::new(10.0, 20.0),
            testruct_core::layout::Size::new(100.0, 80.0),
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
}
