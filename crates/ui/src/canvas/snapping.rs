//! Grid and guide snapping functionality for canvas objects
//!
//! Provides snapping to grid lines and user-defined guides for precise positioning.

use testruct_core::layout::{Point, Rect};

/// Configuration for snap behavior
#[derive(Debug, Clone)]
pub struct SnapConfig {
    pub snap_to_grid: bool,
    pub snap_to_guides: bool,
    pub snap_threshold: f32, // pixels
    pub grid_size: f32,      // pixels
}

impl Default for SnapConfig {
    fn default() -> Self {
        Self {
            snap_to_grid: true,
            snap_to_guides: true,
            snap_threshold: 8.0,
            grid_size: 20.0,
        }
    }
}

/// Result of a snap operation
#[derive(Debug, Clone)]
pub struct SnapResult {
    pub position: Point,
    pub snapped_x: bool,
    pub snapped_y: bool,
    pub snap_lines: Vec<SnapLine>,
}

/// A visual snap line to show snapping alignment
#[derive(Debug, Clone)]
pub struct SnapLine {
    pub line_type: SnapLineType,
    pub position: f32,       // x or y coordinate
    pub is_horizontal: bool, // true for horizontal, false for vertical
    pub bounds: (f32, f32),  // start and end of the line
}

/// Type of snap line
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapLineType {
    Grid,
    Guide,
    ObjectEdge,
}

/// Snap engine for handling snapping calculations
pub struct SnapEngine {
    config: SnapConfig,
}

impl SnapEngine {
    /// Create a new snap engine with default configuration
    pub fn new(config: SnapConfig) -> Self {
        Self { config }
    }

    /// Snap a position to grid and guides
    pub fn snap_position(&self, pos: Point) -> SnapResult {
        let mut snapped_pos = pos;
        let mut snap_lines = Vec::new();
        let mut snapped_x = false;
        let mut snapped_y = false;

        // Snap X coordinate to grid
        if self.config.snap_to_grid {
            if let Some((snapped_x_pos, snap_line)) = self.snap_to_grid_x(pos.x) {
                snapped_pos.x = snapped_x_pos;
                snap_lines.push(snap_line);
                snapped_x = true;
            }
        }

        // Snap Y coordinate to grid
        if self.config.snap_to_grid {
            if let Some((snapped_y_pos, snap_line)) = self.snap_to_grid_y(pos.y) {
                snapped_pos.y = snapped_y_pos;
                snap_lines.push(snap_line);
                snapped_y = true;
            }
        }

        SnapResult {
            position: snapped_pos,
            snapped_x,
            snapped_y,
            snap_lines,
        }
    }

    /// Snap a rectangle (for object positioning)
    pub fn snap_rect(&self, bounds: &Rect) -> SnapResult {
        let snapped = self.snap_position(bounds.origin);

        let _snapped_bounds = Rect {
            origin: snapped.position,
            size: bounds.size,
        };

        SnapResult {
            position: snapped.position,
            snapped_x: snapped.snapped_x,
            snapped_y: snapped.snapped_y,
            snap_lines: snapped.snap_lines,
        }
    }

    /// Snap X coordinate to grid
    fn snap_to_grid_x(&self, x: f32) -> Option<(f32, SnapLine)> {
        let grid_size = self.config.grid_size;
        let nearest_grid = ((x + grid_size / 2.0) / grid_size).floor() * grid_size;
        let distance = (x - nearest_grid).abs();

        if distance <= self.config.snap_threshold {
            let snap_line = SnapLine {
                line_type: SnapLineType::Grid,
                position: nearest_grid,
                is_horizontal: false,   // vertical line at x position
                bounds: (0.0, 10000.0), // full canvas height
            };
            Some((nearest_grid, snap_line))
        } else {
            None
        }
    }

    /// Snap Y coordinate to grid
    fn snap_to_grid_y(&self, y: f32) -> Option<(f32, SnapLine)> {
        let grid_size = self.config.grid_size;
        let nearest_grid = ((y + grid_size / 2.0) / grid_size).floor() * grid_size;
        let distance = (y - nearest_grid).abs();

        if distance <= self.config.snap_threshold {
            let snap_line = SnapLine {
                line_type: SnapLineType::Grid,
                position: nearest_grid,
                is_horizontal: true,    // horizontal line at y position
                bounds: (0.0, 10000.0), // full canvas width
            };
            Some((nearest_grid, snap_line))
        } else {
            None
        }
    }

    /// Snap to guides (simplified - actual implementation would take guide list)
    pub fn snap_to_guides(&self, pos: Point, guides: &[GuideInfo]) -> SnapResult {
        let mut snapped_pos = pos;
        let mut snap_lines = Vec::new();
        let mut snapped_x = false;
        let mut snapped_y = false;

        // Check vertical guides (for X snapping)
        for guide in guides {
            if guide.is_vertical {
                let distance = (pos.x - guide.position).abs();
                if distance <= self.config.snap_threshold {
                    snapped_pos.x = guide.position;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::Guide,
                        position: guide.position,
                        is_horizontal: false,
                        bounds: (0.0, 10000.0),
                    });
                    snapped_x = true;
                    break;
                }
            }
        }

        // Check horizontal guides (for Y snapping)
        for guide in guides {
            if !guide.is_vertical {
                let distance = (pos.y - guide.position).abs();
                if distance <= self.config.snap_threshold {
                    snapped_pos.y = guide.position;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::Guide,
                        position: guide.position,
                        is_horizontal: true,
                        bounds: (0.0, 10000.0),
                    });
                    snapped_y = true;
                    break;
                }
            }
        }

        SnapResult {
            position: snapped_pos,
            snapped_x,
            snapped_y,
            snap_lines,
        }
    }

    /// Update configuration
    pub fn set_config(&mut self, config: SnapConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn config(&self) -> &SnapConfig {
        &self.config
    }
}

/// Information about a guide for snapping
#[derive(Debug, Clone)]
pub struct GuideInfo {
    pub position: f32,
    pub is_vertical: bool, // true = vertical (x), false = horizontal (y)
}

/// Information about an object's alignment points
#[derive(Debug, Clone)]
pub struct ObjectAlignmentPoints {
    pub id: uuid::Uuid,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
    pub center_x: f32,
    pub center_y: f32,
}

impl ObjectAlignmentPoints {
    /// Create alignment points from a rectangle bounds
    pub fn from_rect(id: uuid::Uuid, bounds: &Rect) -> Self {
        Self {
            id,
            left: bounds.origin.x,
            right: bounds.origin.x + bounds.size.width,
            top: bounds.origin.y,
            bottom: bounds.origin.y + bounds.size.height,
            center_x: bounds.origin.x + bounds.size.width / 2.0,
            center_y: bounds.origin.y + bounds.size.height / 2.0,
        }
    }
}

/// Smart guide engine for object-to-object alignment
pub struct SmartGuideEngine {
    pub snap_threshold: f32,
}

impl Default for SmartGuideEngine {
    fn default() -> Self {
        Self {
            snap_threshold: 8.0,
        }
    }
}

impl SmartGuideEngine {
    /// Calculate smart guide snap lines for a dragging object against other objects
    /// Returns the adjusted position and snap lines to display
    pub fn calculate_snap(
        &self,
        dragging_bounds: &Rect,
        other_objects: &[ObjectAlignmentPoints],
        canvas_width: f32,
        canvas_height: f32,
    ) -> SmartGuideResult {
        let dragging = ObjectAlignmentPoints::from_rect(uuid::Uuid::nil(), dragging_bounds);

        let mut snap_lines = Vec::new();
        let mut adjusted_x = dragging_bounds.origin.x;
        let mut adjusted_y = dragging_bounds.origin.y;
        let mut snapped_x = false;
        let mut snapped_y = false;

        // Check alignment with each other object
        for other in other_objects {
            // Skip self (shouldn't happen but just in case)
            if other.id == dragging.id {
                continue;
            }

            // Vertical alignments (X axis)
            if !snapped_x {
                // Left to left
                if (dragging.left - other.left).abs() <= self.snap_threshold {
                    adjusted_x = other.left;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.left,
                        is_horizontal: false,
                        bounds: (0.0, canvas_height),
                    });
                    snapped_x = true;
                }
                // Right to right
                else if (dragging.right - other.right).abs() <= self.snap_threshold {
                    adjusted_x = other.right - dragging_bounds.size.width;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.right,
                        is_horizontal: false,
                        bounds: (0.0, canvas_height),
                    });
                    snapped_x = true;
                }
                // Left to right
                else if (dragging.left - other.right).abs() <= self.snap_threshold {
                    adjusted_x = other.right;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.right,
                        is_horizontal: false,
                        bounds: (0.0, canvas_height),
                    });
                    snapped_x = true;
                }
                // Right to left
                else if (dragging.right - other.left).abs() <= self.snap_threshold {
                    adjusted_x = other.left - dragging_bounds.size.width;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.left,
                        is_horizontal: false,
                        bounds: (0.0, canvas_height),
                    });
                    snapped_x = true;
                }
                // Center to center (X)
                else if (dragging.center_x - other.center_x).abs() <= self.snap_threshold {
                    adjusted_x = other.center_x - dragging_bounds.size.width / 2.0;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.center_x,
                        is_horizontal: false,
                        bounds: (0.0, canvas_height),
                    });
                    snapped_x = true;
                }
            }

            // Horizontal alignments (Y axis)
            if !snapped_y {
                // Top to top
                if (dragging.top - other.top).abs() <= self.snap_threshold {
                    adjusted_y = other.top;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.top,
                        is_horizontal: true,
                        bounds: (0.0, canvas_width),
                    });
                    snapped_y = true;
                }
                // Bottom to bottom
                else if (dragging.bottom - other.bottom).abs() <= self.snap_threshold {
                    adjusted_y = other.bottom - dragging_bounds.size.height;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.bottom,
                        is_horizontal: true,
                        bounds: (0.0, canvas_width),
                    });
                    snapped_y = true;
                }
                // Top to bottom
                else if (dragging.top - other.bottom).abs() <= self.snap_threshold {
                    adjusted_y = other.bottom;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.bottom,
                        is_horizontal: true,
                        bounds: (0.0, canvas_width),
                    });
                    snapped_y = true;
                }
                // Bottom to top
                else if (dragging.bottom - other.top).abs() <= self.snap_threshold {
                    adjusted_y = other.top - dragging_bounds.size.height;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.top,
                        is_horizontal: true,
                        bounds: (0.0, canvas_width),
                    });
                    snapped_y = true;
                }
                // Center to center (Y)
                else if (dragging.center_y - other.center_y).abs() <= self.snap_threshold {
                    adjusted_y = other.center_y - dragging_bounds.size.height / 2.0;
                    snap_lines.push(SnapLine {
                        line_type: SnapLineType::ObjectEdge,
                        position: other.center_y,
                        is_horizontal: true,
                        bounds: (0.0, canvas_width),
                    });
                    snapped_y = true;
                }
            }

            // If we've snapped both axes, no need to check more objects
            if snapped_x && snapped_y {
                break;
            }
        }

        SmartGuideResult {
            adjusted_position: Point { x: adjusted_x, y: adjusted_y },
            snap_lines,
            snapped_x,
            snapped_y,
        }
    }
}

/// Result of smart guide calculation
#[derive(Debug, Clone)]
pub struct SmartGuideResult {
    pub adjusted_position: Point,
    pub snap_lines: Vec<SnapLine>,
    pub snapped_x: bool,
    pub snapped_y: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snap_to_grid_x() {
        let config = SnapConfig {
            snap_to_grid: true,
            snap_to_guides: false,
            snap_threshold: 8.0,
            grid_size: 20.0,
        };
        let engine = SnapEngine::new(config);

        // Test snapping to 0
        let result = engine.snap_to_grid_x(2.0);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 0.0);

        // Test snapping to 20
        let result = engine.snap_to_grid_x(22.0);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 20.0);

        // Test no snap (too far)
        let result = engine.snap_to_grid_x(50.0);
        assert!(result.is_none());
    }

    #[test]
    fn test_snap_to_grid_y() {
        let config = SnapConfig {
            snap_to_grid: true,
            snap_to_guides: false,
            snap_threshold: 8.0,
            grid_size: 20.0,
        };
        let engine = SnapEngine::new(config);

        let result = engine.snap_to_grid_y(5.0);
        assert!(result.is_some());
        assert_eq!(result.unwrap().0, 0.0);
    }

    #[test]
    fn test_snap_position() {
        let config = SnapConfig::default();
        let engine = SnapEngine::new(config);

        let pos = Point { x: 2.0, y: 3.0 };
        let result = engine.snap_position(pos);

        assert!(result.snapped_x || result.snapped_y);
    }

    #[test]
    fn test_snap_to_guides() {
        let config = SnapConfig {
            snap_to_grid: false,
            snap_to_guides: true,
            snap_threshold: 8.0,
            grid_size: 20.0,
        };
        let engine = SnapEngine::new(config);

        let guides = vec![
            GuideInfo {
                position: 100.0,
                is_vertical: true,
            },
            GuideInfo {
                position: 200.0,
                is_vertical: false,
            },
        ];

        let pos = Point { x: 105.0, y: 150.0 };
        let result = engine.snap_to_guides(pos, &guides);

        assert_eq!(result.position.x, 100.0);
    }
}
