use gtk4::{prelude::*, DrawingArea, EventControllerMotion, GestureClick, GestureDrag};
use gtk4::gdk;

pub fn wire_pointer_events(drawing_area: &DrawingArea) {
    drawing_area.set_focusable(true);

    // Keyboard event controller
    drawing_area.add_controller(gtk4::EventControllerKey::new());

    // Motion controller for mouse movement
    let motion = EventControllerMotion::new();
    motion.connect_motion(|_controller, x, y| {
        tracing::debug!("pointer moved: x={}, y={}", x, y);
        // TODO: Update cursor based on hover state (resize handles, etc)
    });
    drawing_area.add_controller(motion);

    // Click gesture for object selection
    let click_gesture = GestureClick::new();
    click_gesture.set_button(gdk::BUTTON_PRIMARY);
    click_gesture.connect_pressed(|_gesture, _n_press, x, y| {
        tracing::debug!("canvas click: x={}, y={}", x, y);
        // TODO: Handle object selection at (x, y)
    });
    drawing_area.add_controller(click_gesture);

    // Drag gesture for object movement
    let drag_gesture = GestureDrag::new();
    drag_gesture.set_button(gdk::BUTTON_PRIMARY);
    drag_gesture.connect_drag_begin(|_gesture, x, y| {
        tracing::debug!("drag start: x={}, y={}", x, y);
        // TODO: Determine what is being dragged (object, resize handle, etc)
    });
    drag_gesture.connect_drag_update(|_gesture, offset_x, offset_y| {
        tracing::debug!("drag update: offset_x={}, offset_y={}", offset_x, offset_y);
        // TODO: Update object position during drag
    });
    drag_gesture.connect_drag_end(|_gesture, offset_x, offset_y| {
        tracing::debug!("drag end: offset_x={}, offset_y={}", offset_x, offset_y);
        // TODO: Finalize object position and trigger rerender
    });
    drawing_area.add_controller(drag_gesture);
}
