//! Canvas rendering pipeline - Cairo drawing system
//!
//! This module provides the complete drawing implementation for the canvas,
//! including document objects (shapes, text, images) and UI overlays.
//!
//! Grid, ruler, and guide rendering has been moved to the `grid_rendering` module.
//! Text rendering has been moved to the `rendering_text` module.
//! Selection and resize handles have been moved to the `rendering_selection` module.
//! Image rendering has been moved to the `rendering_images` module.

use gtk4::cairo::{self, Context};
use testruct_core::layout::{Point, Size};

// Re-export types from grid_rendering for backward compatibility
pub use super::grid_rendering::{Guide, GuideOrientation, RulerConfig, GridConfig, GridStyle};

// Re-export from rendering_text module
pub use super::rendering_text::{
    draw_text_cursor, draw_text_editing_frame, draw_text_element, measure_text_height, TEXT_PADDING,
};

// Re-export from rendering_selection module
pub use super::rendering_selection::{draw_resize_handles, draw_selection_box, ResizeHandle};

// Re-export from rendering_images module
pub use super::rendering_images::{draw_image_element, draw_image_placeholder};

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
    pub grid_config: GridConfig,
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
            grid_config: GridConfig::default(),
            guides: Vec::new(),
            snap_to_guides: true,
            guide_snap_distance: 5.0,
        }
    }
}

impl RenderConfig {
    /// Get grid spacing (convenience method)
    pub fn grid_spacing(&self) -> f32 {
        self.grid_config.spacing
    }

    /// Set grid spacing
    pub fn set_grid_spacing(&mut self, spacing: f32) {
        self.grid_config.spacing = spacing;
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
pub use super::grid_rendering::{draw_grid, draw_grid_with_config};

/// Draw page border
pub fn draw_page_border(ctx: &Context, page_size: &Size) -> Result<(), cairo::Error> {
    ctx.set_source_rgb(0.8, 0.8, 0.8);
    ctx.set_line_width(1.0);
    ctx.rectangle(0.0, 0.0, page_size.width as f64, page_size.height as f64);
    ctx.stroke()?;
    Ok(())
}

// Re-export shape drawing functions for backward compatibility
pub use super::shapes_rendering::{
    draw_arrow, draw_ellipse, draw_line, draw_polygon, draw_rectangle,
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
pub fn snap_rect_to_grid(
    rect: &testruct_core::layout::Rect,
    spacing: f32,
) -> testruct_core::layout::Rect {
    testruct_core::layout::Rect::new(snap_point_to_grid(&rect.origin, spacing), rect.size.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

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
