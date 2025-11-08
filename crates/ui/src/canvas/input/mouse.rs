use crate::app::AppState;
use crate::canvas::mouse::{test_resize_handle, CanvasMousePos, ResizeHandle};
use crate::canvas::CanvasRenderState;
use gtk4::prelude::*;
use gtk4::{DrawingArea, EventControllerMotion};
use gtk4::gdk;

/// Setup mouse motion tracking for cursor changes
pub fn setup_mouse_tracking(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    app_state: &AppState,
) {
    let motion = EventControllerMotion::new();
    let render_state_motion = render_state.clone();
    let drawing_area_cursor = drawing_area.clone();
    let app_state_motion = app_state.clone();

    motion.connect_motion(move |_controller, x, y| {
        let state = render_state_motion.clone();
        tracing::debug!("pointer moved: x={}, y={}", x, y);

        // Get canvas position
        let config = state.config.borrow();
        let ruler_config = state.ruler_config.borrow();
        let screen_x = x - (ruler_config.size + config.pan_x);
        let screen_y = y - (ruler_config.size + config.pan_y);
        let doc_x = screen_x / config.zoom;
        let doc_y = screen_y / config.zoom;
        drop(config);
        drop(ruler_config);

        let canvas_mouse_pos = CanvasMousePos { x: doc_x, y: doc_y };

        // Check if cursor is over a resize handle of selected objects
        let selected = state.selected_ids.borrow();
        let mut cursor_name = "default";

        if let Some(document) = app_state_motion.active_document() {
            if let Some(page) = document.pages.first() {
                for selected_id in selected.iter() {
                    // Find the selected element
                    for element in &page.elements {
                        let (elem_id, bounds) = match element {
                            testruct_core::document::DocumentElement::Shape(shape) => {
                                (shape.id, &shape.bounds)
                            }
                            testruct_core::document::DocumentElement::Text(text) => {
                                (text.id, &text.bounds)
                            }
                            testruct_core::document::DocumentElement::Image(image) => {
                                (image.id, &image.bounds)
                            }
                            testruct_core::document::DocumentElement::Frame(frame) => {
                                (frame.id, &frame.bounds)
                            }
                        };

                        if elem_id == *selected_id {
                            // Test for resize handle hit
                            if let Some(handle) = test_resize_handle(canvas_mouse_pos, bounds, 8.0) {
                                cursor_name = match handle {
                                    ResizeHandle::TopLeft | ResizeHandle::BottomRight => {
                                        "nwse-resize"
                                    }
                                    ResizeHandle::TopRight | ResizeHandle::BottomLeft => {
                                        "nesw-resize"
                                    }
                                    ResizeHandle::Top | ResizeHandle::Bottom => "ns-resize",
                                    ResizeHandle::Left | ResizeHandle::Right => "ew-resize",
                                };
                                break;
                            }
                        }
                    }
                    if cursor_name != "default" {
                        break;
                    }
                }
            }
        }

        // Set cursor
        let cursor = gdk::Cursor::from_name(cursor_name, None);
        drawing_area_cursor.set_cursor(cursor.as_ref());
    });
    drawing_area.add_controller(motion);
}
