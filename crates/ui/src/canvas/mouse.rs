//! Canvas mouse input handling
//!
//! This module implements mouse event handling for object selection,
//! dragging, and resizing on the canvas.

use std::cell::RefCell;
use std::rc::Rc;
use testruct_core::layout::{Point, Rect};

/// Mouse interaction state
#[derive(Clone, Debug, PartialEq)]
pub enum MouseInteraction {
    /// No active interaction
    Idle,
    /// Dragging an object
    Dragging {
        object_id: uuid::Uuid,
        start_pos: Point,
        offset_x: f64,
        offset_y: f64,
    },
    /// Resizing an object
    Resizing {
        object_id: uuid::Uuid,
        handle: ResizeHandle,
        start_pos: Point,
        original_bounds: Rect,
    },
    /// Selection drag (marquee)
    SelectionDrag {
        start_pos: Point,
        current_pos: Point,
    },
    /// Creating a guide from ruler
    CreatingGuide {
        orientation: GuideOrientation,
        position: f64,
    },
}

/// Which resize handle is being dragged
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

/// Guide orientation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideOrientation {
    Horizontal,
    Vertical,
}

/// Mouse position in canvas coordinates
#[derive(Clone, Copy, Debug)]
pub struct CanvasMousePos {
    pub x: f64,
    pub y: f64,
}

impl CanvasMousePos {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn to_point(self) -> Point {
        Point::new(self.x as f32, self.y as f32)
    }
}

/// Convert widget coordinates to canvas coordinates
pub fn widget_to_canvas(
    widget_x: f64,
    widget_y: f64,
    zoom: f64,
    pan_x: f64,
    pan_y: f64,
    ruler_size: f64,
) -> CanvasMousePos {
    let canvas_x = ((widget_x - ruler_size - pan_x) / zoom).max(0.0);
    let canvas_y = ((widget_y - ruler_size - pan_y) / zoom).max(0.0);
    CanvasMousePos::new(canvas_x, canvas_y)
}

/// Test if a point is within a rectangle bounds
pub fn point_in_bounds(point: CanvasMousePos, bounds: &Rect) -> bool {
    point.x >= bounds.origin.x as f64
        && point.x <= bounds.origin.x as f64 + bounds.size.width as f64
        && point.y >= bounds.origin.y as f64
        && point.y <= bounds.origin.y as f64 + bounds.size.height as f64
}

/// Test if a point is near a resize handle
pub fn test_resize_handle(
    point: CanvasMousePos,
    bounds: &Rect,
    handle_size: f64,
) -> Option<ResizeHandle> {
    let half_size = handle_size / 2.0;
    let handles = [
        (
            ResizeHandle::TopLeft,
            bounds.origin.x as f64,
            bounds.origin.y as f64,
        ),
        (
            ResizeHandle::Top,
            bounds.origin.x as f64 + bounds.size.width as f64 / 2.0,
            bounds.origin.y as f64,
        ),
        (
            ResizeHandle::TopRight,
            bounds.origin.x as f64 + bounds.size.width as f64,
            bounds.origin.y as f64,
        ),
        (
            ResizeHandle::Right,
            bounds.origin.x as f64 + bounds.size.width as f64,
            bounds.origin.y as f64 + bounds.size.height as f64 / 2.0,
        ),
        (
            ResizeHandle::BottomRight,
            bounds.origin.x as f64 + bounds.size.width as f64,
            bounds.origin.y as f64 + bounds.size.height as f64,
        ),
        (
            ResizeHandle::Bottom,
            bounds.origin.x as f64 + bounds.size.width as f64 / 2.0,
            bounds.origin.y as f64 + bounds.size.height as f64,
        ),
        (
            ResizeHandle::BottomLeft,
            bounds.origin.x as f64,
            bounds.origin.y as f64 + bounds.size.height as f64,
        ),
        (
            ResizeHandle::Left,
            bounds.origin.x as f64,
            bounds.origin.y as f64 + bounds.size.height as f64 / 2.0,
        ),
    ];

    for (handle, hx, hy) in handles.iter() {
        if (point.x - hx).abs() <= half_size && (point.y - hy).abs() <= half_size {
            return Some(*handle);
        }
    }

    None
}

/// Get the new bounds after a resize operation
pub fn calculate_resize_bounds(
    original_bounds: &Rect,
    handle: ResizeHandle,
    delta_x: f64,
    delta_y: f64,
) -> Rect {
    let mut new_bounds = *original_bounds;
    let delta_x_f32 = delta_x as f32;
    let delta_y_f32 = delta_y as f32;

    match handle {
        ResizeHandle::TopLeft => {
            new_bounds.origin.x += delta_x_f32;
            new_bounds.origin.y += delta_y_f32;
            new_bounds.size.width -= delta_x_f32;
            new_bounds.size.height -= delta_y_f32;
        }
        ResizeHandle::Top => {
            new_bounds.origin.y += delta_y_f32;
            new_bounds.size.height -= delta_y_f32;
        }
        ResizeHandle::TopRight => {
            new_bounds.origin.y += delta_y_f32;
            new_bounds.size.width += delta_x_f32;
            new_bounds.size.height -= delta_y_f32;
        }
        ResizeHandle::Right => {
            new_bounds.size.width += delta_x_f32;
        }
        ResizeHandle::BottomRight => {
            new_bounds.size.width += delta_x_f32;
            new_bounds.size.height += delta_y_f32;
        }
        ResizeHandle::Bottom => {
            new_bounds.size.height += delta_y_f32;
        }
        ResizeHandle::BottomLeft => {
            new_bounds.origin.x += delta_x_f32;
            new_bounds.size.width -= delta_x_f32;
            new_bounds.size.height += delta_y_f32;
        }
        ResizeHandle::Left => {
            new_bounds.origin.x += delta_x_f32;
            new_bounds.size.width -= delta_x_f32;
        }
    }

    // Ensure minimum size
    new_bounds.size.width = new_bounds.size.width.max(10.0);
    new_bounds.size.height = new_bounds.size.height.max(10.0);

    new_bounds
}

/// Mouse event handler state
pub struct MouseEventHandler {
    pub interaction: Rc<RefCell<MouseInteraction>>,
    pub last_pos: Rc<RefCell<Option<CanvasMousePos>>>,
}

impl MouseEventHandler {
    pub fn new() -> Self {
        Self {
            interaction: Rc::new(RefCell::new(MouseInteraction::Idle)),
            last_pos: Rc::new(RefCell::new(None)),
        }
    }

    pub fn start_drag(&self, object_id: uuid::Uuid, pos: CanvasMousePos) {
        *self.interaction.borrow_mut() = MouseInteraction::Dragging {
            object_id,
            start_pos: pos.to_point(),
            offset_x: 0.0,
            offset_y: 0.0,
        };
        *self.last_pos.borrow_mut() = Some(pos);
    }

    pub fn start_resize(
        &self,
        object_id: uuid::Uuid,
        handle: ResizeHandle,
        pos: CanvasMousePos,
        original_bounds: Rect,
    ) {
        *self.interaction.borrow_mut() = MouseInteraction::Resizing {
            object_id,
            handle,
            start_pos: pos.to_point(),
            original_bounds,
        };
        *self.last_pos.borrow_mut() = Some(pos);
    }

    pub fn start_selection_drag(&self, pos: CanvasMousePos) {
        *self.interaction.borrow_mut() = MouseInteraction::SelectionDrag {
            start_pos: pos.to_point(),
            current_pos: pos.to_point(),
        };
        *self.last_pos.borrow_mut() = Some(pos);
    }

    pub fn update_position(&self, pos: CanvasMousePos) {
        *self.last_pos.borrow_mut() = Some(pos);

        match *self.interaction.borrow_mut() {
            MouseInteraction::Dragging {
                object_id: _,
                start_pos,
                ref mut offset_x,
                ref mut offset_y,
            } => {
                *offset_x = pos.x - start_pos.x as f64;
                *offset_y = pos.y - start_pos.y as f64;
            }
            MouseInteraction::SelectionDrag {
                start_pos: _,
                ref mut current_pos,
            } => {
                *current_pos = pos.to_point();
            }
            _ => {}
        }
    }

    pub fn end_interaction(&self) {
        *self.interaction.borrow_mut() = MouseInteraction::Idle;
        *self.last_pos.borrow_mut() = None;
    }

    pub fn is_interacting(&self) -> bool {
        *self.interaction.borrow() != MouseInteraction::Idle
    }
}

impl Default for MouseEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use testruct_core::layout::Size;

    #[test]
    fn test_widget_to_canvas_conversion() {
        let pos = widget_to_canvas(
            100.0, 150.0, // widget coords
            1.0,   // zoom
            0.0, 0.0,  // pan
            20.0, // ruler size
        );
        assert_eq!(pos.x, 80.0);
        assert_eq!(pos.y, 130.0);
    }

    #[test]
    fn test_widget_to_canvas_with_zoom() {
        let pos = widget_to_canvas(
            100.0, 100.0, // widget coords
            2.0,   // 2x zoom
            0.0, 0.0,  // pan
            20.0, // ruler size
        );
        assert_eq!(pos.x, 40.0);
        assert_eq!(pos.y, 40.0);
    }

    #[test]
    fn test_point_in_bounds() {
        let bounds = Rect::new(Point::new(10.0, 10.0), Size::new(100.0, 50.0));

        assert!(point_in_bounds(CanvasMousePos::new(50.0, 30.0), &bounds));
        assert!(!point_in_bounds(CanvasMousePos::new(5.0, 30.0), &bounds));
        assert!(!point_in_bounds(CanvasMousePos::new(150.0, 30.0), &bounds));
    }

    #[test]
    fn test_resize_bounds_bottom_right() {
        let bounds = Rect::new(Point::new(0.0, 0.0), Size::new(100.0, 100.0));

        let new_bounds = calculate_resize_bounds(&bounds, ResizeHandle::BottomRight, 50.0, 25.0);
        assert_eq!(new_bounds.size.width, 150.0);
        assert_eq!(new_bounds.size.height, 125.0);
    }

    #[test]
    fn test_mouse_event_handler_drag() {
        let handler = MouseEventHandler::new();
        let object_id = uuid::Uuid::new_v4();

        handler.start_drag(object_id, CanvasMousePos::new(0.0, 0.0));
        handler.update_position(CanvasMousePos::new(50.0, 30.0));

        assert!(handler.is_interacting());
    }
}
