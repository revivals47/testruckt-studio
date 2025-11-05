use gtk4::{prelude::*, DrawingArea, EventControllerMotion, GestureClick, GestureDrag};
use gtk4::gdk;
use crate::canvas::{CanvasRenderState, tools::{ToolMode, ShapeFactory}};
use crate::app::AppState;
use testruct_core::layout::{Point, Rect, Size};

/// Initialize pointer events for canvas
pub fn wire_pointer_events(
    drawing_area: &DrawingArea,
    render_state: &CanvasRenderState,
    _app_state: &AppState,
) {
    drawing_area.set_focusable(true);

    // Keyboard event controller
    drawing_area.add_controller(gtk4::EventControllerKey::new());

    // Motion controller for mouse movement
    let motion = EventControllerMotion::new();
    let render_state_motion = render_state.clone();
    motion.connect_motion(move |_controller, x, y| {
        let _state = render_state_motion.clone();
        tracing::debug!("pointer moved: x={}, y={}", x, y);
        // TODO: Update cursor based on hover state (resize handles, etc)
    });
    drawing_area.add_controller(motion);

    // Click gesture for object selection
    let click_gesture = GestureClick::new();
    click_gesture.set_button(gdk::BUTTON_PRIMARY);
    let render_state_click = render_state.clone();
    click_gesture.connect_pressed(move |_gesture, _n_press, x, y| {
        let state = render_state_click.clone();
        let tool_state = state.tool_state.borrow();
        let current_tool = tool_state.current_tool;
        drop(tool_state);

        tracing::debug!("canvas click at ({}, {}), tool: {:?}", x, y, current_tool);

        if current_tool == ToolMode::Select {
            // TODO: Perform hit detection and update selection
            tracing::debug!("Select tool: performing hit detection");
        }
    });
    drawing_area.add_controller(click_gesture);

    // Drag gesture for shape creation and object movement
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(gdk::BUTTON_PRIMARY);
    let render_state_drag = render_state.clone();
    let drawing_area_drag = drawing_area.clone();

    drag_gesture.connect_drag_begin(move |_gesture, x, y| {
        let state = render_state_drag.clone();
        let tool_state = state.tool_state.borrow();
        let current_tool = tool_state.current_tool;
        drop(tool_state);

        tracing::debug!("drag start at ({}, {}), tool: {:?}", x, y, current_tool);

        // Store drag start position
        let mut tool_state = state.tool_state.borrow_mut();
        tool_state.drag_start = Some((x, y));
    });

    let render_state_update = render_state.clone();
    drag_gesture.connect_drag_update(move |_gesture, offset_x, offset_y| {
        let state = render_state_update.clone();
        let tool_state = state.tool_state.borrow();
        if let Some((start_x, start_y)) = tool_state.drag_start {
            let current_x = start_x + offset_x;
            let current_y = start_y + offset_y;

            tracing::debug!(
                "drag update: from ({}, {}) to ({}, {})",
                start_x, start_y, current_x, current_y
            );

            // Update drag box for preview rendering
            let (x1, y1, x2, y2) = (start_x, start_y, current_x, current_y);
            let min_x = x1.min(x2);
            let min_y = y1.min(y2);
            let max_x = x1.max(x2);
            let max_y = y1.max(y2);

            let drag_rect = Rect {
                origin: Point {
                    x: min_x as f32,
                    y: min_y as f32,
                },
                size: Size {
                    width: (max_x - min_x) as f32,
                    height: (max_y - min_y) as f32,
                },
            };

            *state.drag_box.borrow_mut() = Some(drag_rect);
        }

        drawing_area_drag.queue_draw();
    });

    let render_state_end = render_state.clone();
    let drawing_area_end = drawing_area.clone();
    drag_gesture.connect_drag_end(move |_gesture, offset_x, offset_y| {
        let state = render_state_end.clone();
        let tool_state = state.tool_state.borrow();

        if let Some((start_x, start_y)) = tool_state.drag_start {
            let current_x = start_x + offset_x;
            let current_y = start_y + offset_y;
            let current_tool = tool_state.current_tool;

            tracing::debug!(
                "drag end: shape creation/move from ({}, {}) to ({}, {})",
                start_x, start_y, current_x, current_y
            );

            // Handle shape creation based on tool
            if current_tool != ToolMode::Select && (offset_x.abs() > 5.0 || offset_y.abs() > 5.0) {
                let _element = match current_tool {
                    ToolMode::Rectangle => ShapeFactory::create_rectangle(
                        start_x.min(current_x),
                        start_y.min(current_y),
                        (start_x - current_x).abs(),
                        (start_y - current_y).abs(),
                    ),
                    ToolMode::Circle => ShapeFactory::create_circle(
                        start_x.min(current_x),
                        start_y.min(current_y),
                        (start_x - current_x).abs(),
                        (start_y - current_y).abs(),
                    ),
                    ToolMode::Text => ShapeFactory::create_text(
                        start_x,
                        start_y,
                        (start_x - current_x).abs(),
                        (start_y - current_y).abs(),
                        "New Text".to_string(),
                    ),
                    _ => return,
                };
                // TODO: Add element to document and push to undo stack
            }
        }

        // Clear drag state
        drop(tool_state);
        let mut tool_state = state.tool_state.borrow_mut();
        tool_state.drag_start = None;
        *state.drag_box.borrow_mut() = None;

        drawing_area_end.queue_draw();
    });

    drawing_area.add_controller(drag_gesture);
}
