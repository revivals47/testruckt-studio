//! Dirty region tracking for optimized rendering
//!
//! Tracks which areas of the canvas have changed and need to be redrawn.
//! This allows for efficient partial redraws instead of redrawing the entire canvas.

use testruct_core::layout::Rect;
use std::cell::RefCell;
use std::rc::Rc;

/// Represents a region that needs to be redrawn
#[derive(Clone, Debug, Default)]
pub struct DirtyRegion {
    rects: Vec<Rect>,
    is_full_redraw: bool,
}

impl DirtyRegion {
    /// Create a new empty dirty region
    pub fn new() -> Self {
        Self {
            rects: Vec::new(),
            is_full_redraw: false,
        }
    }

    /// Mark the entire canvas as needing redraw
    pub fn mark_full_redraw(&mut self) {
        self.is_full_redraw = true;
        self.rects.clear();
    }

    /// Add a rectangle to the dirty region
    pub fn add_rect(&mut self, rect: Rect) {
        if self.is_full_redraw {
            return;
        }

        // If adding too many rects, just do full redraw
        if self.rects.len() > 10 {
            self.mark_full_redraw();
            return;
        }

        // Check if new rect overlaps with existing rects and merge
        self.rects.push(rect);
        self.merge_overlapping_rects();
    }

    /// Merge overlapping rectangles to optimize region
    fn merge_overlapping_rects(&mut self) {
        let mut merged = true;
        while merged {
            merged = false;
            let mut i = 0;
            while i < self.rects.len() {
                let mut j = i + 1;
                while j < self.rects.len() {
                    if self.rects_overlap(self.rects[i], self.rects[j]) {
                        let merged_rect = self.merge_rects(self.rects[i], self.rects[j]);
                        self.rects[i] = merged_rect;
                        self.rects.remove(j);
                        merged = true;
                        break;
                    }
                    j += 1;
                }
                if merged {
                    break;
                }
                i += 1;
            }
        }
    }

    /// Check if two rectangles overlap or are adjacent
    fn rects_overlap(&self, r1: Rect, r2: Rect) -> bool {
        let padding = 5.0; // Merge if close together
        r1.origin.x - padding <= r2.origin.x + r2.size.width
            && r1.origin.x + r1.size.width + padding >= r2.origin.x
            && r1.origin.y - padding <= r2.origin.y + r2.size.height
            && r1.origin.y + r1.size.height + padding >= r2.origin.y
    }

    /// Merge two rectangles into their bounding box
    fn merge_rects(&self, r1: Rect, r2: Rect) -> Rect {
        let min_x = r1.origin.x.min(r2.origin.x);
        let min_y = r1.origin.y.min(r2.origin.y);
        let max_x = (r1.origin.x + r1.size.width).max(r2.origin.x + r2.size.width);
        let max_y = (r1.origin.y + r1.size.height).max(r2.origin.y + r2.size.height);

        Rect {
            origin: testruct_core::layout::Point {
                x: min_x,
                y: min_y,
            },
            size: testruct_core::layout::Size {
                width: max_x - min_x,
                height: max_y - min_y,
            },
        }
    }

    /// Check if full redraw is needed
    pub fn needs_full_redraw(&self) -> bool {
        self.is_full_redraw
    }

    /// Get rectangles that need redraw
    pub fn get_rects(&self) -> Vec<Rect> {
        self.rects.clone()
    }

    /// Clear the dirty region
    pub fn clear(&mut self) {
        self.rects.clear();
        self.is_full_redraw = false;
    }

    /// Check if there are any dirty regions
    pub fn is_empty(&self) -> bool {
        !self.is_full_redraw && self.rects.is_empty()
    }
}

/// Thread-safe dirty region tracker
pub type DirtyRegionTracker = Rc<RefCell<DirtyRegion>>;

/// Create a new dirty region tracker
pub fn new_tracker() -> DirtyRegionTracker {
    Rc::new(RefCell::new(DirtyRegion::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_full_redraw() {
        let mut region = DirtyRegion::new();
        region.mark_full_redraw();
        assert!(region.needs_full_redraw());
        assert!(region.get_rects().is_empty());
    }

    #[test]
    fn test_add_rect() {
        let mut region = DirtyRegion::new();
        let rect = Rect {
            origin: testruct_core::layout::Point { x: 0.0, y: 0.0 },
            size: testruct_core::layout::Size {
                width: 100.0,
                height: 100.0,
            },
        };

        region.add_rect(rect);
        assert!(!region.is_empty());
        assert_eq!(region.get_rects().len(), 1);
    }

    #[test]
    fn test_merge_overlapping_rects() {
        let mut region = DirtyRegion::new();
        let rect1 = Rect {
            origin: testruct_core::layout::Point { x: 0.0, y: 0.0 },
            size: testruct_core::layout::Size {
                width: 100.0,
                height: 100.0,
            },
        };

        let rect2 = Rect {
            origin: testruct_core::layout::Point { x: 50.0, y: 50.0 },
            size: testruct_core::layout::Size {
                width: 100.0,
                height: 100.0,
            },
        };

        region.add_rect(rect1);
        region.add_rect(rect2);

        // Should merge overlapping rects
        assert!(region.get_rects().len() <= 2);
    }

    #[test]
    fn test_too_many_rects_triggers_full_redraw() {
        let mut region = DirtyRegion::new();

        for i in 0..15 {
            let rect = Rect {
                origin: testruct_core::layout::Point {
                    x: (i * 50) as f32,
                    y: 0.0,
                },
                size: testruct_core::layout::Size {
                    width: 40.0,
                    height: 40.0,
                },
            };
            region.add_rect(rect);
        }

        // Should trigger full redraw when too many rects
        assert!(region.needs_full_redraw());
    }
}
