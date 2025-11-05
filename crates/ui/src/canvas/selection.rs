//! Canvas object selection management
//!
//! Handles selection state, selection modes, and selection queries.

use std::cell::RefCell;
use std::rc::Rc;
use testruct_core::layout::Rect;

/// Selection mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectionMode {
    /// Single object selection (default)
    Single,
    /// Multiple object selection
    Multiple,
}

/// Selection state manager
#[derive(Clone)]
pub struct SelectionState {
    selected: Rc<RefCell<Vec<uuid::Uuid>>>,
    mode: Rc<RefCell<SelectionMode>>,
}

impl SelectionState {
    /// Create a new selection state
    pub fn new() -> Self {
        Self {
            selected: Rc::new(RefCell::new(Vec::new())),
            mode: Rc::new(RefCell::new(SelectionMode::Single)),
        }
    }

    /// Select a single object
    pub fn select(&self, object_id: uuid::Uuid) {
        let mode = *self.mode.borrow();
        if mode == SelectionMode::Single {
            self.clear();
        }
        let mut selected = self.selected.borrow_mut();
        if !selected.contains(&object_id) {
            selected.push(object_id);
        }
    }

    /// Add object to selection
    pub fn add(&self, object_id: uuid::Uuid) {
        let mut selected = self.selected.borrow_mut();
        if !selected.contains(&object_id) {
            selected.push(object_id);
        }
    }

    /// Remove object from selection
    pub fn remove(&self, object_id: uuid::Uuid) {
        let mut selected = self.selected.borrow_mut();
        selected.retain(|id| *id != object_id);
    }

    /// Toggle object selection
    pub fn toggle(&self, object_id: uuid::Uuid) {
        let mut selected = self.selected.borrow_mut();
        if let Some(pos) = selected.iter().position(|id| *id == object_id) {
            selected.remove(pos);
        } else {
            selected.push(object_id);
        }
    }

    /// Clear all selections
    pub fn clear(&self) {
        self.selected.borrow_mut().clear();
    }

    /// Check if object is selected
    pub fn is_selected(&self, object_id: uuid::Uuid) -> bool {
        self.selected.borrow().contains(&object_id)
    }

    /// Get all selected object IDs
    pub fn selected(&self) -> Vec<uuid::Uuid> {
        self.selected.borrow().clone()
    }

    /// Get count of selected objects
    pub fn count(&self) -> usize {
        self.selected.borrow().len()
    }

    /// Check if any objects are selected
    pub fn has_selection(&self) -> bool {
        !self.selected.borrow().is_empty()
    }

    /// Set selection mode
    pub fn set_mode(&self, mode: SelectionMode) {
        *self.mode.borrow_mut() = mode;
    }

    /// Get current selection mode
    pub fn mode(&self) -> SelectionMode {
        *self.mode.borrow()
    }
}

impl Default for SelectionState {
    fn default() -> Self {
        Self::new()
    }
}

/// Selection bounds calculation
pub struct SelectionBounds;

impl SelectionBounds {
    /// Calculate bounding box for all selected objects
    pub fn calculate(objects: &[(uuid::Uuid, &Rect)], selected_ids: &[uuid::Uuid]) -> Option<Rect> {
        let selected_bounds: Vec<_> = objects
            .iter()
            .filter(|(id, _)| selected_ids.contains(id))
            .map(|(_, bounds)| bounds)
            .collect();

        if selected_bounds.is_empty() {
            return None;
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for bounds in selected_bounds {
            min_x = min_x.min(bounds.origin.x);
            min_y = min_y.min(bounds.origin.y);
            max_x = max_x.max(bounds.origin.x + bounds.size.width);
            max_y = max_y.max(bounds.origin.y + bounds.size.height);
        }

        Some(Rect::new(
            testruct_core::layout::Point::new(min_x, min_y),
            testruct_core::layout::Size::new(max_x - min_x, max_y - min_y),
        ))
    }
}

/// Hit testing for object selection
pub struct HitTest;

impl HitTest {
    /// Find object at position
    pub fn hit_test(
        objects: &[(uuid::Uuid, &Rect)],
        pos_x: f64,
        pos_y: f64,
    ) -> Option<uuid::Uuid> {
        // Test in reverse order (top-to-bottom) for proper layering
        for (id, bounds) in objects.iter().rev() {
            if pos_x >= bounds.origin.x as f64
                && pos_x <= bounds.origin.x as f64 + bounds.size.width as f64
                && pos_y >= bounds.origin.y as f64
                && pos_y <= bounds.origin.y as f64 + bounds.size.height as f64
            {
                return Some(*id);
            }
        }
        None
    }

    /// Find objects in selection rectangle
    pub fn hit_test_rect(
        objects: &[(uuid::Uuid, &Rect)],
        select_x1: f64,
        select_y1: f64,
        select_x2: f64,
        select_y2: f64,
    ) -> Vec<uuid::Uuid> {
        let min_x = select_x1.min(select_x2);
        let max_x = select_x1.max(select_x2);
        let min_y = select_y1.min(select_y2);
        let max_y = select_y1.max(select_y2);

        objects
            .iter()
            .filter(|(_, bounds)| {
                let obj_left = bounds.origin.x as f64;
                let obj_right = obj_left + bounds.size.width as f64;
                let obj_top = bounds.origin.y as f64;
                let obj_bottom = obj_top + bounds.size.height as f64;

                // Check if object intersects with selection rectangle
                obj_right >= min_x && obj_left <= max_x && obj_bottom >= min_y && obj_top <= max_y
            })
            .map(|(id, _)| *id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_state() {
        let state = SelectionState::new();
        let id = uuid::Uuid::new_v4();

        assert!(!state.is_selected(id));
        assert_eq!(state.count(), 0);

        state.select(id);
        assert!(state.is_selected(id));
        assert_eq!(state.count(), 1);

        state.clear();
        assert!(!state.is_selected(id));
        assert_eq!(state.count(), 0);
    }

    #[test]
    fn test_multiple_selection() {
        let state = SelectionState::new();
        state.set_mode(SelectionMode::Multiple);

        let id1 = uuid::Uuid::new_v4();
        let id2 = uuid::Uuid::new_v4();

        state.add(id1);
        state.add(id2);

        assert_eq!(state.count(), 2);
        assert!(state.is_selected(id1));
        assert!(state.is_selected(id2));
    }

    #[test]
    fn test_selection_toggle() {
        let state = SelectionState::new();
        state.set_mode(SelectionMode::Multiple);

        let id = uuid::Uuid::new_v4();

        state.toggle(id);
        assert!(state.is_selected(id));

        state.toggle(id);
        assert!(!state.is_selected(id));
    }
}
