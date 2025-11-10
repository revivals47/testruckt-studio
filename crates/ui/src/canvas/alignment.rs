//! Alignment and distribution tools for multiple selected objects
//!
//! Provides functions for aligning and distributing objects on the canvas

use testruct_core::layout::Rect;

/// Alignment and distribution tools for canvas objects
pub struct AlignmentTools;

impl AlignmentTools {
    /// Align selected objects to the left edge
    pub fn align_left(rects: &mut [Rect]) {
        if rects.is_empty() {
            return;
        }
        let count = rects.len();
        let min_x = rects.iter().map(|r| r.origin.x).fold(f32::MAX, f32::min);
        for rect in rects.iter_mut() {
            rect.origin.x = min_x;
        }
        tracing::info!("✅ Aligned {} objects to left", count);
    }

    /// Align selected objects to the right edge
    pub fn align_right(rects: &mut [Rect]) {
        if rects.is_empty() {
            return;
        }
        let count = rects.len();
        let max_right = rects
            .iter()
            .map(|r| r.origin.x + r.size.width)
            .fold(f32::MIN, f32::max);
        for rect in rects.iter_mut() {
            rect.origin.x = max_right - rect.size.width;
        }
        tracing::info!("✅ Aligned {} objects to right", count);
    }

    /// Align selected objects to the center horizontally
    pub fn align_center_h(rects: &mut [Rect]) {
        if rects.is_empty() {
            return;
        }
        let count = rects.len();
        let avg_center_x = rects
            .iter()
            .map(|r| r.origin.x + r.size.width / 2.0)
            .sum::<f32>()
            / count as f32;
        for rect in rects.iter_mut() {
            rect.origin.x = avg_center_x - rect.size.width / 2.0;
        }
        tracing::info!("✅ Aligned {} objects to center horizontally", count);
    }

    /// Align selected objects to the top edge
    pub fn align_top(rects: &mut [Rect]) {
        if rects.is_empty() {
            return;
        }
        let count = rects.len();
        let min_y = rects.iter().map(|r| r.origin.y).fold(f32::MAX, f32::min);
        for rect in rects.iter_mut() {
            rect.origin.y = min_y;
        }
        tracing::info!("✅ Aligned {} objects to top", count);
    }

    /// Align selected objects to the bottom edge
    pub fn align_bottom(rects: &mut [Rect]) {
        if rects.is_empty() {
            return;
        }
        let count = rects.len();
        let max_bottom = rects
            .iter()
            .map(|r| r.origin.y + r.size.height)
            .fold(f32::MIN, f32::max);
        for rect in rects.iter_mut() {
            rect.origin.y = max_bottom - rect.size.height;
        }
        tracing::info!("✅ Aligned {} objects to bottom", count);
    }

    /// Align selected objects to the center vertically
    pub fn align_middle(rects: &mut [Rect]) {
        if rects.is_empty() {
            return;
        }
        let count = rects.len();
        let avg_center_y = rects
            .iter()
            .map(|r| r.origin.y + r.size.height / 2.0)
            .sum::<f32>()
            / count as f32;
        for rect in rects.iter_mut() {
            rect.origin.y = avg_center_y - rect.size.height / 2.0;
        }
        tracing::info!("✅ Aligned {} objects to middle vertically", count);
    }

    /// Distribute selected objects evenly with equal spacing horizontally
    pub fn distribute_h_equal(rects: &mut [Rect]) {
        if rects.len() < 3 {
            tracing::warn!("⚠️  Need at least 3 objects to distribute");
            return;
        }

        let count = rects.len();

        // Sort by x position
        rects.sort_by(|a, b| a.origin.x.partial_cmp(&b.origin.x).unwrap());

        // Calculate total width and gaps
        let leftmost_x = rects.first().unwrap().origin.x;
        let rightmost_right = rects.last().unwrap().origin.x + rects.last().unwrap().size.width;
        let total_span = rightmost_right - leftmost_x;

        let total_object_width: f32 = rects.iter().map(|r| r.size.width).sum();
        let total_gap = total_span - total_object_width;
        let gap = total_gap / (count - 1) as f32;

        let mut current_x = leftmost_x;
        for rect in rects.iter_mut() {
            rect.origin.x = current_x;
            current_x += rect.size.width + gap;
        }
        tracing::info!("✅ Distributed {} objects evenly horizontally", count);
    }

    /// Distribute selected objects evenly with equal spacing vertically
    pub fn distribute_v_equal(rects: &mut [Rect]) {
        if rects.len() < 3 {
            tracing::warn!("⚠️  Need at least 3 objects to distribute");
            return;
        }

        let count = rects.len();

        // Sort by y position
        rects.sort_by(|a, b| a.origin.y.partial_cmp(&b.origin.y).unwrap());

        // Calculate total height and gaps
        let topmost_y = rects.first().unwrap().origin.y;
        let bottommost_bottom = rects.last().unwrap().origin.y + rects.last().unwrap().size.height;
        let total_span = bottommost_bottom - topmost_y;

        let total_object_height: f32 = rects.iter().map(|r| r.size.height).sum();
        let total_gap = total_span - total_object_height;
        let gap = total_gap / (count - 1) as f32;

        let mut current_y = topmost_y;
        for rect in rects.iter_mut() {
            rect.origin.y = current_y;
            current_y += rect.size.height + gap;
        }
        tracing::info!("✅ Distributed {} objects evenly vertically", count);
    }

    /// Get the bounding box of a set of rects
    pub fn get_bounding_box(rects: &[Rect]) -> Option<Rect> {
        if rects.is_empty() {
            return None;
        }

        let min_x = rects.iter().map(|r| r.origin.x).fold(f32::MAX, f32::min);
        let min_y = rects.iter().map(|r| r.origin.y).fold(f32::MAX, f32::min);
        let max_right = rects
            .iter()
            .map(|r| r.origin.x + r.size.width)
            .fold(f32::MIN, f32::max);
        let max_bottom = rects
            .iter()
            .map(|r| r.origin.y + r.size.height)
            .fold(f32::MIN, f32::max);

        Some(Rect {
            origin: testruct_core::layout::Point::new(min_x, min_y),
            size: testruct_core::layout::Size::new(max_right - min_x, max_bottom - min_y),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testruct_core::layout::{Point, Size};

    fn create_test_rect(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            origin: Point::new(x, y),
            size: Size::new(w, h),
        }
    }

    #[test]
    fn test_align_left() {
        let mut rects = vec![
            create_test_rect(100.0, 0.0, 50.0, 50.0),
            create_test_rect(150.0, 0.0, 50.0, 50.0),
            create_test_rect(50.0, 0.0, 50.0, 50.0),
        ];

        AlignmentTools::align_left(&mut rects);

        assert_eq!(rects[0].origin.x, 50.0);
        assert_eq!(rects[1].origin.x, 50.0);
        assert_eq!(rects[2].origin.x, 50.0);
    }

    #[test]
    fn test_align_right() {
        let mut rects = vec![
            create_test_rect(100.0, 0.0, 50.0, 50.0),
            create_test_rect(150.0, 0.0, 50.0, 50.0),
            create_test_rect(50.0, 0.0, 50.0, 50.0),
        ];

        AlignmentTools::align_right(&mut rects);

        let right_edge = 150.0 + 50.0;
        for rect in &rects {
            assert_eq!(rect.origin.x + rect.size.width, right_edge);
        }
    }

    #[test]
    fn test_align_center_h() {
        let mut rects = vec![
            create_test_rect(0.0, 0.0, 50.0, 50.0),
            create_test_rect(100.0, 0.0, 50.0, 50.0),
        ];

        AlignmentTools::align_center_h(&mut rects);

        let avg_center = (0.0 + 25.0 + 100.0 + 25.0) / 2.0;
        for rect in &rects {
            assert!((rect.origin.x + rect.size.width / 2.0 - avg_center).abs() < 0.01);
        }
    }

    #[test]
    fn test_align_top() {
        let mut rects = vec![
            create_test_rect(0.0, 100.0, 50.0, 50.0),
            create_test_rect(0.0, 150.0, 50.0, 50.0),
            create_test_rect(0.0, 50.0, 50.0, 50.0),
        ];

        AlignmentTools::align_top(&mut rects);

        assert_eq!(rects[0].origin.y, 50.0);
        assert_eq!(rects[1].origin.y, 50.0);
        assert_eq!(rects[2].origin.y, 50.0);
    }

    #[test]
    fn test_distribute_h_equal() {
        let mut rects = vec![
            create_test_rect(0.0, 0.0, 50.0, 50.0),
            create_test_rect(100.0, 0.0, 50.0, 50.0),
            create_test_rect(200.0, 0.0, 50.0, 50.0),
        ];

        AlignmentTools::distribute_h_equal(&mut rects);

        // Objects should be evenly distributed
        // Total span is 250 (from 0 to 250)
        // Total width is 150 (50+50+50)
        // Total gap is 100, divided by 2 gaps = 50 each
        assert_eq!(rects[0].origin.x, 0.0);
        assert_eq!(rects[1].origin.x, 100.0);
        assert_eq!(rects[2].origin.x, 200.0);
    }

    #[test]
    fn test_bounding_box() {
        let rects = vec![
            create_test_rect(10.0, 20.0, 30.0, 40.0),
            create_test_rect(50.0, 30.0, 20.0, 50.0),
        ];

        let bbox = AlignmentTools::get_bounding_box(&rects).unwrap();

        assert_eq!(bbox.origin.x, 10.0);
        assert_eq!(bbox.origin.y, 20.0);
        assert_eq!(bbox.size.width, 60.0); // 10 to 70
        assert_eq!(bbox.size.height, 60.0); // 20 to 80
    }

    #[test]
    fn test_distribute_needs_3_objects() {
        let mut rects = vec![create_test_rect(0.0, 0.0, 50.0, 50.0)];

        // Should not panic, just log warning
        AlignmentTools::distribute_h_equal(&mut rects);
    }
}
